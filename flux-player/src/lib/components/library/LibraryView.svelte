<script lang="ts">
  import MediaCard from './MediaCard.svelte';

  // Mock Data
  let libraryItems = $state(
    Array.from({ length: 24 }).map((_, i) => {
      let title = `Placeholder File ${i + 1}`;
      let poster: string | undefined = undefined;
      let type: 'video' | 'audio' | 'unknown' = 'unknown';

      if (i === 0) {
        title = "dropped_video.mp4";
        type = "video";
      } else if (i === 1) {
        title = "Blue Lock-S2E1-480P";
        type = "video";
      } else if (i === 2) {
        title = "BLUE LOCK THE MOVIE ...";
        poster = "https://image.tmdb.org/t/p/w500/aeu1aheebQjZlV4uEkiWkLwI42s.jpg";
        type = "video";
      } else if (i === 3) {
        title = "Guardians of the galaxy";
        poster = "https://image.tmdb.org/t/p/w500/r7vmZjiyZw9rpJMQJdXpjgiCOd9.jpg";
        type = "video";
      } else if (i >= 4) {
        title = "Audio Track || TrendyBeatz.com";
        poster = "https://image.tmdb.org/t/p/w500/b0nkiZ7LpQd4aXvB4tH00QibZMB.jpg";
        type = "audio";
      }

      return { id: `item-${i}`, title, poster, type };
    })
  );

  type ViewMode = 'grid' | 'list' | 'detail';
  let viewMode = $state<ViewMode>('grid');

  const gridZoomSteps = [130, 220, 320];
  const listZoomSteps = [2, 1]; // Index 0 (Zoom out) = 2 columns, Index 1 (Zoom in) = 1 column

  let gridZoomIndex = $state(1); // Default to 220px
  let listZoomIndex = $state(0); // Default to 2 columns 

  function zoomIn() {
    if (viewMode === 'grid') {
      gridZoomIndex = Math.min(gridZoomSteps.length - 1, gridZoomIndex + 1); // Larger cards
    } else if (viewMode === 'list') {
      listZoomIndex = Math.min(listZoomSteps.length - 1, listZoomIndex + 1); // Fewer columns
    }
  }

  function zoomOut() {
    if (viewMode === 'grid') {
      gridZoomIndex = Math.max(0, gridZoomIndex - 1); // Smaller cards
    } else if (viewMode === 'list') {
      listZoomIndex = Math.max(0, listZoomIndex - 1); // More columns
    }
  }

  let disableZoom = $derived(viewMode === 'detail');
  
  let gridStyle = $derived.by(() => {
    let transition = 'transition: grid-template-columns 0.4s ease;';
    if (viewMode === 'grid') {
      return `grid-template-columns: repeat(auto-fill, minmax(${gridZoomSteps[gridZoomIndex]}px, 1fr)); ${transition}`;
    } else if (viewMode === 'list') {
      return `grid-template-columns: repeat(${listZoomSteps[listZoomIndex]}, minmax(0, 1fr)); ${transition}`;
    } else {
      return `grid-template-columns: 1fr; list-style-type: none; ${transition}`;
    }
  });
</script>

