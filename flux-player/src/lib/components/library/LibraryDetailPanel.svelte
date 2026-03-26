<script lang="ts">
  import Icon from '$lib/components/ui/Icon.svelte';
  /**
   * @typedef {Object} Props
   * @property {any} item - The media item to display details for
   * @property {() => void} onClose - Function to call when closing the panel
   */
  let { item, onClose } = $props<{
    item: {
      id: string;
      title: string;
      poster?: string;
      type: 'video' | 'audio' | 'unknown';
    };
    onClose: () => void;
  }>();

  // Mock metadata based on master guide
  let metadata = $derived({
    year: '2024',
    duration: '2h 15m',
    rating: '8.4',
    genres: ['Action', 'Sci-Fi', 'Thriller'],
    backdrop: item.poster // Uses current item poster as backdrop
  });
</script>

<div class="detail-panel glass-dark">
  <button class="close-btn" onclick={onClose} aria-label="Close Details">
    <Icon name="close" />
  </button>

  <div class="hero-section">
    <div class="backdrop-container">
      <img src={metadata.backdrop || '/placeholder.jpg'} alt="" class="backdrop-image" />
      <div class="backdrop-gradient"></div>
    </div>
    
    <div class="poster-overlay">
      {#if item.poster}
        <img src={item.poster} alt={item.title} class="poster-image" />
      {:else}
        <div class="poster-placeholder">
          <img src="/flux2d.png" alt="Flux" />
        </div>
      {/if}
    </div>
  </div>

  <div class="content-section">
    <div class="metadata-header">
      <h2 class="title">{item.title}</h2>
      <div class="info-row">
        <span>{metadata.year}</span>
        <span class="dot">•</span>
        <span>{metadata.duration}</span>
        <span class="dot">•</span>
        <span class="rating">★ {metadata.rating}</span>
      </div>
      
      <div class="genre-tags">
        {#each metadata.genres as genre}
          <span class="genre-tag">{genre}</span>
        {/each}
      </div>
    </div>

    <div class="actions">
      <button class="btn-play">Play Now</button>
      <button class="btn-secondary">Add to Queue</button>
    </div>

    <div class="description-section">
      <h3>Overview</h3>
      <p class="description">
        This is a placeholder description for {item.title}. Flux will automatically fetch real metadata from TMDB once the scanner is operational.
      </p>
    </div>
  </div>
</div>

<style>
  .detail-panel {
    width: 35%;
    min-width: 400px;
    max-width: 600px;
    height: 100%;
    border-left: 1px solid var(--glass-border-low);
    display: flex;
    flex-direction: column;
    position: relative;
    overflow-y: auto;
    z-index: 1000;
  }

  .close-btn {
    position: absolute;
    top: 16px;
    right: 16px;
    z-index: 10;
    background: rgba(0,0,0,0.5);
    border: none;
    color: white;
    padding: 8px;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: rgba(0,0,0,0.8);
    transform: rotate(90deg);
  }

  :global(.close-btn svg) { width: 20px; height: 20px; }

  /* Hero Section */
  .hero-section {
    position: relative;
    width: 100%;
    height: auto;
    aspect-ratio: 16 / 9;
    margin-bottom: 80px; /* Space for overhanging poster */
  }

  .backdrop-container {
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .backdrop-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    filter: brightness(0.7);
  }

  .backdrop-gradient {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 100%;
    background: linear-gradient(to bottom, rgba(0,0,0,0) 0%, rgba(26,26,31,0.9) 100%);
  }

  .poster-overlay {
    position: absolute;
    bottom: -80px;
    left: 24px;
    width: 180px;
    aspect-ratio: 2 / 3;
    border-radius: 12px;
    overflow: hidden;
    border: 2px solid rgba(0, 255, 255, 0.3);
    box-shadow: 0 12px 32px rgba(0,0,0,0.6);
    background: var(--bg-surface);
  }

  .poster-image {
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
  .poster-placeholder img { width: 50%; }

  /* Content Section */
  .content-section {
    padding: 0 24px 40px;
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .metadata-header {
    /* Offset to account for poster to the left */
    margin-left: 196px; 
    display: flex;
    flex-direction: column;
    gap: 12px;
    min-height: 80px;
  }

  .title {
    font-family: var(--font-heading);
    font-size: 20px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-main);
  }

  .info-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    color: var(--text-muted);
  }

  .dot { opacity: 0.5; }
  .rating { color: gold; font-weight: 600; }

  .genre-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 4px;
  }

  .genre-tag {
    background: rgba(0, 255, 255, 0.1);
    border: 1px solid rgba(0, 255, 255, 0.2);
    color: var(--secondary);
    font-size: 11px;
    font-weight: 600;
    padding: 4px 10px;
    border-radius: 20px;
    text-transform: uppercase;
  }

  /* Actions */
  .actions {
    display: flex;
    gap: 12px;
    margin-top: -8px;
  }

  .btn-play {
    flex: 1;
    background: var(--secondary);
    color: #000;
    border: none;
    padding: 12px;
    border-radius: 8px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-play:hover {
    filter: brightness(1.1);
    transform: translateY(-2px);
  }

  .btn-secondary {
    flex: 1;
    background: var(--glass-bg-mid);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 12px;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
  }

  .description-section h3 {
    font-size: 16px;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-bottom: 12px;
  }

  .description {
    font-size: 15px;
    line-height: 1.6;
    color: rgba(255,255,255,0.8);
  }
</style>
