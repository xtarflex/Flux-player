<script lang="ts">
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
      <svg viewBox="0 0 24 24" fill="none" class:is-active={shuffleState} stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l4.1-6.1C11.6 9 12.8 8.4 14.1 8.4H22" />
        <path d="M18 5l4 3.4-4 3.4" />
        <path d="M2 8.4h1.4c1.3 0 2.5.6 3.3 1.7l1.2 1.8" />
        <path d="M11 14.3l1.2 1.8c.8 1.1 2 1.7 3.3 1.7H22" />
        <path d="M18 14.6l4 3.4-4 3.4" />
      </svg>
    </button>

    <div class="control-pill">
      <button class="pill-btn" aria-label="Previous" disabled={!controlsEnabled}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="5" y1="6" x2="5" y2="18" />
          <path d="M19,7.5 V16.5 C19,18 17.5,19 16.5,18.2 L10,13.7 C8.8,12.9 8.8,11.1 10,10.3 L16.5,5.8 C17.5,5 19,6 19,7.5Z" />
        </svg>
      </button>
      <div class="separator"></div>
      <button class="pill-btn seek-btn" aria-label="Seek Backward" disabled={!controlsEnabled}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 11V9a4 4 0 0 1 4-4h14"/><polyline points="7 23 3 19 7 15"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/></svg>
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
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M5,7.5 V16.5 C5,18 6.5,19 7.5,18.2 L14,13.7 C15.2,12.9 15.2,11.1 14,10.3 L7.5,5.8 C6.5,5 5,6 5,7.5Z" />
          <line x1="19" y1="6" x2="19" y2="18" />
        </svg>
      </button>
    </div>

    <button 
      class="icon-btn repeat {repeatMode > 0 ? 'active' : ''}" 
      class:disabled={!controlsEnabled}
      aria-label="Repeat"
      disabled={!controlsEnabled}
      onclick={toggleRepeat}
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M17 2l4 4-4 4"/>
        <path d="M3 11v-1a4 4 0 0 1 4-4h14"/>
        <path d="M7 22l-4-4 4-4"/>
        <path d="M21 13v1a4 4 0 0 1-4 4H3"/>
        {#if repeatMode === 2}
          <path d="M11 10h1v4"/>
        {/if}
      </svg>
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
    display: flex;
    align-items: center;
    justify-content: center;
  }

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

  .icon-btn svg { width: 20px; height: 20px; }

  .icon-btn.active {
    color: var(--secondary);
    opacity: 1;
  }

  .shuffle svg, .repeat svg {
    width: 16px;
    height: 16px;
    transition: opacity 0.2s;
  }
  
  .shuffle:not(.active) svg, .repeat:not(.active) svg {
    opacity: 0.6;
  }
</style>
