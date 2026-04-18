<script lang="ts">
  import { fade } from "svelte/transition";

  let { 
    type, 
    videoTitle, 
    videoTime,
    artistName = "",
    posterSrc = "/flux2d.png"
  } = $props<{ 
    type: "audio" | "playing";
    videoTitle?: string;
    videoTime?: string;
    artistName?: string;
    posterSrc?: string;
  }>();

  let imageError = $state(false);
  let finalSrc = $derived(imageError ? "/flux2d.png" : posterSrc);
</script>

<div class="island-layer" transition:fade={{ duration: 300 }}>
  {#if type === "audio"}
    <div class="audio-content">
      <div class="vinyl-container">
        <img 
          src={finalSrc} 
          alt="Album Art" 
          class="island-thumb vinyl-spin" 
          onerror={() => imageError = true}
        />
      </div>
      <div class="audio-info">
        <span class="track-title">{videoTitle ?? 'NOW PLAYING'}</span>
        <span class="artist-name">{artistName || 'FLUX PLAYER'}</span>
      </div>
      <div class="eq-container">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="eq-svg">
          <g class="eq-bar-1">
            <path d="M 4 10 A 2 2 0 0 1 8 10 V 12.5 C 6.5 12.5, 5.5 14.5, 4 14.5 Z" fill="var(--secondary)" />
            <path d="M 4 14.5 C 5.5 14.5, 6.5 12.5, 8 12.5 V 14 A 2 2 0 0 1 4 14 Z" fill="var(--primary)" />
          </g>
          <g class="eq-bar-2">
            <path d="M 10 8 A 2 2 0 0 1 14 8 V 12.5 C 12.5 12.5, 11.5 14.5, 10 14.5 Z" fill="var(--primary)" />
            <path d="M 10 14.5 C 11.5 14.5, 12.5 12.5, 14 12.5 V 16 A 2 2 0 0 1 10 16 Z" fill="var(--secondary)" />
          </g>
          <g class="eq-bar-3">
            <path d="M 16 10 A 2 2 0 0 1 20 10 V 11.5 C 18.5 11.5, 17.5 13.5, 16 13.5 Z" fill="var(--secondary)" />
            <path d="M 16 13.5 C 17.5 13.5, 18.5 11.5, 20 11.5 V 14 A 2 2 0 0 1 16 14 Z" fill="var(--primary)" />
          </g>
        </svg>
      </div>
    </div>
  {:else}
    <div class="video-playing-content">
      <div class="mini-poster">
        <img 
          src={finalSrc} 
          alt="Poster" 
          class="poster-thumb" 
          onerror={() => imageError = true}
        />
      </div>
      <div class="video-meta">
        <span class="video-title">{videoTitle}</span>
        <span class="video-time">{videoTime}</span>
      </div>
      <div class="eq-container">
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="eq-svg">
          <g class="eq-bar-1">
            <path d="M 4 10 A 2 2 0 0 1 8 10 V 12.5 C 6.5 12.5, 5.5 14.5, 4 14.5 Z" fill="var(--secondary)" />
            <path d="M 4 14.5 C 5.5 14.5, 6.5 12.5, 8 12.5 V 14 A 2 2 0 0 1 4 14 Z" fill="var(--primary)" />
          </g>
          <g class="eq-bar-2">
            <path d="M 10 8 A 2 2 0 0 1 14 8 V 12.5 C 12.5 12.5, 11.5 14.5, 10 14.5 Z" fill="var(--primary)" />
            <path d="M 10 14.5 C 11.5 14.5, 12.5 12.5, 14 12.5 V 16 A 2 2 0 0 1 10 16 Z" fill="var(--secondary)" />
          </g>
          <g class="eq-bar-3">
            <path d="M 16 10 A 2 2 0 0 1 20 10 V 11.5 C 18.5 11.5, 17.5 13.5, 16 13.5 Z" fill="var(--secondary)" />
            <path d="M 16 13.5 C 17.5 13.5, 18.5 11.5, 20 11.5 V 14 A 2 2 0 0 1 16 14 Z" fill="var(--primary)" />
          </g>
        </svg>
      </div>
    </div>
  {/if}
</div>

<style>
  .island-layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .audio-content, .video-playing-content {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 16px;
    width: 100%;
  }

  .video-playing-content {
    gap: 12px;
    padding: 0 16px;
    height: 100%;
  }

  .vinyl-container {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    overflow: hidden;
    position: relative;
    flex-shrink: 0;
    margin-left: -2px;
    box-shadow: 0 0 16px rgba(0, 0, 0, 0.6);
    background: #111;
  }

  .mini-poster {
    width: 40px;
    height: 40px;
    border-radius: 7px;
    overflow: hidden;
    flex-shrink: 0;
    border: 1px solid var(--glass-border-mid);
    background: var(--glass-bg-low);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .island-thumb, .poster-thumb {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  @keyframes island-spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .vinyl-spin {
    animation: island-spin 3s linear infinite;
  }

  .audio-info, .video-meta {
    display: flex;
    flex-direction: column;
    justify-content: center;
    flex: 1;
    min-width: 0;
  }

  .track-title, .video-title {
    font-family: var(--font-heading);
    font-size: 0.6rem;
    color: var(--text-white);
    letter-spacing: 0.05em;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .video-title { font-size: 0.7rem; }

  .artist-name, .video-time {
    font-size: 0.55rem;
    color: var(--text-muted);
  }

  .video-time {
    font-size: 0.6rem;
    font-variant-numeric: tabular-nums;
  }

  .eq-container {
    margin-left: auto;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .eq-svg {
    width: 100%;
    height: 100%;
  }

  @keyframes eq-bounce-1 {
    0%, 100% { transform: scaleY(1); }
    50% { transform: scaleY(1.3); }
  }
  @keyframes eq-bounce-2 {
    0%, 100% { transform: scaleY(1); }
    50% { transform: scaleY(0.6); }
  }
  @keyframes eq-bounce-3 {
    0%, 100% { transform: scaleY(1); }
    50% { transform: scaleY(1.2); }
  }

  .eq-bar-1 { animation: eq-bounce-1 0.6s infinite cubic-bezier(0.34, 1.56, 0.64, 1); transform-origin: center 12px; }
  .eq-bar-2 { animation: eq-bounce-2 0.6s infinite cubic-bezier(0.34, 1.56, 0.64, 1) 0.1s; transform-origin: center 12px; }
  .eq-bar-3 { animation: eq-bounce-3 0.6s infinite cubic-bezier(0.34, 1.56, 0.64, 1) 0.2s; transform-origin: center 12px; }
</style>
