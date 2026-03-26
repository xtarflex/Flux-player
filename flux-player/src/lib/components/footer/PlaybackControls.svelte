<script lang="ts">
  import Icon from '$lib/components/ui/Icon.svelte';
  let { 
    controlsEnabled, 
    shuffleState = $bindable(false), 
    repeatMode = $bindable(0),
    isPlaying = $bindable(false)
  } = $props<{
    controlsEnabled: boolean;
    shuffleState?: boolean;
    repeatMode?: number;
    isPlaying?: boolean;
  }>();

  function toggleRepeat() {
    repeatMode = (repeatMode + 1) % 3;
  }

  function togglePlay() {
    if (!controlsEnabled) return;
    isPlaying = !isPlaying;
  }
</script>

<div class="center-section">
  <div class="main-controls">
    <button 
      class="icon-btn shuffle {shuffleState ? 'active' : ''}" 
      class:disabled={!controlsEnabled}
      aria-label="Shuffle"
      disabled={!controlsEnabled}
      onclick={() => shuffleState = !shuffleState}
    >
      <Icon name="shuffle-2" class={shuffleState ? "is-active" : ""} />
    </button>

    <div class="control-pill">
      <button class="pill-btn" aria-label="Previous" disabled={!controlsEnabled}>
        <Icon name="skip-prev-fill" />
      </button>
      <div class="separator"></div>
      <button class="pill-btn seek-btn" aria-label="Seek Backward" disabled={!controlsEnabled}>
        <Icon name="skip-previous" />
      </button>
      <div class="separator"></div>
      <button class="pill-btn play-btn" aria-label={isPlaying ? 'Pause' : 'Play'} disabled={!controlsEnabled} onclick={togglePlay}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
          {#if isPlaying}
            <rect x="6" y="4" width="4" height="16" />
            <rect x="14" y="4" width="4" height="16" />
          {:else}
            <line x1="5" y1="6" x2="5" y2="18" />
            <path d="M10,7.5 V16.5 C10,18 11.5,19 12.5,18.2 L19,13.7 C20.2,12.9 20.2,11.1 19,10.3 L12.5,5.8 C11.5,5 10,6 10,7.5Z" />
          {/if}
        </svg>
      </button>
      <div class="separator"></div>
      <button class="pill-btn seek-btn" aria-label="Seek Forward" disabled={!controlsEnabled}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 11V9a4 4 0 0 0-4-4H3"/><polyline points="17 23 21 19 17 15"/><path d="M3 13v2a4 4 0 0 0 4 4h14"/></svg>
      </button>
      <div class="separator"></div>
      <button class="pill-btn" aria-label="Next" disabled={!controlsEnabled}>
        <Icon name="skip-next-fill" />
      </button>
    </div>

    <button 
      class="icon-btn repeat {repeatMode > 0 ? 'active' : ''}" 
      class:disabled={!controlsEnabled}
      aria-label="Repeat"
      disabled={!controlsEnabled}
      onclick={toggleRepeat}
    >
      {#if repeatMode === 2}
        <Icon name="repeat-1" />
      {:else}
        <Icon name="repeat" />
      {/if}
    </button>
  </div>
</div>

<style>
  .center-section {
    display: flex;
    justify-content: center;
    flex: 1;
  }

  .main-controls {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .control-pill {
    display: flex;
    align-items: center;
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-mid);
    border-radius: 99px;
    padding: 2px 8px;
  }

  .pill-btn {
    background: none;
    border: none;
    color: var(--text-main);
    padding: 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .pill-btn:not(:disabled):hover {
    color: var(--secondary);
    transform: scale(1.1);
  }

  .pill-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .play-btn {
    margin: 0 4px;
  }

  .play-btn svg { width: 28px; height: 28px; }
  .pill-btn svg { width: 20px; height: 20px; }

  .seek-btn {
    color: rgba(255, 255, 255, 0.5);
  }
  
  .seek-btn svg {
    width: 16px;
    height: 16px;
  }

  .separator {
    width: 1px;
    height: 20px;
    background: var(--glass-border-low);
    margin: 0 2px;
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    display: flex;
    align-items: center;
    transition: all 0.2s ease;
  }

  .icon-btn:not(:disabled):hover {
    color: var(--text-main);
  }

  .icon-btn:disabled,
  button.disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  :global(.icon-btn svg) { width: 20px; height: 20px; }

  .icon-btn.active {
    color: var(--secondary);
    opacity: 1;
  }

  :global(.shuffle svg), :global(.repeat svg) {
    width: 16px;
    height: 16px;
    transition: opacity 0.2s;
  }
  
  :global(.shuffle:not(.active) svg), :global(.repeat:not(.active) svg) {
    opacity: 0.6;
  }
</style>
