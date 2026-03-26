<script lang="ts">
  import { mediaItems, selectedMediaId } from '$lib/stores/media';
  import DetailPanel from './DetailPanel.svelte';

  export let isDetailPanelOpen = false;

  function selectItem(id: string) {
    if ($selectedMediaId === id) {
      isDetailPanelOpen = !isDetailPanelOpen; // Toggle if clicking the same item
    } else {
      $selectedMediaId = id;
      isDetailPanelOpen = true; // Open panel when a new item is selected
    }
  }

  function formatDuration(duration: string) {
    return duration; // Assuming format is already correct from the store
  }
</script>

<div class="library-container" class:with-detail={isDetailPanelOpen}>
  <div class="header">
    <div class="title-group">
      <h1 class="heading">YOUR LIBRARY</h1>
      <span class="count-badge">{$mediaItems.length} Items</span>
    </div>

    <div class="actions">
      <button class="action-btn">
        <span class="icon">↺</span> Refresh
      </button>
      <div class="dropdown">
        All Media <span class="caret">▼</span>
      </div>
      <div class="dropdown">
        Recently Added <span class="caret">▼</span>
      </div>
      <div class="view-toggles">
        <button class="icon-btn">🔍</button>
        <button class="icon-btn">▦</button>
        <button class="icon-btn">≡</button>
        <button class="icon-btn active" on:click={() => isDetailPanelOpen = !isDetailPanelOpen}>
           ☐
        </button>
      </div>
    </div>
  </div>

  <div class="media-list">
    {#each $mediaItems as item}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        class="media-row"
        class:selected={$selectedMediaId === item.id && isDetailPanelOpen}
        on:click={() => selectItem(item.id)}
      >
        <div class="poster-thumb" style="background-image: url('{item.poster}')"></div>
        <div class="info">
          <span class="title">{item.title}</span>
        </div>
        <div class="more-opts">
          <button class="dots">⋮</button>
        </div>
      </div>
    {/each}
  </div>
</div>

<DetailPanel isOpen={isDetailPanelOpen} />

<style>
  .library-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 0 40px;
    transition: padding-right 0.4s cubic-bezier(0.23, 1, 0.320, 1);
  }

  .library-container.with-detail {
    padding-right: calc(35% + 40px); /* Adjust content width when panel is open */
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 30px 0 20px 0;
  }

  .title-group {
    display: flex;
    align-items: baseline;
    gap: 16px;
  }

  .heading {
    font-size: 28px;
    color: var(--text-main);
  }

  .count-badge {
    background: rgba(255, 255, 255, 0.1);
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 12px;
    color: var(--text-muted);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .action-btn {
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-main);
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-body);
    font-size: 14px;
  }

  .action-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .dropdown {
    background: transparent;
    border: 1px solid var(--border-light);
    color: var(--text-main);
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font-body);
    font-size: 14px;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .caret {
    font-size: 10px;
  }

  .view-toggles {
    display: flex;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 6px;
    padding: 4px;
    gap: 4px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    width: 32px;
    height: 32px;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    color: var(--text-main);
    background: rgba(255, 255, 255, 0.05);
  }

  .icon-btn.active {
    background: var(--primary);
    color: var(--text-main);
  }

  .media-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    overflow-y: auto;
    padding-bottom: 40px;
  }

  .media-row {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background: rgba(255, 255, 255, 0.02);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid transparent;
  }

  .media-row:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .media-row.selected {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
    box-shadow: 0 0 0 1px var(--secondary); /* To make it slightly thicker without affecting layout */
  }

  .poster-thumb {
    width: 40px;
    height: 40px;
    border-radius: 6px;
    background-size: cover;
    background-position: center;
    flex-shrink: 0;
    margin-right: 16px;
    background-color: var(--bg-surface);
  }

  .info {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
  }

  .title {
    font-family: var(--font-body);
    font-weight: 500;
    font-size: 15px;
    color: var(--text-main);
  }

  .more-opts {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .dots {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 20px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
  }

  .dots:hover {
    color: var(--text-main);
    background: rgba(255, 255, 255, 0.1);
  }

  /* Custom Scrollbar for list */
  .media-list::-webkit-scrollbar {
    width: 6px;
  }
  .media-list::-webkit-scrollbar-track {
    background: transparent;
  }
  .media-list::-webkit-scrollbar-thumb {
    background: var(--border-light);
    border-radius: 4px;
  }
  .media-list::-webkit-scrollbar-thumb:hover {
    background: var(--primary);
  }
</style>
