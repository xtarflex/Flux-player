<script lang="ts">

  let { item, viewMode = 'grid', selected = false, onclick, onmenu } = $props<{
    item: {
      id: string;
      title: string;
      poster?: string;
      type: 'video' | 'audio' | 'unknown';
    };
    viewMode?: 'grid' | 'list' | 'detail';
    selected?: boolean;
    onclick?: () => void;
    onmenu?: (e: MouseEvent) => void;
  }>();

  let hasPoster = $derived(!!item.poster);
</script>

<div 
  class="media-card" 
  class:fallback={!hasPoster} 
  class:list-mode={viewMode === 'list'} 
  class:selected={selected}
  onclick={onclick}
  onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && onclick?.()}
  role="button"
  tabindex="0"
>
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
</div>

<style>
  .media-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    width: 100%;
    border-radius: 12px;
    padding: 6px;
    border: 1px solid transparent; /* Base state invisible border */
    position: relative; /* Context for children */
  }

  .media-card:hover {
    background: var(--glass-bg-low);
    border-color: var(--glass-border-mid);
  }

  .media-card:active, .media-card.selected {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
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
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.6);
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