<div class="discovery-hub">
  <div class="hub-header">
    <div class="hub-title-section">
      <h1 class="hub-title">Your <span class="uppercase">Library</span></h1>
      <span class="item-count">{libraryItems.length} Items</span>
    </div>

    <div class="action-bar">
      <!-- Search Input -->
      <div class="search-container">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        <input type="text" class="search-input" placeholder="Search library..." />
      </div>

      <button class="action-btn select-dropdown">
        <span>All Media</span>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
      </button>

      <button class="action-btn select-dropdown">
        <span>Recently Added</span>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M6 9l6 6 6-6"/></svg>
      </button>

      <!-- Zoom Controls -->
      <div class="zoom-controls" style:opacity={disableZoom ? 0.3 : 1} style:pointer-events={disableZoom ? 'none' : 'auto'}>
        <button class="icon-btn" onclick={zoomOut} aria-label="Zoom Out" title="Smaller Grid" disabled={disableZoom}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
        </button>
        <button class="icon-btn" onclick={zoomIn} aria-label="Zoom In" title="Larger Grid" disabled={disableZoom}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="11" y1="8" x2="11" y2="14"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
        </button>
      </div>

      <!-- View Toggles -->
      <div class="view-toggles">
        <button class="toggle-btn" class:active={viewMode === 'grid'} onclick={() => viewMode = 'grid'} aria-label="Grid View">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7"></rect>
            <rect x="14" y="3" width="7" height="7"></rect>
            <rect x="14" y="14" width="7" height="7"></rect>
            <rect x="3" y="14" width="7" height="7"></rect>
          </svg>
        </button>
        <button class="toggle-btn" class:active={viewMode === 'list'} onclick={() => viewMode = 'list'} aria-label="List View">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <line x1="8" y1="6" x2="21" y2="6"></line>
            <line x1="8" y1="12" x2="21" y2="12"></line>
            <line x1="8" y1="18" x2="21" y2="18"></line>
            <line x1="3" y1="6" x2="3.01" y2="6"></line>
            <line x1="3" y1="12" x2="3.01" y2="12"></line>
            <line x1="3" y1="18" x2="3.01" y2="18"></line>
          </svg>
        </button>
        <button class="toggle-btn" class:active={viewMode === 'detail'} onclick={() => viewMode = 'detail'} aria-label="Detail View">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
            <line x1="9" y1="3" x2="9" y2="21"></line>
          </svg>
        </button>
      </div>
    </div>
  </div>

  <div class="media-grid" style={gridStyle}>
    {#each libraryItems as item}
      <MediaCard {item} {viewMode} />
    {/each}
  </div>
</div>

<style>
  .discovery-hub {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    padding: 32px;
    overflow-y: auto;
  }

  /* Header Section */
  .hub-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 32px;
  }

  .hub-title-section {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .hub-title {
    font-family: var(--font-heading);
    font-size: 32px;
    font-weight: 700;
    margin: 0;
    letter-spacing: -0.5px;
  }

  .uppercase {
    text-transform: uppercase;
  }

  .item-count {
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 13px;
    color: var(--text-muted);
    font-weight: 500;
  }

  /* Action Bar */
  .action-bar {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .search-container {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    width: 16px;
    height: 16px;
    color: var(--text-muted);
  }

  .search-input {
    background: var(--bg-surface);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 8px 16px 8px 36px;
    border-radius: 8px;
    font-family: var(--font-body);
    font-size: 14px;
    width: 200px;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--secondary);
    box-shadow: 0 0 0 2px rgba(0, 255, 255, 0.2);
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    color: var(--text-main);
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .action-btn:hover {
    background: var(--glass-bg-high);
    border-color: var(--glass-border-mid);
  }

  .action-btn svg {
    width: 16px;
    height: 16px;
  }

  .select-dropdown svg {
    margin-left: 4px;
    opacity: 0.6;
  }

  .zoom-controls {
    display: flex;
    align-items: center;
    gap: 4px;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    padding: 4px;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    color: var(--text-main);
    background: var(--glass-bg-low);
  }

  .icon-btn svg {
    width: 16px;
    height: 16px;
  }

  .view-toggles {
    display: flex;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    padding: 4px;
  }

  .toggle-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    padding: 6px;
    border-radius: 6px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .toggle-btn svg {
    width: 18px;
    height: 18px;
  }

  .toggle-btn:hover:not(.active) {
    color: var(--text-main);
  }

  .toggle-btn.active {
    background: var(--primary);
    color: white;
  }

  /* Grid Layout */
  .media-grid {
    display: grid;
    gap: 24px 16px;
    padding-bottom: 32px;
  }

  /* Responsive Adjustments */
  @media (max-width: 1200px) {
    .search-input { width: 150px; }
  }

  @media (max-width: 1000px) {
    .action-btn span { display: none; } /* Hide text, keep icons */
    .hub-header { flex-direction: column; align-items: flex-start; gap: 16px; }
    .action-bar { width: 100%; justify-content: space-between; }
    .search-input { width: 100%; }
    .search-container { flex: 1; }
  }
</style>
