<script lang="ts">
  import Icon from '../ui/Icon.svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import { tooltip } from '$lib/actions/tooltip';
  import { fade, scale } from 'svelte/transition';
  import { backOut, quintOut } from 'svelte/easing';
  import { toggleFavorite as toggleFavoriteAction } from '$lib/stores/media';

  let { 
    item, 
    viewMode = 'grid', 
    selected = false, 
    selectionMode = false,
    batchSelected = false,
    onclick, 
    onmenu,
    onrightclick,
    ondblclick
  } = $props<{
    item: {
      id: string;
      title: string;
      poster?: string | null;
      type: 'video' | 'audio' | 'mixed' | 'unknown';
      is_favorite?: boolean;
    };
    viewMode?: 'grid' | 'list' | 'detail';
    selected?: boolean;
    selectionMode?: boolean;
    batchSelected?: boolean;
    onclick?: (e: MouseEvent) => void;
    onmenu?: (e: MouseEvent) => void;
    onrightclick?: (e: MouseEvent) => void;
    ondblclick?: (e: MouseEvent) => void;
  }>();

  let imageError = $state(false);

  function toggleFavorite(e?: Event) {
    if (e) e.stopPropagation();
    toggleFavoriteAction(item.id);
  }

  let hasPoster = $derived(!!item.poster && !imageError);
  let resolvedPoster = $derived(resolveResource(item.poster));
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (e.ctrlKey) {
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Playing ${item.title}`, icon: 'playing' } }));
        ondblclick?.(e as any);
      } else {
        onclick?.(e as any);
      }
    } else if (e.key.toLowerCase() === 'f' && !e.ctrlKey && !e.metaKey) {
      e.preventDefault();
      toggleFavorite(e);
    }
  }
</script>

<div 
  class="media-card" 
  class:fallback={!hasPoster} 
  class:list-mode={viewMode === 'list'} 
  class:selected={selected}
  class:selection-mode={selectionMode}
  class:batch-selected={batchSelected}
  onclick={(e) => {
    e.stopPropagation();
    onclick?.(e);
  }}
  ondblclick={(e) => {
    e.stopPropagation();
    ondblclick?.(e);
  }}
  onkeydown={handleKeydown}
  oncontextmenu={(e) => {
    e.stopPropagation();
    onrightclick?.(e);
  }}
  role="button"
  tabindex="0"
>
  {#if selectionMode}
    <div class="choice-indicator">
      <Icon name="check" strokeWidth={3} size={14} />
    </div>
  {/if}
  <div class="poster-container">
    {#if hasPoster && !imageError}
      {#key resolvedPoster}
        <img 
          src={resolvedPoster} 
          alt={item.title} 
          class="poster-image" 
          onerror={() => (imageError = true)}
        />
      {/key}
    {:else}
      <div class="placeholder-logo">
        <img src="/flux2d.png" alt="Flux" />
      </div>
    {/if}
  </div>

  <div class="metadata">
    <span class="title" use:tooltip={{ content: item.title, placement: 'top' }}>{item.title}</span>
  </div>

  <!-- Favorite Button (Top Left) -->
  {#if !selectionMode}
    <button 
      class="favorite-btn" 
      class:is-favorited={item.is_favorite}
      aria-label="Toggle Favorite"
      use:tooltip={{ content: item.is_favorite ? 'Remove from Favorites' : 'Add to Favorites', placement: 'top' }}
      onclick={toggleFavorite}
    >
      <Icon name="star" fill={item.is_favorite ? "var(--secondary)" : "none"} strokeWidth={2.5} size={18} />
    </button>

    <button 
      class="menu-btn" 
      aria-label="Media Options"
      use:tooltip={{ content: 'Options', placement: 'top' }}
      onclick={(e) => {
        e.stopPropagation();
        onmenu?.(e);
      }}
    >
      <Icon name="more" strokeWidth={2.5} size={18} />
    </button>
  {/if}
</div>

<style>
  .media-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    width: 100%;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid transparent; /* Base state invisible border */
    position: relative; /* Context for children */
  }

  /* Selection Mode Dimming */
  .media-card.selection-mode:not(.batch-selected) {
    opacity: 0.4;
    filter: grayscale(0.2);
  }

  .media-card.selection-mode:not(.batch-selected):hover {
    opacity: 0.7;
    filter: grayscale(0);
  }

  /* Base Hover behavior: only for neutral items */
  .media-card:hover:not(.selected):not(.batch-selected) {
    background: var(--glass-bg-low);
    border-color: var(--glass-border-mid);
  }

  .media-card:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2px var(--secondary);
    border-radius: 12px;
  }

  /* Single Selection (Cyan): High Viz, but only outside Batch Mode */
  .media-card.selected:not(.selection-mode) {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
    border-width: 1px;
  }

  /* Batch Selection (Purple): Absolute Precedence */
  .media-card.batch-selected {
    background: rgba(138, 43, 226, 0.1) !important;
    border-color: var(--primary) !important;
    border-width: 2px !important;
    opacity: 1 !important;
    filter: grayscale(0) !important;
  }

  .choice-indicator {
    position: absolute;
    top: 12px;
    left: 12px;
    width: 22px;
    height: 22px;
    border-radius: 6px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    backdrop-filter: blur(4px);
  }

  .media-card.batch-selected .choice-indicator {
    background: var(--primary);
    border-color: var(--primary);
  }

  /* List mode specific indicator positioning */
  .media-card.list-mode .choice-indicator {
    top: 6px;
    left: 6px;
    width: 20px;
    height: 20px;
  }

  /* Favorite Button */
  .favorite-btn {
    position: absolute;
    top: 12px;
    left: 12px;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-main);
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s ease, transform 0.2s ease, color 0.15s ease;
    padding: 0;
    cursor: pointer;
    z-index: 20;
    border-radius: 50%;
  }

  .media-card:hover .favorite-btn {
    opacity: 0.7;
    pointer-events: auto;
  }

  .favorite-btn:hover {
    opacity: 1 !important;
    transform: scale(1.1);
    pointer-events: auto;
  }

  .favorite-btn.is-favorited {
    opacity: 0.6; /* Dimmed when not hovered */
    pointer-events: auto;
  }

  /* List Mode Overrides (Remove button to save space) */
  .media-card.list-mode .favorite-btn {
    display: none;
  }

  .menu-btn {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-main);
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.2s ease, background 0.15s ease, transform 0.2s ease;
    padding: 0;
    cursor: pointer;
    z-index: 20;
    border-radius: 50%;
  }

  .media-card:hover .menu-btn {
    opacity: 0.7;
    pointer-events: auto;
  }

  .menu-btn:hover {
    opacity: 1 !important;
    transform: scale(1.1);
  }

  .poster-container {
    aspect-ratio: 2 / 3;
    width: 100%;
    /* Solid matte background for transparent artwork */
    background: rgba(14, 14, 14, 0.85);
    border: 1px solid rgba(255, 255, 255, 0.18);
    border-radius: 8px;
    overflow: hidden;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .poster-container::before {
    content: '';
    position: absolute;
    inset: 0;
    background: 
      radial-gradient(circle at top left, rgba(0,0,0,0.8) 0%, transparent 22%),
      radial-gradient(circle at top right, rgba(0,0,0,0.8) 0%, transparent 22%);
    pointer-events: none;
    z-index: 3;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .media-card:hover .poster-container::before {
    opacity: 1;
  }

  .poster-container::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.06) 0%, rgba(255,255,255,0.02) 20%, transparent 60%);
    pointer-events: none;
    z-index: 2; /* Ensures it sits above both image and placeholder logo */
  }

  .poster-image {
    width: 100%;
    height: 100%;
    object-fit: cover; 
    transition: opacity 0.3s ease, transform 1.7s cubic-bezier(0.23, 1, 0.32, 1);
  }

  .media-card:hover .poster-image {
    transform: scale(1.1);
  }

  .media-card:hover .placeholder-logo {
    transform: scale(1.1);
  }

  .placeholder-logo {
    width: 40%;
    aspect-ratio: 1 / 1;
    opacity: 0.5;
    transition: transform 1.7s cubic-bezier(0.23, 1, 0.32, 1);
  }

  .placeholder-logo img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .metadata {
    padding: 0 4px;
    text-align: center;
  }

  .title {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-main);
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* List Mode Overrides (Refined for "Compact" Efficiency) */
  .media-card.list-mode {
    flex-direction: row;
    align-items: center;
    gap: 12px;
    padding: 6px 10px;
    background: transparent;
    border-radius: 8px;
    height: 64px;
    border: 1px solid transparent;
  }

  .media-card.list-mode:hover {
    background: var(--glass-bg-low);
    border-color: var(--glass-border-low);
  }

  .media-card.list-mode.selected {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .media-card.list-mode .poster-container {
    height: 52px;
    width: 34px; /* Scaled 2:3 iconic representation */
    min-width: 34px;
    border-radius: 4px;
    background: rgba(255, 255, 255, 0.03);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .media-card.list-mode .metadata {
    text-align: left;
    flex-grow: 1;
    overflow: hidden;
  }

  .media-card.list-mode .title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-main);
  }

  /* List Mode Menu Button - perfectly centered vertically */
  .media-card.list-mode .menu-btn {
    position: static;
    opacity: 0;
    width: 28px;
    height: 28px;
    flex-shrink: 0;
  }

  .media-card.list-mode:hover .menu-btn {
    opacity: 0.6;
    pointer-events: auto;
  }

  .media-card.list-mode .menu-btn:hover {
    opacity: 1;
    background: var(--glass-bg-mid);
  }
</style>
