<script lang="ts">
  /**
   * @file MiniPlayerOverlay.svelte
   * @description UI overlay for the MiniPlayer, containing controls and metadata.
   */
  import { activeMedia, playbackState } from '$lib/stores/playback';

  interface Props {
    /** Callback for expanding the player back to full view */
    onExpand: (e: MouseEvent) => void;
    /** Callback for closing the player entirely */
    onClose: (e: MouseEvent) => void;
    /** Current playback state indicator */
    isPlaying: boolean;
  }
  let { onExpand, onClose, isPlaying }: Props = $props();

  const media = $derived($activeMedia);

  function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  }
</script>

<div class="mini-overlay">
  <div class="mini-header">
    <button class="mini-nav-btn" 
      onclick={onExpand} 
      onmousedown={(e) => e.stopPropagation()}
      title="Return to Player"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
        <polyline points="15 18 9 12 15 6"/>
      </svg>
      <span>Return</span>
    </button>
    
    <div class="mini-title-spacer">
      {#if media}
        <span class="mini-media-title">{media.title}</span>
      {/if}
    </div>

    <button class="mini-close-btn" 
      onclick={onClose} 
      onmousedown={(e) => e.stopPropagation()}
      title="Close Player"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="18" height="18">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>

  <div class="mini-center-actions">
    <button class="mini-play-btn" 
      onclick={togglePlay}
      onmousedown={(e) => e.stopPropagation()}
    >
      {#if isPlaying}
        <svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
          <rect x="6" y="4" width="4" height="16" rx="1"/>
          <rect x="14" y="4" width="4" height="16" rx="1"/>
        </svg>
      {:else}
        <svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32" style="margin-left: 4px;">
          <path d="M5 3l14 9-14 9V3z"/>
        </svg>
      {/if}
    </button>
  </div>
</div>

<style>
  .mini-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0,0,0,0);
    transition: background 0.3s ease;
    display: flex;
    flex-direction: column;
    opacity: 0;
    z-index: 10;
  }

  /* Host-dependent hover should be handled by the parent container or a wrapper */
  :global(.engine-container.is-mini:hover) .mini-overlay {
    background: rgba(0,0,0,0.5);
    opacity: 1;
  }

  .mini-header {
    height: 54px;
    display: flex;
    align-items: center;
    padding: 0 16px;
    background: linear-gradient(to bottom, rgba(0,0,0,0.8) 0%, transparent 100%);
    backdrop-filter: blur(8px);
  }

  .mini-nav-btn {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    color: #fff;
    padding: 6px 14px;
    border-radius: 20px;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .mini-nav-btn:hover {
    background: var(--secondary);
    border-color: var(--secondary);
    transform: translateX(-2px);
  }

  .mini-title-spacer {
    flex: 1;
    display: flex;
    justify-content: center;
    padding: 0 12px;
    overflow: hidden;
  }

  .mini-media-title {
    color: rgba(255,255,255,0.9);
    font-size: 0.8rem;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-family: var(--font-body);
    letter-spacing: 0.02em;
  }

  .mini-close-btn {
    background: transparent;
    border: none;
    color: rgba(255,255,255,0.6);
    padding: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mini-close-btn:hover {
    color: var(--primary);
    transform: rotate(90deg) scale(1.1);
  }

  .mini-center-actions {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mini-play-btn {
    width: 64px;
    height: 64px;
    border-radius: 50%;
    background: var(--glass-bg-high);
    border: 1px solid var(--glass-border-mid);
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    backdrop-filter: blur(12px);
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    box-shadow: 0 8px 32px rgba(0,0,0,0.4);
  }

  .mini-play-btn:hover {
    transform: scale(1.15);
    background: var(--secondary);
    border-color: var(--secondary);
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.2);
  }
</style>
