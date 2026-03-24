<script lang="ts">

  let { 
    item, 
    viewMode = 'grid', 
    selected = false, 
    selectionMode = false,
    batchSelected = false,
    onclick, 
    onmenu,
    onrightclick
  } = $props<{
    item: {
      id: string;
      title: string;
      poster?: string;
      type: 'video' | 'audio' | 'unknown';
    };
    viewMode?: 'grid' | 'list' | 'detail';
    selected?: boolean;
    selectionMode?: boolean;
    batchSelected?: boolean;
    onclick?: (e: MouseEvent) => void;
    onmenu?: (e: MouseEvent) => void;
    onrightclick?: (e: MouseEvent) => void;
  }>();

  let hasPoster = $derived(!!item.poster);
</script>

<div 
  class="media-card" 
  class:fallback={!hasPoster} 
  class:list-mode={viewMode === 'list'} 
  class:selected={selected}
  class:selection-mode={selectionMode}
  class:batch-selected={batchSelected}
  onclick={(e) => onclick?.(e)}
  onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && onclick?.(e as any)}
  oncontextmenu={(e) => onrightclick?.(e)}
  role="button"
  tabindex="0"
>
  {#if selectionMode}
    <div class="choice-indicator">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="20 6 9 17 4 12"></polyline>
      </svg>
    </div>
  {/if}
  <div class="poster-container">
    {#if hasPoster}
      <img src={item.poster} alt={item.title} class="poster-image" />
    {:else}
      <div class="placeholder-logo">
        <img src="/flux2d.png" alt="Flux" />
      </div>
    {/if}
  </div>

  <div class="metadata">
    <span class="title" title={item.title}>{item.title}</span>
  </div>

  <!-- Context Menu Button: Transparent, reveals on hover -->
  {#if !selectionMode}
    <button 
      class="menu-btn" 
      aria-label="Media Options"
      title="Options"
      onclick={(e) => {
        e.stopPropagation();
        onmenu?.(e);
      }}
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="1.5"></circle>
        <circle cx="12" cy="5" r="1.5"></circle>
        <circle cx="12" cy="19" r="1.5"></circle>
      </svg>
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

  .choice-indicator svg {
    width: 14px;
    height: 14px;
    color: var(--bg-base);
    opacity: 0;
    transform: scale(0.5);
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .media-card.batch-selected .choice-indicator svg {
    opacity: 1;
    transform: scale(1);
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

  .menu-btn svg {
    width: 18px;
    height: 18px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.5));
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

  /* List Mode Overrides */
  .media-card.list-mode {
    flex-direction: row;
    align-items: center;
    gap: 16px;
    padding: 12px;
    background: var(--glass-bg-low);
    border-radius: 12px;
    height: 100%;
  }

  .media-card.list-mode:hover {
    background: var(--glass-bg-mid);
    border-color: var(--glass-border-high);
  }

  .media-card.list-mode:active, .media-card.list-mode.selected {
    border-color: var(--secondary);
  }

  .media-card.list-mode .poster-container {
    height: 72px;
    width: 48px; /* maintains 2:3 ratio */
    min-width: 48px;
    border-radius: 8px;
  }

  .media-card.list-mode .metadata {
    text-align: left;
    flex-grow: 1;
  }

  /* List Mode Menu Button */
  .media-card.list-mode .menu-btn {
    position: absolute;
    top: 12px;
    right: 12px;
    opacity: 0;
  }

  .media-card.list-mode:hover .menu-btn {
    opacity: 0.8;
    pointer-events: auto;
  }
</style>
