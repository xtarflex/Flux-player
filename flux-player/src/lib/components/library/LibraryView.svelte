<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { fade, slide, crossfade, fly } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  import { flip } from 'svelte/animate';
  import { onboarding, triggerTour } from '$lib/stores/onboarding';
  import MediaCard from './MediaCard.svelte';
  import Icon from '../ui/Icon.svelte';
  import Dropdown from '../ui/Dropdown.svelte';
  import DetailPanel from '../DetailPanel.svelte';
  import EmptyState from '../ui/EmptyState.svelte';
  import ContextMenu from '../ui/ContextMenu.svelte';
  import type { MenuItem } from '../ui/context-menu';
  import { mediaItems, selectedMediaId, loadLibraryFromDb, libraryLoadState, toggleFavorite, toggleWatched } from '$lib/stores/media';
  import { setMedia } from '$lib/stores/playback';
  import { playWithAutoQueue, hydrateQueue } from '$lib/stores/queue';
  import { goto } from '$app/navigation';
  import { tooltip } from '$lib/actions/tooltip';

  type ViewMode = 'grid' | 'list' | 'detail';
  let viewMode = $state<ViewMode>('grid');

  // Batch Selection State
  let isSelectionMode = $state(false);
  let selectedBatchIds = $state<string[]>([]);
  let lastSelectedIndex = $state(-1);
  let searchInput = $state<HTMLInputElement>();

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
  let isFilterOpen = $state(false);
  let isSortOpen = $state(false);

  const mediaOptions = ['All Media', 'Movies', 'Series', 'Music', 'Watched', 'Unwatched'];
  const sortOptions = ['Recently Added', 'A-Z', 'Z-A', 'Release Date', 'Rating', 'Duration'];

  let searchText = $state('');

  let filteredItems = $derived.by(() => {
    let result = [...$mediaItems];

    // Filter by type or status
    if (mediaFilter !== 'All Media') {
      if (mediaFilter === 'Movies') {
        result = result.filter(item => item.type === 'video' && !item.series_tag);
      } else if (mediaFilter === 'Series') {
        result = result.filter(item => item.type === 'video' && item.series_tag);
      } else if (mediaFilter === 'Music') {
        result = result.filter(item => item.type === 'audio');
      } else if (mediaFilter === 'Watched') {
        result = result.filter(item => item.is_watched);
      } else if (mediaFilter === 'Unwatched') {
        result = result.filter(item => !item.is_watched);
      }
    }

    // Smart Global Search (0ms Latency)
    if (searchText.trim()) {
      const query = searchText.toLowerCase();
      result = result.filter(item => 
        item.title.toLowerCase().includes(query) ||
        (item.artist && item.artist.toLowerCase().includes(query)) ||
        (item.album && item.album.toLowerCase().includes(query)) ||
        (item.series_tag && item.series_tag.toLowerCase().includes(query))
      );
    }

    // Sort
    result.sort((a, b) => {
      switch (sortOption) {
        case 'A-Z': return a.title.localeCompare(b.title);
        case 'Z-A': return b.title.localeCompare(a.title);
        case 'Recently Added': 
          return (b.added_at - a.added_at) || a.title.localeCompare(b.title);
        case 'Release Date':
          return ((b.year ?? 0) - (a.year ?? 0)) || a.title.localeCompare(b.title);
        case 'Rating':
          return ((b.rating ?? 0) - (a.rating ?? 0)) || a.title.localeCompare(b.title);
        case 'Duration':
          return ((b.duration ?? 0) - (a.duration ?? 0)) || a.title.localeCompare(b.title);
        default: return 0;
      }
    });

    return result;
  });

  function selectItem(id: string, event?: MouseEvent) {
    const idx = filteredItems.findIndex(i => i.id === id);

    // 1. Modifier Check: Auto-enter Selection Mode if Shift/Ctrl held
    if (event?.shiftKey || event?.ctrlKey) {
      if (!isSelectionMode) isSelectionMode = true;
      toggleSelection(id, event);
      return;
    }

    // 2. Standard Selection Mode Check
    if (isSelectionMode) {
      toggleSelection(id, event);
      return;
    }

    // 3. Normal Single Selection
    if ($selectedMediaId === id && viewMode === 'detail') {
      // Already selected
    } else {
      $selectedMediaId = id;
    }
    
    // Play audio immediately on single click
    const item = filteredItems[idx];
    if (item && item.type === 'audio') {
      playWithAutoQueue(item, undefined, filteredItems);
    } else if (item && item.type === 'video') {
      // Highlight/Focus video AND hydrate the footer silently
      setMedia(item);
      hydrateQueue(item, filteredItems);
    }
    
    // CRITICAL: Always update lastSelectedIndex so Shift+Click knows the start point
    lastSelectedIndex = idx;
  }

  function handleDoubleClick(id: string) {
    const item = filteredItems.find(i => i.id === id);
    if (!item) return;

    // Double-click always uses auto-queue logic and routes to Now Playing
    playWithAutoQueue(item, undefined, filteredItems);
    goto('/playing');
  }

  function toggleSelection(id: string, event?: MouseEvent) {
    const currentIndex = filteredItems.findIndex(i => i.id === id);

    if (event?.shiftKey && lastSelectedIndex !== -1) {
      const [start, end] = lastSelectedIndex < currentIndex 
        ? [lastSelectedIndex, currentIndex] 
        : [currentIndex, lastSelectedIndex];
      
      const rangeIds = filteredItems.slice(start, end + 1).map(i => i.id);
      
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

  function handleGlobalKeydown(e: KeyboardEvent) {
    // 1. Context Check: Ignore if typing
    const target = e.target as HTMLElement;
    const isEditing = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;

    if (isEditing) {
      if (e.key === 'Escape') {
        searchInput?.blur();
        if (searchText) searchText = '';
      }
      return;
    }

    const isCmd = e.ctrlKey || e.metaKey;

    // 2. Global Toggles & Navigation
    if (e.key === 'Escape') {
      if (menuVisible) { menuVisible = false; return; }
      if (isSelectionMode) { exitSelectionMode(); return; }
      if (searchText) { searchText = ''; return; }
    }

    // 3. Library Search Focus
    if (e.key === '/' || (isCmd && e.key === 'f')) {
      e.preventDefault();
      searchInput?.focus();
    }

    // 4. Batch Operations
    if (isCmd && e.key.toLowerCase() === 'a') {
      e.preventDefault();
      if (e.shiftKey) {
        selectedBatchIds = [];
      } else {
        isSelectionMode = true;
        selectedBatchIds = filteredItems.map(i => i.id);
      }
    }

    if (isCmd && e.key.toLowerCase() === 'i') {
      e.preventDefault();
      if (!isSelectionMode) isSelectionMode = true;
      const currentSet = new Set(selectedBatchIds);
      selectedBatchIds = filteredItems
        .filter(item => !currentSet.has(item.id))
        .map(item => item.id);
    }

    // 5. View & Zoom Cycle
    if (e.key.toLowerCase() === 'v' && !isCmd) {
      const modes: ViewMode[] = ['grid', 'list', 'detail'];
      const nextIdx = (modes.indexOf(viewMode) + 1) % modes.length;
      viewMode = modes[nextIdx];
    }

    if (isCmd && (e.key === '=' || e.key === '+')) { e.preventDefault(); zoomIn(); }
    if (isCmd && e.key === '-') { e.preventDefault(); zoomOut(); }
    if (isCmd && e.key === '0') { e.preventDefault(); gridZoomIndex = 1; listZoomIndex = 0; }

    // 6. View & Sort (Section 3.B)
    if (!isCmd) {
      const active = document.activeElement as HTMLElement;
      const isCardFocused = active?.classList.contains('media-card');

      switch(e.key.toLowerCase()) {
        case 's':
          e.preventDefault();
          isSortOpen = !isSortOpen;
          isFilterOpen = false;
          break;
        case 'f':
          e.preventDefault();
          if (!isCardFocused) {
            isFilterOpen = !isFilterOpen;
            isSortOpen = false;
          }
          break;
        case 'delete':
          if (isCardFocused) {
            e.preventDefault();
            const deep = e.shiftKey;
            window.dispatchEvent(new CustomEvent('flux-toast', { detail: { 
              label: deep ? 'Permanent Delete' : 'Remove from Library', 
              icon: deep ? 'check' : 'check' 
            }}));
          }
          break;
        case 'e':
          if (isCardFocused) {
            e.preventDefault();
            window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Edit Metadata', icon: 'settings' } }));
          }
          break;
        case 'r':
          if (isCardFocused) { e.preventDefault(); /* Global scanning state handles feedback */ }
          break;
        case 'i':
          if (isCardFocused) { e.preventDefault(); window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Media Info', icon: 'library' } })); }
          break;
        case 'w':
          if (isCardFocused) { 
            e.preventDefault(); 
            // Get the item ID from the focused element
            const id = active.getAttribute('data-id');
            if (id) {
              const item = $mediaItems.find(i => i.id === id);
              if (item) toggleWatched(item.path);
            }
          }
          break;
      }
    }

    // 7. Grid Navigation
    if (['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(e.key) && !isCmd) {
      const active = document.activeElement as HTMLElement;
      const isCardFocused = active?.classList.contains('media-card');
      
      if (isCardFocused) {
        e.preventDefault();
        const cards = Array.from(document.querySelectorAll('.media-grid .media-card')) as HTMLElement[];
        const index = cards.indexOf(active);
        if (index === -1) return;

        // Position-based column detection (100% accurate)
        let cols = 1;
        if (cards.length > 1) {
          const firstTop = cards[0].getBoundingClientRect().top;
          for (let i = 1; i < cards.length; i++) {
            if (cards[i].getBoundingClientRect().top > firstTop + 10) {
              cols = i;
              break;
            }
          }
        }

        let nextIdx = index;
        switch (e.key) {
          case 'ArrowLeft': nextIdx = Math.max(0, index - 1); break;
          case 'ArrowRight': nextIdx = Math.min(cards.length - 1, index + 1); break;
          case 'ArrowUp': nextIdx = Math.max(0, index - cols); break;
          case 'ArrowDown': nextIdx = Math.min(cards.length - 1, index + cols); break;
        }

        cards[nextIdx]?.focus();
      }
    }
  }

  onMount(() => {
    // Orchestrate Onboarding Flow for Library
    if (!$onboarding.isActive && $onboarding.completedSections.includes('global') && !$onboarding.completedSections.includes('library')) {
      triggerTour('library');
    }
    
    // Subscribe to onboarding changes to cascade from Global -> Library automatically
    const unsubOnboarding = onboarding.subscribe(state => {
      if (!state.isActive && state.completedSections.includes('global') && !state.completedSections.includes('library')) {
        setTimeout(() => triggerTour('library'), 500); // Small delay for smooth transition
      }
    });

    // Load real data from SQLite immediately
    loadLibraryFromDb();

    const handleFocus = () => searchInput?.focus();
    window.addEventListener('flux-search-focus', handleFocus);

    // Re-fetch after any library scan completes
    const handleLibraryUpdate = () => loadLibraryFromDb();
    window.addEventListener('flux-library-updated', handleLibraryUpdate);

    return () => {
      window.removeEventListener('flux-search-focus', handleFocus);
      window.removeEventListener('flux-library-updated', handleLibraryUpdate);
      unsubOnboarding();
    };
  });

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
    if (!menuTarget) return;

    switch (action) {
      case 'play':
        playWithAutoQueue(menuTarget, undefined, filteredItems);
        break;
      case 'favorite':
        toggleFavorite(menuTarget.id);
        break;
      case 'watched':
        toggleWatched(menuTarget.path);
        break;
      case 'details':
        $selectedMediaId = menuTarget.id;
        viewMode = 'detail';
        break;
    }
    menuVisible = false;
  }

  let menuItems = $derived.by(() => {
    if (!menuTarget) return [] as MenuItem[];
    
    const currentItem = $mediaItems.find(i => i.id === menuTarget.id);
    const favoriteLabel = currentItem?.is_favorite ? 'Remove from Favorite' : 'Add to Favorite';

    return [
      { label: 'Play Now', action: () => handleMenuAction('play') },
      { label: 'Add to Queue', action: () => handleMenuAction('queue') },
      { separator: true },
      { label: currentItem?.is_watched ? 'Mark as Unwatched' : 'Mark as Watched', action: () => handleMenuAction('watched') },
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
      { label: favoriteLabel, action: () => handleMenuAction('favorite') },
      { label: 'Get Subtitles', action: () => handleMenuAction('subtitles') },
      { label: 'Remove from Library', danger: true, action: () => handleMenuAction('remove') },
      { separator: true },
      { label: 'Details', action: () => handleMenuAction('details') },
    ] as MenuItem[];
  });
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div 
  class="discovery-hub" 
  onclick={(e: MouseEvent) => {
    // Only deselect if we clicked the actual background (not a card, bar, or modal)
    const target = e.target as HTMLElement;
    if (target.closest('.media-card, .batch-action-bar, .action-bar, .context-menu, .dropdown, .detail-panel')) {
      return;
    }
    
    $selectedMediaId = '';
    if (isSelectionMode) exitSelectionMode();
  }}
  role="presentation"
>
  <div class="hub-header">
    <div class="hub-title-section">
      <h1 class="hub-title">Your <span class="uppercase">Library</span></h1>
      <span class="item-count">{filteredItems.length} Items</span>
    </div>

    <div class="action-bar">
      <!-- Search Input -->
      <div class="search-container">
        <Icon name="search" size={18} class="search-icon" />
        <input 
          type="text" 
          class="search-input" 
          placeholder="Search library..." 
          bind:value={searchText}
          bind:this={searchInput}
        />
      </div>

      <div id="onboard-filters" style="display: flex; gap: 16px;">
        <Dropdown options={mediaOptions} bind:value={mediaFilter} bind:isOpen={isFilterOpen} showCheckmark={false} />
        <Dropdown options={sortOptions} bind:value={sortOption} bind:isOpen={isSortOpen} showCheckmark={false} />
      </div>

      <!-- Zoom Controls -->
      <div id="onboard-zoom" class="zoom-controls" style:opacity={disableZoom ? 0.3 : 1} style:pointer-events={disableZoom ? 'none' : 'auto'}>
        <button class="icon-btn" onclick={zoomOut} aria-label="Zoom Out" use:tooltip={{ content: 'Smaller Grid', shortcut: 'Ctrl -', placement: 'bottom' }} disabled={disableZoom}>
          <Icon name="zoom-out" size={16} />
        </button>
        <div class="v-divider"></div>
        <button class="icon-btn" onclick={zoomIn} aria-label="Zoom In" use:tooltip={{ content: 'Larger Grid', shortcut: 'Ctrl +', placement: 'bottom' }} disabled={disableZoom}>
          <Icon name="zoom-in" size={16} />
        </button>
      </div>

      <!-- View Toggles -->
      <div id="onboard-views" class="view-toggles">
        <button class="toggle-btn" class:active={viewMode === 'grid'} onclick={() => viewMode = 'grid'} aria-label="Grid View">
          <Icon name="grid-view" size={20} />
        </button>
        <button class="toggle-btn" class:active={viewMode === 'list'} onclick={() => viewMode = 'list'} aria-label="List View">
          <Icon name="list-view" size={20} />
        </button>
        <button id="onboard-detail-trigger" class="toggle-btn" class:active={viewMode === 'detail'} onclick={() => viewMode = 'detail'} aria-label="Detail View">
          <Icon name="library" size={20} />
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
            <button class="action-btn" onclick={() => console.log('Playing Batch', selectedBatchIds)} use:tooltip={{ content: 'Play Selection', placement: 'bottom' }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 3l14 9-14 9V3z"/></svg>
              <span>Play All</span>
            </button>
            <button class="action-btn" onclick={() => console.log('Queueing Batch', selectedBatchIds)} use:tooltip={{ content: 'Add to Queue', placement: 'bottom' }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"/></svg>
              <span>Queue</span>
            </button>
            
            <div class="action-divider"></div>
            
            <button class="action-btn" onclick={() => console.log('Playlist Batch', selectedBatchIds)} use:tooltip={{ content: 'Add to Playlist', placement: 'bottom' }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
              <span>Playlist</span>
            </button>
            <button class="action-btn" onclick={() => console.log('Favorite Batch', selectedBatchIds)} use:tooltip={{ content: 'Toggle Favorites', placement: 'bottom' }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>
              <span>Favorite</span>
            </button>
            <button class="action-btn danger" onclick={() => console.log('Remove Batch', selectedBatchIds)} use:tooltip={{ content: 'Remove Selection', placement: 'bottom' }}>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
              <span>Remove</span>
            </button>
          </div>

          <button class="exit-btn" onclick={exitSelectionMode} aria-label="Cancel Selection" use:tooltip={{ content: 'Cancel Selection', shortcut: 'Esc', placement: 'bottom' }}>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12" stroke-linecap="round" stroke-linejoin="round"/></svg>
          </button>
        </div>
      {/if}

      <div class="media-grid" style={gridStyle}>
        {#if $libraryLoadState === 'loading'}
          <div class="grid-state">
            <div class="grid-spinner"></div>
            <p>Loading library…</p>
          </div>
        {:else if filteredItems.length === 0 && searchText}
          <div class="grid-state">
            <EmptyState
              variant="search"
              title="No matches found" 
              description="We couldn't find any items matching '{searchText}'. Try a different keyword or check your filters."
              actionLabel="Clear Search"
              onAction={() => { searchText = ''; }}
            />
          </div>
        {:else if $mediaItems.length === 0}
          <div class="grid-state">
            <EmptyState
              variant="library"
              title="Your Library is Empty" 
              description="Flux Player hasn't found any media files yet. Connect a folder to begin scanning your collection."
              actionLabel="Add Media Folder"
              onAction={() => window.dispatchEvent(new CustomEvent('flux-import-folder'))}
            />
          </div>
        {:else if filteredItems.length === 0}
          <div class="grid-state">
            <EmptyState
              variant="search"
              title="No matches found" 
              description="We couldn't find any items matching your current filters."
              actionLabel="Reset Filters"
              onAction={() => { searchText = ''; mediaFilter = 'All Media'; }}
            />
          </div>
        {:else if viewMode !== 'detail'}
          {#each filteredItems as item, i (item.id)}
            <div 
              style="display: contents;" 
              in:fly={{ y: 24, duration: 600, delay: Math.min(i * 35, 1200), easing: quintOut }}
            >
              <MediaCard 
                {item} 
                {viewMode} 
                selected={$selectedMediaId === item.id}
                selectionMode={isSelectionMode}
                batchSelected={selectedBatchIds.includes(item.id)}
                onclick={(e: MouseEvent) => selectItem(item.id, e)}
                ondblclick={() => handleDoubleClick(item.id)}
                onmenu={(e: MouseEvent) => openMenu(e, item)}
                onrightclick={(e: MouseEvent) => handleCardContextMenu(e, item.id)}
              />
            </div>
          {/each}
        {:else}
          <!-- In detail mode, left pane always shows as list -->
          {#each filteredItems as item, i (item.id)}
            <div 
              style="display: contents;" 
              in:fly={{ y: 16, duration: 500, delay: Math.min(i * 25, 1000), easing: quintOut }}
            >
              <MediaCard 
                {item} 
                viewMode="list" 
                selected={$selectedMediaId === item.id}
                selectionMode={isSelectionMode}
                batchSelected={selectedBatchIds.includes(item.id)}
                onclick={(e: MouseEvent) => selectItem(item.id, e)}
                ondblclick={() => handleDoubleClick(item.id)}
                onmenu={(e: MouseEvent) => openMenu(e, item)}
                onrightclick={(e: MouseEvent) => handleCardContextMenu(e, item.id)}
              />
            </div>
          {/each}
        {/if}
      </div>
    </div>

    {#if viewMode === 'detail'}
      <div 
        class="blank-detail-panel" 
        in:fly={{ x: 20, duration: 600, delay: 200, easing: quintOut }}
      >
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

  :global(.search-icon) {
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

  .icon-btn :global(svg) {
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

  .toggle-btn :global(svg) {
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

  /* Loading & Empty States */
  .grid-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 80px 32px;
    color: var(--text-muted);
    text-align: center;
  }

  .grid-state p {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
    margin: 0;
  }
  .grid-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(138, 43, 226, 0.2);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: gridSpin 0.8s linear infinite;
  }

  @keyframes gridSpin {
    to { transform: rotate(360deg); }
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
