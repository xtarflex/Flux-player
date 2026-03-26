<script lang="ts">
  /**
   * @file DetailPanel.svelte
   * @description Library Detail Panel — displays metadata for the selected media item.
   */
  import { mediaItems, selectedMediaId, type MediaItem } from '$lib/stores/media';
  import { derived } from 'svelte/store';
  import { playerActions } from '$lib/stores/player';
  import AnimatedPlayPause from './ui/animated-icons/AnimatedPlayPause.svelte';

  const selectedItem = derived([mediaItems, selectedMediaId], ([$items, $id]: [MediaItem[], string | null]) => {
    return $items.find((item: MediaItem) => item.id === $id) || null;
  });

  let playingHovered = $state(false);

  function formatTime(seconds: number) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
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
            style="background-image: url('{$selectedItem.backdrop || '/flux_backdrop.png'}')"
          ></div>
          <div class="backdrop-overlay"></div>
        </div>

      <!-- Content Layer: Poster + Title Info -->
      <div class="hero-row">
        <!-- Poster -->
        <div class="poster">
          {#if $selectedItem.poster && !$selectedItem.poster.includes('flux2d')}
            <img src={$selectedItem.poster} alt={$selectedItem.title} class="poster-img" />
          {:else}
            <div class="poster-placeholder">
              <img src="/flux2d.png" alt="Flux" />
            </div>
          {/if}
        </div>

        <!-- Title / Meta -->
        <div class="title-block">
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
              <span class="badge">{$selectedItem.year || 'N/A'}</span>
              <span class="badge">{$selectedItem.duration || '0m'}</span>
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
        <!-- Visualizer Button -->
        <button 
          class="btn-play"
          onmouseenter={() => playingHovered = true}
          onmouseleave={() => playingHovered = false}
        >
          <AnimatedPlayPause isPlaying={playingHovered} size={14} />
          Visualizer
        </button>
      {:else}
        <!-- Video Specific Synopsis -->
        {#if $selectedItem.synopsis && $selectedItem.synopsis !== 'No synopsis available.' && $selectedItem.synopsis !== 'Audio track.'}
          <p class="synopsis">{$selectedItem.synopsis}</p>
        {/if}

        <!-- Action Buttons -->
        <div class="action-row">
          <button 
            class="btn-play"
            onmouseenter={() => playingHovered = true}
            onmouseleave={() => playingHovered = false}
            onclick={() => playerActions.play($selectedItem)}
          >
            <AnimatedPlayPause isPlaying={playingHovered} size={14} />
            Play {$selectedItem.subtitle === 'Movie' ? 'Movie' : 'Video'}
          </button>

          {#if $selectedItem.last_position > 5 && !$selectedItem.is_watched}
            <button 
              class="btn-resume"
              onclick={() => playerActions.play($selectedItem)}
            >
              Resume from {formatTime($selectedItem.last_position)}
            </button>
          {/if}
        </div>
      {/if}

      <!-- === Subtitle Row (Shared) === -->
      <div class="subtitle-row">
        <span class="sub-label">Subtitle:</span>
        <span class="sub-value">{$selectedItem.subtitle || 'None'}</span>
        <button class="change-btn">Change</button>
      </div>

      <!-- === Metadata Table === -->
      <div class="meta-table">
        <div class="meta-row">
          <span class="meta-key">TITLE</span>
          <span class="meta-val">{$selectedItem.title}</span>
          <div class="meta-actions">
            <button class="icon-action" title="Edit">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/>
              </svg>
            </button>
            {#if $selectedItem.type !== 'audio'}
              <button class="icon-action" title="Reset">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="1 4 1 10 7 10"/><path d="M3.51 15a9 9 0 102.13-9.36L1 10"/>
                </svg>
              </button>
            {/if}
          </div>
        </div>

        {#if $selectedItem.type === 'audio'}
          <!-- Audio Table -->
          <div class="meta-row">
            <span class="meta-key">ARTIST</span>
            <span class="meta-val">{$selectedItem.artist || 'Unknown'}</span>
            <div class="meta-actions"><button class="icon-action" title="Edit"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg></button></div>
          </div>
          <div class="meta-row">
            <span class="meta-key">ALBUM</span>
            <span class="meta-val">{$selectedItem.album || 'Unknown'}</span>
            <div class="meta-actions"><button class="icon-action" title="Edit"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7"/><path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z"/></svg></button></div>
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
    min-height: 320px;
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
    /* Balanced 8s cinematic loop */
    animation: kenBurns 8s infinite alternate cubic-bezier(0.445, 0.05, 0.55, 0.95);
  }

  @keyframes kenBurns {
    from { transform: scale(1.0) translate(0, 0); }
    to   { transform: scale(1.18) translate(1%, 2%); }
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
    width: 140px;
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

  .poster-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.3;
  }

  .poster-placeholder img {
    width: 50%;
    object-fit: contain;
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
  .action-row {
    display: flex;
    gap: 12px;
    width: 100%;
    flex-shrink: 0;
  }

  .btn-play {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    flex: 1;
    height: 42px;
    background: linear-gradient(90deg, var(--primary), var(--secondary));
    border: none;
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 16px;
    color: var(--text-main);
    cursor: pointer;
    transition: filter 0.2s, transform 0.2s;
  }

  .btn-resume {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    height: 42px;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 600;
    font-size: 14px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-resume:hover {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
    color: var(--secondary);
  }

  .btn-play :global(svg) { width: 14px; height: 14px; }

  .btn-play:hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
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
</style>
