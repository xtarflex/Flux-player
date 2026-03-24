<script lang="ts">
  import MediaCard from './MediaCard.svelte';
  import Dropdown from '../ui/Dropdown.svelte';
  import DetailPanel from '../DetailPanel.svelte';
  import { mediaItems, selectedMediaId } from '$lib/stores/media';

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

  let mediaFilter = $state('All Media');
  let sortOption = $state('Recently Added');

  const mediaOptions = ['All Media', 'Movies', 'Music', 'Series'];
  const sortOptions = ['Recently Added', 'A-Z', 'Z-A', 'Release Date'];

  let searchText = $state('');

  let filteredItems = $derived.by(() => {
    let result = [...$mediaItems];

    // Filter by type
    if (mediaFilter !== 'All Media') {
      const typeMap: Record<string, string> = {
        'Movies': 'video',
        'Series': 'video', // Series would ideally have its own type but for now maps to video
        'Music': 'audio'
      };
      result = result.filter(item => item.type === typeMap[mediaFilter]);
    }

    // Filter by text
    if (searchText.trim()) {
      const query = searchText.toLowerCase();
      result = result.filter(item => 
        item.title.toLowerCase().includes(query)
      );
    }

    // Sort
    result.sort((a, b) => {
      switch (sortOption) {
        case 'A-Z': return a.title.localeCompare(b.title);
        case 'Z-A': return b.title.localeCompare(a.title);
        case 'Recently Added': return b.id.localeCompare(a.id); 
        default: return 0;
      }
    });

    return result;
  });

  function selectItem(id: string) {
    if ($selectedMediaId === id && viewMode === 'detail') {
      // If we are already in detail mode and click the same item, it's already selected
      // but maybe switch mode if it was grid? No, user said "detail mode" is a permanent state
    } else {
      $selectedMediaId = id;
    }
  }
</script>

<div class="discovery-hub">
  <div class="hub-header">
    <div class="hub-title-section">
      <h1 class="hub-title">Your <span class="uppercase">Library</span></h1>
      <span class="item-count">{filteredItems.length} Items</span>
    </div>

    <div class="action-bar">
      <!-- Search Input -->
      <div class="search-container">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8"></circle>
          <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
        <input 
          type="text" 
          class="search-input" 
          placeholder="Search library..." 
          bind:value={searchText}
        />
      </div>

      <Dropdown options={mediaOptions} bind:value={mediaFilter} showCheckmark={false} />
      <Dropdown options={sortOptions} bind:value={sortOption} showCheckmark={false} />

      <!-- Zoom Controls -->
      <div class="zoom-controls" style:opacity={disableZoom ? 0.3 : 1} style:pointer-events={disableZoom ? 'none' : 'auto'}>
        <button class="icon-btn" onclick={zoomOut} aria-label="Zoom Out" title="Smaller Grid" disabled={disableZoom}>
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="8" y1="11" x2="14" y2="11"></line></svg>
        </button>
        <div class="v-divider"></div>
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

  <div class="content-area">
    <div class="main-view">
      <div class="media-grid" style={gridStyle}>
        {#if viewMode !== 'detail'}
          {#each filteredItems as item (item.id)}
            <MediaCard 
              {item} 
              {viewMode} 
              selected={$selectedMediaId === item.id}
              onclick={() => selectItem(item.id)}
            />
          {/each}
        {:else}
          <!-- In detail mode, left pane always shows as list -->
          {#each filteredItems as item (item.id)}
            <MediaCard 
              {item} 
              viewMode="list" 
              selected={$selectedMediaId === item.id}
              onclick={() => selectItem(item.id)}
            />
          {/each}
        {/if}
      </div>
    </div>

    {#if viewMode === 'detail'}
      <div class="blank-detail-panel">
        <DetailPanel />
      </div>
    {/if}
  </div>
</div>

<style>
  .discovery-hub {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    overflow: hidden;
  }

  .content-area {
    flex: 1;
    display: flex;
    flex-direction: row;
    overflow: hidden;
  }

  .main-view {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 32px 32px; /* Top padding moved to header margin */
    overflow-y: auto;
  }

  .content-area .main-view {
    flex: 1;
    min-width: 0; /* Important for flex shrinking */
  }

  /* Detail Panel Card Layout (Island Style) */
  .blank-detail-panel {
    flex: 0 0 480px;
    height: 100%;
    padding: 0 24px 24px 0;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    overflow: hidden; /* Scrollbar will be inside DetailPanel */
  }

  @media (max-width: 1200px) {
    .blank-detail-panel {
      flex: 0 0 420px;
      padding: 0 20px 20px 0;
    }
  }

  @media (max-width: 900px) {
    .content-area {
      flex-direction: column;
    }
    .blank-detail-panel {
      flex: 1;
      width: 100%;
      height: auto;
      max-height: 60vh;
      padding: 24px;
      order: -1; /* Panel on top in mobile */
    }
  }

  /* Header Section */
  .hub-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 32px;
    width: 100%;
    gap: 24px;
    flex-wrap: wrap; /* Prevent search squeeze */
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
    width: 18px;
    height: 18px;
    color: var(--secondary); /* Changed to Cyan for better visibility */
    opacity: 0.8;
    z-index: 2;
    pointer-events: none;
  }

  .search-input {
    background: var(--glass-bg-low);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 0 16px 0 40px; /* Increased padding-left to 40px for larger icon */
    height: 40px;
    border-radius: 8px;
    font-family: var(--font-body);
    font-size: 14px;
    width: 200px;
    transition: all 0.2s;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--secondary);
    background: var(--glass-bg-mid);
    box-shadow: 0 0 0 2px rgba(0, 255, 255, 0.1);
  }

  .zoom-controls {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    padding: 0 6px;
    height: 40px;
  }

  .v-divider {
    width: 1px;
    height: 22px;
    background: var(--glass-border-high);
    opacity: 0.5;
    margin: 0 4px;
    user-select: none;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    width: 32px;
    height: 32px;
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
    width: 18px;
    height: 18px;
  }

  .view-toggles {
    display: flex;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    padding: 0 4px;
    height: 40px;
    align-items: center;
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
    .search-input { width: 100%; min-width: 180px; }
    .search-container { flex: 1; }
  }

  @media (max-width: 1000px) {
    .hub-header { 
      flex-direction: column; 
      align-items: stretch; 
      padding: 24px 20px;
      gap: 20px; 
    }
    .action-bar { 
      width: 100%; 
      justify-content: flex-start;
      flex-wrap: wrap;
      gap: 12px;
    }
    .search-container { order: -1; min-width: 250px; width: 100%; }
    .main-view { padding: 0 20px 20px; }
  }
</style>
