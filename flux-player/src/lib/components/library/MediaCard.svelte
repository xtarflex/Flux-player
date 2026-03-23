<script lang="ts">

  let { item, viewMode = 'grid', selected = false } = $props<{
    item: {
      id: string;
      title: string;
      poster?: string;
      type: 'video' | 'audio' | 'unknown';
    };
    viewMode?: 'grid' | 'list' | 'detail';
    selected?: boolean;
  }>();

  let hasPoster = $derived(!!item.poster);
</script>

<div class="media-card" class:fallback={!hasPoster} class:list-mode={viewMode === 'list'} class:selected={selected} >
  <div class="poster-container">
    {#if hasPoster}
      <img src={item.poster} alt={item.title} class="poster-image" />
    {:else}
      <div class="placeholder-logo">
        <img src="/flux2d.png" alt="Flux Fallback Logo" />
      </div>
    {/if}
  </div>
  <div class="metadata">
    <span class="title" title={item.title}>{item.title}</span>
  </div>
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
  }

  .media-card:hover {
    background: var(--glass-bg-low);
    border-color: var(--glass-border-mid);
  }

  .media-card:active, .media-card.selected {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
  }

  .poster-container {
    aspect-ratio: 2 / 3;
    width: 100%;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    overflow: hidden;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.2s ease;
  }

  .poster-image {
    width: 100%;
    height: 100%;
    object-fit: contain; /* allows transparent posters to sit inside gracefully */
    transition: opacity 0.3s ease;
  }

  .fallback .poster-container {
    background: linear-gradient(135deg, rgba(26,26,31,0.8) 0%, rgba(10,10,12,0.9) 100%);
  }

  .placeholder-logo {
    width: 40%;
    aspect-ratio: 1 / 1;
    opacity: 0.5;
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
</style>
