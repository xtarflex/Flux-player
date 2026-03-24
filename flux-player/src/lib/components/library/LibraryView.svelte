<script lang="ts">
  import MediaCard from './MediaCard.svelte';
  import Dropdown from '../ui/Dropdown.svelte';
  import DetailPanel from '../DetailPanel.svelte';
  import ContextMenu from '../ui/ContextMenu.svelte';
  import type { MenuItem } from '../ui/context-menu';
  import { mediaItems, selectedMediaId } from '$lib/stores/media';

  type ViewMode = 'grid' | 'list' | 'detail';
  let viewMode = $state<ViewMode>('grid');

  // Batch Selection State
  let isSelectionMode = $state(false);
  let selectedBatchIds = $state<string[]>([]);
  let lastSelectedIndex = $state(-1);

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

  function selectItem(id: string, event?: MouseEvent) {
    if (isSelectionMode) {
      toggleSelection(id, event);
      return;
    }

    if ($selectedMediaId === id && viewMode === 'detail') {
      // Already selected
    } else {
      $selectedMediaId = id;
    }
  }

  function toggleSelection(id: string, event?: MouseEvent) {
    const currentIndex = filteredItems.findIndex(i => i.id === id);

    if (event?.shiftKey && lastSelectedIndex !== -1) {
      const [start, end] = lastSelectedIndex < currentIndex 
        ? [lastSelectedIndex, currentIndex] 
        : [currentIndex, lastSelectedIndex];
      
      const rangeIds = filteredItems.slice(start, end + 1).map(i => i.id);
      
      // If majority of range is selected, deselect all. Otherwise select all.
      const alreadySelectedInRange = rangeIds.filter(rid => selectedBatchIds.includes(rid));
      
      if (alreadySelectedInRange.length > rangeIds.length / 2) {
        selectedBatchIds = selectedBatchIds.filter(rid => !rangeIds.includes(rid));
      } else {
        const newSelection = [...selectedBatchIds];
        rangeIds.forEach(rid => {
          if (!newSelection.includes(rid)) newSelection.push(rid);
        });
        selectedBatchIds = newSelection;
      }
    } else {
      if (selectedBatchIds.includes(id)) {
        selectedBatchIds = selectedBatchIds.filter(rid => rid !== id);
      } else {
        selectedBatchIds = [...selectedBatchIds, id];
      }
    }
    
    lastSelectedIndex = currentIndex;
  }

  function exitSelectionMode() {
    isSelectionMode = false;
    selectedBatchIds = [];
    lastSelectedIndex = -1;
  }

  function enterSelectionMode(id?: string) {
    isSelectionMode = true;
    if (id && !selectedBatchIds.includes(id)) {
      selectedBatchIds = [...selectedBatchIds, id];
      lastSelectedIndex = filteredItems.findIndex(i => i.id === id);
    }
    // Also close any open context menu
    menuVisible = false;
  }

  function handleCardContextMenu(e: MouseEvent, id: string) {
    e.preventDefault();
    e.stopPropagation();
    
    if (!isSelectionMode) {
      enterSelectionMode(id);
    } else {
      toggleSelection(id, e);
    }
  }

  // Context Menu Logic
  let menuVisible = $state(false);
  let menuPos = $state({ x: 0, y: 0 });
  let menuTarget = $state<any>(null);

  function openMenu(e: MouseEvent, item: any) {
    menuPos = { x: e.clientX, y: e.clientY };
    menuTarget = item;
    menuVisible = true;
  }

  function handleMenuAction(action: string) {
    console.log(`Action: ${action} on ${menuTarget?.title}`);
    // Future: Connect to PlaylistStore and PlayerEngine
    menuVisible = false;
  }

  let menuItems = $derived.by(() => {
    if (!menuTarget) return [] as MenuItem[];
    
    return [
      { label: 'Play Now', action: () => handleMenuAction('play') },
      { label: 'Add to Queue', action: () => handleMenuAction('queue') },
      { separator: true },
      { label: isSelectionMode ? 'Exit Selection Mode' : 'Batch Select', action: () => isSelectionMode ? exitSelectionMode() : enterSelectionMode(menuTarget.id) },
      { 
        label: 'Add to Playlist', 
        children: [
          { label: 'Recently Played', action: () => handleMenuAction('playlist-recent') },
          { label: 'Favorites', action: () => handleMenuAction('playlist-favs') },
          { separator: true },
          { label: '+ New Playlist', action: () => handleMenuAction('playlist-new') },
        ]
      },
      { label: 'Add to Favorite', action: () => handleMenuAction('favorite') },
      { label: 'Get Subtitles', action: () => handleMenuAction('subtitles') },
      { label: 'Remove from Library', danger: true, action: () => handleMenuAction('remove') },
      { separator: true },
      { label: 'Details', action: () => handleMenuAction('details') },
    ] as MenuItem[];
  });
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
      {#if isSelectionMode}
        <div class="batch-action-bar glass">
          <div class="selection-info">
            <span class="count-bubble">{selectedBatchIds.length}</span>
            <span class="selection-label">Items Selected</span>
          </div>

          <div class="action-divider"></div>

          <div class="batch-actions">
            <button class="action-btn" onclick={() => console.log('Playing Batch', selectedBatchIds)} title="Play Selection">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 3l14 9-14 9V3z"/></svg>
              <span>Play All</span>
            </button>
            <button class="action-btn" onclick={() => console.log('Queueing Batch', selectedBatchIds)} title="Add to Queue">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"/></svg>
              <span>Queue</span>
            </button>
            
            <div class="action-divider"></div>
            
            <button class="action-btn" onclick={() => console.log('Playlist Batch', selectedBatchIds)} title="Add to Playlist">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              <span>Playlist</span>
            </button>
            <button class="action-btn" onclick={() => console.log('Favorite Batch', selectedBatchIds)} title="Toggle Favorites">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
              <span>Favorite</span>
            </button>
            <button class="action-btn danger" onclick={() => console.log('Remove Batch', selectedBatchIds)} title="Remove Selection">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
              <span>Remove</span>
            </button>
          </div>

          <button class="exit-btn" onclick={exitSelectionMode} title="Cancel Selection">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/></svg>
          </button>
        </div>
      {/if}

      <div class="media-grid" style={gridStyle}>
        {#if viewMode !== 'detail'}
          {#each filteredItems as item (item.id)}
            <MediaCard 
              {item} 
              {viewMode} 
              selected={$selectedMediaId === item.id}
              selectionMode={isSelectionMode}
              batchSelected={selectedBatchIds.includes(item.id)}
              onclick={(e: MouseEvent) => selectItem(item.id, e)}
              onmenu={(e: MouseEvent) => openMenu(e, item)}
              onrightclick={(e: MouseEvent) => handleCardContextMenu(e, item.id)}
            />
          {/each}
        {:else}
          <!-- In detail mode, left pane always shows as list -->
          {#each filteredItems as item (item.id)}
            <MediaCard 
              {item} 
              viewMode="list" 
              selected={$selectedMediaId === item.id}
              selectionMode={isSelectionMode}
              batchSelected={selectedBatchIds.includes(item.id)}
              onclick={(e: MouseEvent) => selectItem(item.id, e)}
              onmenu={(e: MouseEvent) => openMenu(e, item)}
              onrightclick={(e: MouseEvent) => handleCardContextMenu(e, item.id)}
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

{#if menuVisible}
  <ContextMenu 
    x={menuPos.x} 
    y={menuPos.y} 
    items={menuItems} 
    onclose={() => (menuVisible = false)} 
  />
{/if}

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

  .batch-action-bar {
    position: fixed;
    bottom: 110px;
    left: calc(50% + 125px);
    transform: translateX(-50%);
    z-index: 1000;
    width: auto;
    min-width: 620px;
    background: rgba(14, 14, 16, 0.85); /* Deep frosted obsidian */
    backdrop-filter: blur(40px) saturate(220%);
    -webkit-backdrop-filter: blur(40px) saturate(220%);
    border: 1px solid var(--glass-border-mid);
    border-radius: 28px;
    padding: 10px 14px 10px 24px;
    display: flex;
    align-items: center;
    gap: 28px;
    box-shadow: 0 24px 80px rgba(0, 0, 0, 0.9);
    animation: barSlideUp 0.45s cubic-bezier(0.23, 1, 0.32, 1);
  }

  @keyframes barSlideUp {
    from { transform: translate(-50%, 40px); opacity: 0; }
    to { transform: translate(-50%, 0); opacity: 1; }
  }

  .selection-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .count-bubble {
    background: var(--primary);
    color: white;
    font-weight: 700;
    min-width: 24px;
    height: 24px;
    padding: 0 8px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px;
  }

  .selection-label {
    font-weight: 600;
    color: var(--text-main);
    font-size: 15px;
    letter-spacing: -0.01em;
  }

  .batch-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-grow: 0;
  }

  .action-btn {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    color: var(--text-main);
    padding: 10px 20px;
    border-radius: 14px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    transition: all 0.25s ease;
    white-space: nowrap;
    letter-spacing: 0.02em;
  }

  .action-btn:hover {
    background: var(--glass-bg-high);
    border-color: var(--glass-border-high);
    transform: scale(1.03);
  }

  .action-btn.danger {
    color: #ff5555; /* Soft Red Protective Tint */
  }

  .action-btn.danger:hover {
    background: rgba(255, 85, 85, 0.1);
    border-color: rgba(255, 85, 85, 0.3);
  }

  .action-btn svg {
    width: 17px;
    height: 17px;
    opacity: 0.85;
  }

  .action-btn svg {
    width: 16px;
    height: 16px;
  }

  .action-btn.favorite:hover {
    color: #ff9d00;
  }

  .action-btn.danger:hover {
    background: rgba(255, 68, 68, 0.1);
    color: #ff4444;
    border-color: rgba(255, 68, 68, 0.2);
  }

  .exit-btn {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--glass-bg-low);
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .exit-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-main);
  }

  .exit-btn svg {
    width: 18px;
    height: 18px;
  }

  .action-divider {
    width: 1px;
    height: 24px;
    background: var(--glass-border-low);
  }
</style>
