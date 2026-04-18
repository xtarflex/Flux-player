<script lang="ts">
  /**
   * @file DetailPanel.svelte
   * @description Library Detail Panel — displays metadata for the selected media item.
   */
  import { mediaItems, selectedMediaId, type MediaItem, loadLibraryFromDb } from '$lib/stores/media';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import { derived } from 'svelte/store';
  import { fade, fly, scale } from 'svelte/transition';
  import { quintOut, backOut } from 'svelte/easing';
  import AnimatedPlayPause from './ui/animated-icons/AnimatedPlayPause.svelte';
  import { playWithAutoQueue } from '$lib/stores/queue';
  import { tooltip } from '$lib/actions/tooltip';

  /**
   * Action to focus an element on mount
   */
  function focus(node: HTMLInputElement) {
    node.focus();
  }

  const selectedItem = derived([mediaItems, selectedMediaId], ([$items, $id]: [MediaItem[], string | null]) => {
    return $items.find((item: MediaItem) => item.id === $id) || null;
  });

  const resolvedBackdrop = derived(selectedItem, ($item) => {
    return resolveResource($item?.backdrop, '/flux_backdrop.png');
  });

  const resolvedPoster = derived(selectedItem, ($item) => {
    return $item?.poster ? resolveResource($item.poster) : null;
  });

  let playingHovered = $state(false);
  let imageError = $state(false);
  let isRefreshing = $state(false);
  let editingField = $state<string | null>(null);
  let editValue = $state("");

  const finalPoster = derived([resolvedPoster, selectedItem], ([$rp, $item]) => {
    if (imageError || !$rp) return "/flux2d.png";
    return $rp;
  });

  $effect(() => {
    // Reset error state when the item changes
    if ($selectedMediaId) imageError = false;
  });

  /**
   * Formats duration in seconds to "X min"
   */
  function formatDuration(seconds: number | null): string {
    if (!seconds || seconds <= 0) return '0 min';
    const mins = Math.round(seconds / 60);
    return `${mins} min`;
  }

  /**
   * Formats a playback position in seconds to "MM:SS" for the Resume button.
   * @param seconds - Position in seconds.
   * @returns Formatted string like "1:23" or "14:07".
   */
  function formatPosition(seconds: number): string {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, '0')}`;
  }

  /** The resume timestamp from the DB (0 = start from beginning) */
  const lastPosition = derived(selectedItem, ($item) => $item?.last_position ?? 0);

  /** Dynamic label for the primary action button */
  const resumeLabel = derived(lastPosition, ($pos) =>
    $pos > 0 ? `Resume from ${formatPosition($pos)}` : null
  );

  async function handleRefresh() {
    if (!$selectedMediaId || isRefreshing) return;
    isRefreshing = true;
    try {
      await invoke('refresh_media_metadata', { path: $selectedMediaId });
      await loadLibraryFromDb(); // Refresh store
    } catch (e) {
      console.error("Flux DetailPanel: Refresh failed:", e);
    } finally {
      isRefreshing = false;
    }
  }

  function startEdit(field: string, initialValue: string) {
    editingField = field;
    editValue = initialValue;
  }

  async function saveEdit() {
    if (!editingField || !$selectedMediaId) return;
    const field = editingField;
    const value = editValue;
    editingField = null;

    try {
      await invoke('update_media_field', { 
        path: $selectedMediaId, 
        field, 
        value 
      });
      await loadLibraryFromDb(); // Refresh store
    } catch (e) {
      console.error("Flux DetailPanel: Update failed:", e);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') saveEdit();
    if (e.key === 'Escape') editingField = null;
  }
</script>

<aside class="detail-panel">
  {#if $selectedItem}
    <!-- Cinematic Header: Backdrop + Hero Row Overlay -->
    <div class="cinematic-header">
      {#key $selectedItem.id}
        <!-- Backdrop Layer -->
        <div class="backdrop-wrapper">
          <div 
            class="backdrop-image" 
            style="background-image: url('{$resolvedBackdrop}')"
            in:fade={{ duration: 800 }}
          ></div>
          <div class="backdrop-overlay"></div>
        </div>

      <!-- Content Layer: Poster + Title Info -->
      <div class="hero-row">
        <!-- Poster -->
        <div 
          class="poster" 
          in:scale={{ duration: 600, start: 0.9, delay: 150, easing: backOut }}
        >
          <img 
            src={$finalPoster} 
            alt={$selectedItem.title} 
            class="poster-img" 
            onerror={() => imageError = true}
          />
        </div>

        <!-- Title / Meta -->
        <div 
          class="title-block"
          in:fly={{ y: 20, duration: 600, delay: 300, easing: quintOut }}
        >
          <h2 class="title">{$selectedItem.title}</h2>

          {#if $selectedItem.type === 'audio'}
            <!-- Audio Artist/Album Card beside poster -->
            <div class="artist-card artist-card--side">
              <div class="card-group">
                <span class="card-label">ARTIST</span>
                <span class="card-value">{$selectedItem.artist || 'Unknown Artist'}</span>
              </div>
              <div class="card-group">
                <span class="card-label">ALBUM</span>
                <span class="card-value">{$selectedItem.album || 'Unknown Album'}</span>
              </div>
            </div>
          {:else}
            <!-- Video Badges/Genres beside poster -->
            <div class="badges">
              <span class="badge">
                {($selectedItem.year && !isNaN($selectedItem.year)) ? $selectedItem.year : 'N/A'}
              </span>
              <span class="badge">{formatDuration($selectedItem.duration)}</span>
              {#if $selectedItem.rating && $selectedItem.rating > 0}
                <span class="badge badge--rating">★ {$selectedItem.rating.toFixed(1)}</span>
              {/if}
            </div>

            {#if $selectedItem.genres && $selectedItem.genres.length > 0}
              <div class="genre-tags">
                {#each $selectedItem.genres as genre}
                  <span class="genre-tag">{genre}</span>
                {/each}
              </div>
            {/if}
          {/if}
        </div>
      </div>
      {/key}
    </div>

    <!-- Body Information -->
    <div class="body-content">
      
      {#if $selectedItem.type === 'audio'}
        <!-- Visualiser / Audio play button -->
        <button 
          class="btn-play"
          onmouseenter={() => playingHovered = true}
          onmouseleave={() => playingHovered = false}
          onclick={() => $selectedItem && playWithAutoQueue($selectedItem, $lastPosition)}
        >
          <AnimatedPlayPause isPlaying={playingHovered} size={14} />
          {$resumeLabel ?? 'Play Audio'}
        </button>
      {:else}
        <!-- Video Specific Synopsis -->
        {#if $selectedItem.synopsis && $selectedItem.synopsis !== 'No synopsis available.' && $selectedItem.synopsis !== 'Audio track.'}
          <p class="synopsis">{$selectedItem.synopsis}</p>
        {/if}

        <!-- Play / Resume Button -->
        <button 
          class="btn-play"
          class:btn-resume={!!$resumeLabel}
          onmouseenter={() => playingHovered = true}
          onmouseleave={() => playingHovered = false}
          onclick={() => $selectedItem && playWithAutoQueue($selectedItem, $lastPosition)}
        >
          <AnimatedPlayPause isPlaying={playingHovered} size={14} />
          {$resumeLabel ?? `Play ${$selectedItem.subtitle === 'Movie' ? 'Movie' : 'Video'}`}
        </button>
      {/if}

      <!-- === Subtitle Row (Shared) === -->
      <div class="subtitle-row">
        <span class="sub-label">Subtitle:</span>
        <span class="sub-value">{$selectedItem.subtitle || 'None'}</span>
        <button class="change-btn">Change</button>
      </div>

      <!-- === Metadata Table === -->
      <div class="meta-table">
        <div class="meta-row" class:is-editing={editingField === 'title'}>
          <span class="meta-key">TITLE</span>
          {#if editingField === 'title'}
            <input 
              type="text" 
              class="edit-input" 
              bind:value={editValue} 
              onkeydown={handleKeydown}
              use:focus 
            />
          {:else}
            <span class="meta-val">{$selectedItem.title}</span>
          {/if}
          
          <div class="meta-actions">
            {#if editingField === 'title'}
              <button class="icon-action save" onclick={saveEdit} aria-label="Save" use:tooltip={{ content: 'Save', shortcut: 'Enter', placement: 'top' }}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              </button>
            {:else}
              <button class="icon-action" onclick={() => startEdit('title', $selectedItem.title)} aria-label="Edit Title" use:tooltip={{ content: 'Edit Title', placement: 'top' }}>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
                </svg>
              </button>
            {/if}

            {#if $selectedItem.type !== 'audio'}
              <button 
                class="icon-action" 
                class:is-spinning={isRefreshing}
                onclick={handleRefresh} 
                aria-label="Refresh Metadata"
                use:tooltip={{ content: 'Refresh Metadata', placement: 'top' }}
              >
                {#if isRefreshing}
                  <div class="spinner-box">
                    <svg class="flux-spinner" viewBox="0 0 24 24" fill="none">
                      <path class="flux-spin-cyan" d="M12 22 A10 10 0 0 1 12 2 A5 5 0 0 1 12 12 A5 5 0 0 0 12 22 Z" fill="var(--secondary)"/>
                      <path class="flux-spin-violet" d="M12 2 A10 10 0 0 1 12 22 A5 5 0 0 1 12 12 A5 5 0 0 0 12 2 Z" fill="var(--primary)"/>
                    </svg>
                  </div>
                {:else}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
                  </svg>
                {/if}
              </button>
            {/if}
          </div>
        </div>

        {#if $selectedItem.type === 'audio'}
          <!-- Audio Table -->
          <div class="meta-row" class:is-editing={editingField === 'artist'}>
            <span class="meta-key">ARTIST</span>
            {#if editingField === 'artist'}
              <input type="text" class="edit-input" bind:value={editValue} onkeydown={handleKeydown} use:focus />
            {:else}
              <span class="meta-val">{$selectedItem.artist || 'Unknown'}</span>
            {/if}
            <div class="meta-actions">
               {#if editingField === 'artist'}
                <button class="icon-action save" onclick={saveEdit} aria-label="Save" use:tooltip={{ content: 'Save', shortcut: 'Enter', placement: 'top' }}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg></button>
              {:else}
                <button class="icon-action" onclick={() => startEdit('artist', $selectedItem.artist || '')} aria-label="Edit Artist" use:tooltip={{ content: 'Edit Artist', placement: 'top' }}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg></button>
              {/if}
            </div>
          </div>
          <div class="meta-row" class:is-editing={editingField === 'album'}>
            <span class="meta-key">ALBUM</span>
            {#if editingField === 'album'}
              <input type="text" class="edit-input" bind:value={editValue} onkeydown={handleKeydown} use:focus />
            {:else}
              <span class="meta-val">{$selectedItem.album || 'Unknown'}</span>
            {/if}
            <div class="meta-actions">
              {#if editingField === 'album'}
                <button class="icon-action save" onclick={saveEdit} aria-label="Save" use:tooltip={{ content: 'Save', shortcut: 'Enter', placement: 'top' }}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><polyline points="20 6 9 17 4 12"/></svg></button>
              {:else}
                <button class="icon-action" onclick={() => startEdit('album', $selectedItem.album || '')} aria-label="Edit Album" use:tooltip={{ content: 'Edit Album', placement: 'top' }}><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg></button>
              {/if}
            </div>
          </div>
        {:else}
          <!-- Video Table -->
          {#if $selectedItem.director && $selectedItem.director !== 'Unknown'}
            <div class="meta-row">
              <span class="meta-key">DIRECTOR</span>
              <span class="meta-val">{$selectedItem.director}</span>
            </div>
          {/if}
          {#if $selectedItem.starring && $selectedItem.starring !== 'Unknown'}
            <div class="meta-row">
              <span class="meta-key">STARRING</span>
              <span class="meta-val">{$selectedItem.starring}</span>
            </div>
          {/if}
        {/if}
      </div>
    </div>

  {:else}
    <!-- Empty / no selection -->
    <div class="empty-state">
      <img src="/flux2d.png" alt="Flux" class="empty-logo" />
      <p>Select an item to view details</p>
    </div>
  {/if}
</aside>

<style>
  /* ===================== Layout ===================== */
  .detail-panel {
    width: 100%;
    height: 100%;
    background: var(--bg-surface);
    border: 1px solid var(--glass-border-low);
    border-radius: 12px;
    backdrop-filter: blur(24px);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overflow-x: hidden;
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.5);
    scrollbar-width: thin;
    scrollbar-color: var(--glass-border-high) transparent;
  }

  .detail-panel::-webkit-scrollbar { width: 4px; }
  .detail-panel::-webkit-scrollbar-track { background: transparent; }
  .detail-panel::-webkit-scrollbar-thumb { background: var(--glass-border-high); border-radius: 4px; }

  /* ===================== Cinematic Header ===================== */
  .cinematic-header {
    position: relative;
    width: 100%;
    min-height: 360px;
    flex-shrink: 0;
    display: flex;
    align-items: flex-end; /* Push content back down slightly */
    padding: 48px 24px 24px 24px;
    overflow: hidden;
  }

  .backdrop-wrapper {
    position: absolute;
    inset: 0;
    z-index: 0;
    pointer-events: none;
  }

  .backdrop-image {
    width: 100%;
    height: 100%;
    background-size: cover;
    background-position: center 20%;
    filter: brightness(0.6) contrast(1.15);
    /* Extended 20s slow cinematic zoom */
    animation: kenBurns 20s infinite alternate cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes kenBurns {
    0%   { transform: scale(1.0) translate(0, 0); }
    100% { transform: scale(1.18) translate(-1.5%, -1%); }
  }

  .backdrop-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      to bottom, 
      rgba(0, 0, 0, 0.1) 0%, 
      rgba(0, 0, 0, 0.3) 50%, 
      var(--bg-surface) 100%
    );
  }

  .hero-row {
    position: relative;
    z-index: 1;
    display: flex;
    gap: 20px;
    align-items: flex-end;
    width: 100%;
  }

  .poster {
    width: 170px;
    aspect-ratio: 2 / 3;
    border-radius: 10px;
    overflow: hidden;
    /* Solid matte background for transparent artwork */
    background: rgba(15, 15, 15, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.18);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7);
    position: relative;
  }

  .poster::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, transparent 50%);
    pointer-events: none;
  }

  .poster-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }



  /* ===================== Body Information ===================== */
  .body-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 0 24px 32px 24px;
  }

  /* ===================== Audio Specific Card ===================== */
  .artist-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 14px 20px;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    margin-bottom: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    width: 100%;
    box-sizing: border-box;
  }

  .artist-card--side {
    margin-bottom: 0;
    background: rgba(255, 255, 255, 0.04);
  }

  .card-group {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 12px;
  }

  .card-label {
    font-size: 11px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.1em;
    flex-shrink: 0;
  }

  .card-value {
    font-size: 13px;
    font-weight: 700;
    color: var(--secondary);
    letter-spacing: 0.02em;
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
  }

  /* ===================== Title Block ===================== */
  .title-block {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .title {
    font-family: var(--font-heading);
    font-size: 24px;
    font-weight: 700;
    color: var(--text-main);
    letter-spacing: 0.025em;
    margin: 0;
    line-height: 1.25;
    /* 3-line clamping with ellipses */
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .badges {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .badge {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.12);
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 600;
    padding: 3px 8px;
    border-radius: 4px;
  }

  .badge--rating {
    background: rgba(255, 200, 0, 0.1);
    border-color: rgba(255, 200, 0, 0.3);
    color: gold;
  }

  .genre-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
  }

  .genre-tag {
    background: rgba(0, 255, 255, 0.07);
    border: 1px solid rgba(0, 255, 255, 0.2);
    color: var(--secondary);
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 20px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  /* ===================== Synopsis ===================== */
  .synopsis {
    font-size: 15px;
    line-height: 1.65;
    color: rgba(255, 255, 255, 0.65);
    margin: 0;
  }

  /* ===================== Play Button ===================== */
  .btn-play {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    width: 100%;
    height: 42px;
    background: linear-gradient(90deg, var(--primary), var(--secondary));
    border: none;
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 16px;
    color: var(--text-main);
    cursor: pointer;
    flex-shrink: 0;
    transition: filter 0.2s, transform 0.2s;
  }

  .btn-play:hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
  }

  /* Resume variant: warm amber gradient to visually distinguish from plain Play */
  .btn-resume {
    background: linear-gradient(90deg, #c47f17, #e8a020);
  }

  .btn-resume:hover {
    filter: brightness(1.15);
  }

  /* ===================== Subtitle Row ===================== */
  .subtitle-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    flex-shrink: 0;
  }

  .sub-label {
    font-size: 12px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .sub-value {
    font-size: 13px;
    color: var(--secondary);
    flex: 1;
  }

  .change-btn {
    background: rgba(138, 43, 226, 0.15);
    border: 1px solid rgba(138, 43, 226, 0.35);
    color: var(--text-main);
    font-size: 11px;
    font-weight: 600;
    padding: 4px 12px;
    border-radius: 6px;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.2s;
  }

  .change-btn:hover { background: rgba(138, 43, 226, 0.3); }

  /* ===================== Metadata Table ===================== */
  .meta-table {
    display: flex;
    flex-direction: column;
    border-top: 1px solid var(--glass-border-low);
  }

  .meta-row {
    display: flex;
    align-items: center;
    padding: 10px 0;
    border-bottom: 1px solid var(--glass-border-low);
    gap: 12px;
  }

  .meta-key {
    width: 72px;
    flex-shrink: 0;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-muted);
  }

  .meta-val {
    flex: 1;
    font-size: 13px;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta-actions {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }

  .icon-action {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    transition: color 0.2s;
  }

  .icon-action:hover { color: var(--text-main); }
  .icon-action svg { width: 13px; height: 13px; }

  /* ===================== Empty State ===================== */
  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    opacity: 0.25;
  }

  .empty-logo { width: 56px; }

  .empty-state p {
    font-size: 13px;
    color: var(--text-muted);
  }

  /* ===================== Edit Mode ===================== */
  .edit-input {
    flex: 1;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--secondary);
    color: var(--text-main);
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 13px;
    outline: none;
  }

  .icon-action.save {
    color: var(--secondary);
  }

  /* ===================== Unified Island Spinner ===================== */
  .spinner-box {
    width: 14px;
    height: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  @keyframes flux-spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  @keyframes flux-breathe-cyan {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(-1px, -1px) scale(0.9); opacity: 1; }
  }
  @keyframes flux-breathe-violet {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(1px, 1px) scale(0.9); opacity: 1; }
  }

  .flux-spinner {
    width: 100%;
    height: 100%;
    animation: flux-spin 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
  }
  .flux-spin-cyan {
    animation: flux-breathe-cyan 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }
  .flux-spin-violet {
    animation: flux-breathe-violet 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }
</style>
