<script lang="ts">
  import { mediaItems, selectedMediaId } from '$lib/stores/media';
  import { derived } from 'svelte/store';

  export let isOpen = false;

  const selectedItem = derived([mediaItems, selectedMediaId], ([$items, $id]) => {
    return $items.find(i => i.id === $id) || null;
  });

  function closePanel() {
    isOpen = false;
  }
</script>

<aside class="detail-panel" class:open={isOpen}>
  {#if $selectedItem}
    <div class="hero-section">
      <div class="backdrop" style="background-image: url('{$selectedItem.backdrop}')"></div>

      <div class="hero-content">
        <div class="poster" style="background-image: url('{$selectedItem.poster}')"></div>
        <div class="metadata">
          <h2 class="title">{$selectedItem.title}</h2>
          <div class="info-row">
            <span>{$selectedItem.year}</span>
            <span class="separator">•</span>
            <span>{$selectedItem.duration}</span>
            <span class="separator">•</span>
            <span class="rating">★ {$selectedItem.rating.toFixed(1)}</span>
          </div>
          <div class="genres">
            {#each $selectedItem.genres as genre}
              <span class="genre-badge">{genre}</span>
            {/each}
          </div>
        </div>
      </div>
    </div>

    <div class="scrollable-content">
      <p class="synopsis">{$selectedItem.synopsis}</p>

      <button class="play-btn">
        <span class="icon">▶</span> Play
      </button>

      <div class="subtitle-row">
        <span class="label">Subtitle:</span>
        <span class="value">{$selectedItem.subtitle}</span>
        <button class="change-btn">Change</button>
      </div>

      <div class="metadata-grid">
        <div class="metadata-row">
          <span class="row-label">TITLE</span>
          <span class="row-value">{$selectedItem.title}</span>
        </div>
        <div class="metadata-row">
          <span class="row-label">ARTIST</span>
          <span class="row-value">{$selectedItem.artist}</span>
        </div>
        <div class="metadata-row">
          <span class="row-label">ALBUM</span>
          <span class="row-value">{$selectedItem.album}</span>
        </div>
        {#if $selectedItem.director && $selectedItem.director !== "Unknown"}
          <div class="metadata-row">
            <span class="row-label">DIRECTOR</span>
            <span class="row-value">{$selectedItem.director}</span>
          </div>
        {/if}
        {#if $selectedItem.starring && $selectedItem.starring !== "Unknown"}
          <div class="metadata-row">
            <span class="row-label">STARRING</span>
            <span class="row-value">{$selectedItem.starring}</span>
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>Select an item to view details</p>
    </div>
  {/if}
</aside>

<style>
  .detail-panel {
    position: fixed;
    top: var(--titlebar-height);
    right: -100%;
    bottom: var(--footer-height);
    width: 35%;
    min-width: 400px;
    max-width: 600px;
    background-color: var(--bg-surface);
    border-left: 1px solid rgba(138, 43, 226, 0.2);
    transition: right 0.4s cubic-bezier(0.23, 1, 0.320, 1);
    z-index: 100;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .detail-panel.open {
    right: 0;
  }

  .hero-section {
    position: relative;
    height: 40%;
    min-height: 250px;
    flex-shrink: 0;
  }

  .backdrop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-size: cover;
    background-position: center;
    background-repeat: no-repeat;
    filter: brightness(0.7);
  }

  .backdrop::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(to bottom, rgba(0,0,0,0) 0%, rgba(26,26,31,0.8) 100%);
  }

  .hero-content {
    position: absolute;
    bottom: -80px;
    left: 24px;
    right: 24px;
    display: flex;
    align-items: flex-end;
    gap: 24px;
    z-index: 2;
  }

  .poster {
    width: 180px;
    height: 270px;
    background-size: cover;
    background-position: center;
    border: 2px solid rgba(138, 43, 226, 0.5);
    border-radius: 8px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    flex-shrink: 0;
    background-color: var(--bg-base); /* fallback */
  }

  .metadata {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding-bottom: 84px; /* Account for the poster extending below */
    flex-grow: 1;
    overflow: hidden;
  }

  .title {
    font-family: var(--font-heading);
    font-weight: 700;
    font-size: 24px;
    color: #f8f9fa;
    letter-spacing: 0.05em;
    margin: 0;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.2;
  }

  .info-row {
    font-family: var(--font-body);
    font-size: 14px;
    color: rgba(248, 249, 250, 0.7);
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .separator {
    font-size: 16px;
  }

  .rating {
    color: #ffd700;
  }

  .genres {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 4px;
  }

  .genre-badge {
    background: rgba(138, 43, 226, 0.2);
    border: 1px solid rgba(138, 43, 226, 0.4);
    border-radius: 12px;
    padding: 4px 12px;
    font-family: var(--font-body);
    font-size: 12px;
    color: #f8f9fa;
  }

  .scrollable-content {
    flex-grow: 1;
    overflow-y: auto;
    padding: 100px 24px 24px 24px; /* Top padding to clear the poster */
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .synopsis {
    font-family: var(--font-body);
    font-size: 15px;
    line-height: 1.6;
    color: rgba(248, 249, 250, 0.8);
    margin: 0;
    display: -webkit-box;
    -webkit-line-clamp: 4;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .play-btn {
    width: 100%;
    height: 48px;
    background: linear-gradient(90deg, #8a2be2, #00ffff);
    border: none;
    border-radius: 8px;
    font-family: var(--font-body);
    font-weight: 700;
    font-size: 16px;
    color: #ffffff;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    transition: all 0.2s ease;
  }

  .play-btn:hover {
    filter: brightness(1.1);
    transform: scale(1.02);
  }

  .subtitle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 8px;
  }

  .label {
    font-family: var(--font-body);
    font-size: 14px;
    color: var(--text-muted);
  }

  .value {
    font-family: var(--font-body);
    font-size: 14px;
    color: var(--secondary);
    flex-grow: 1;
    margin-left: 12px;
  }

  .change-btn {
    background: rgba(138, 43, 226, 0.2);
    border: 1px solid rgba(138, 43, 226, 0.4);
    color: #f8f9fa;
    border-radius: 6px;
    padding: 6px 12px;
    font-family: var(--font-body);
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .change-btn:hover {
    background: rgba(138, 43, 226, 0.4);
  }

  .metadata-grid {
    display: flex;
    flex-direction: column;
    margin-top: 8px;
  }

  .metadata-row {
    display: flex;
    align-items: center;
    height: 40px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .metadata-row:last-child {
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .row-label {
    width: 80px;
    font-family: var(--font-body);
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    flex-shrink: 0;
  }

  .row-value {
    font-family: var(--font-body);
    font-size: 14px;
    color: #ffffff;
    flex-grow: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    font-family: var(--font-body);
  }

  /* Custom Scrollbar */
  .scrollable-content::-webkit-scrollbar {
    width: 6px;
  }
  .scrollable-content::-webkit-scrollbar-track {
    background: transparent;
  }
  .scrollable-content::-webkit-scrollbar-thumb {
    background: var(--border-light);
    border-radius: 4px;
  }
  .scrollable-content::-webkit-scrollbar-thumb:hover {
    background: var(--primary);
  }
</style>
