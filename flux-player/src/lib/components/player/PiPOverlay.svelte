<script lang="ts">
  import { playbackState } from '$lib/stores/playback';
  import { exitPiP } from '$lib/utils/pip';

  let isPlaying = $derived($playbackState.isPlaying);

  async function handleReturnToFullApp(e: MouseEvent) {
    e.stopPropagation();
    await exitPiP();
    playbackState.update(s => ({ ...s, isPiP: false }));
  }

  function handleTogglePlay(e: MouseEvent) {
    e.stopPropagation();
    playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  }
</script>

<div class="pip-overlay" role="presentation" data-tauri-drag-region>
  <button
    class="action-btn return-btn"
    onclick={handleReturnToFullApp}
    aria-label="Return to full app"
  >
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M15 3h6v6" />
      <path d="M9 21H3v-6" />
      <path d="M21 3l-7 7" />
      <path d="M3 21l7-7" />
    </svg>
    <span>Return to Full App</span>
  </button>

  <button
    class="action-btn play-btn"
    onclick={handleTogglePlay}
    aria-label={isPlaying ? 'Pause' : 'Play'}
  >
    {#if isPlaying}
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="6" y="4" width="4" height="16" />
        <rect x="14" y="4" width="4" height="16" />
      </svg>
    {:else}
      <svg viewBox="0 0 24 24" fill="currentColor" stroke="none">
        <polygon points="5 3 19 12 5 21 5 3" />
      </svg>
    {/if}
  </button>
</div>

<style>
  .pip-overlay {
    position: absolute;
    inset: 0;
    z-index: 10001;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 24px;
    opacity: 0;
    visibility: hidden;
    transition: opacity 0.3s ease, visibility 0.3s ease;
    backdrop-filter: blur(4px);
  }

  /* Show overlay on hover */
  :global(.engine-container:hover) .pip-overlay {
    opacity: 1;
    visibility: visible;
  }

  .action-btn {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 50%;
    color: #fff;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .action-btn:hover {
    background: var(--primary);
    border-color: var(--primary);
    transform: scale(1.1);
  }

  .return-btn {
    width: auto;
    border-radius: 20px;
    padding: 10px 16px;
    gap: 8px;
    font-family: var(--font-body);
    font-size: 14px;
    font-weight: 500;
  }

  .return-btn svg {
    width: 20px;
    height: 20px;
  }

  .play-btn {
    width: 48px;
    height: 48px;
  }

  .play-btn svg {
    width: 24px;
    height: 24px;
  }
</style>
