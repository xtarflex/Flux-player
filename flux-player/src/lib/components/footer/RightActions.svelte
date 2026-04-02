<script lang="ts">
  import ContextMenu from '../ui/ContextMenu.svelte';
  import type { MenuItem } from '../ui/context-menu';
  import { activeMenu, openMenu } from '../../stores/ui';
  import { playbackState } from '../../stores/playback';

  let { 
    controlsEnabled, 
    playbackSpeed = 1, 
    onSpeedChange,
    showSubtitles, 
    showPiP,
    showVisualizer = false, 
    isPiPActive = false, 
    isFullscreen = false, 
    volume = $bindable(0.7), 
    isMuted = $bindable(false) 
  }: {
    controlsEnabled: boolean;
    playbackSpeed?: number;
    onSpeedChange?: (speed: number) => void;
    showSubtitles: boolean;
    showPiP: boolean;
    showVisualizer?: boolean;
    isPiPActive?: boolean;
    isFullscreen?: boolean;
    volume?: number;
    isMuted?: boolean;
  } = $props();

  function openSpeedMenu(e: MouseEvent) {
    if (!controlsEnabled) return;
    e.stopPropagation(); // Prevent global mousedown listener from closing it immediately
    const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
    openMenu(rect.left, rect.top, speedMenuItems);
  }

  const speeds = [0.25, 0.5, 0.75, 1, 1.25, 1.5, 2];
  let speedMenuItems = $derived(
    speeds.map(s => ({
      label: s === 1 ? 'Normal (1x)' : `${s}x`,
      action: () => onSpeedChange?.(s)
    } as MenuItem)).reverse() // Put fastest at the top
  );

  function togglePiP() {
    if (!showPiP) return;
    playbackState.update(s => ({ ...s, pipRequest: s.pipRequest === null ? true : !s.pipRequest }));
  }

  function toggleFullscreen() {
    if (!controlsEnabled) return;
    playbackState.update(s => ({ ...s, fullscreenRequest: s.fullscreenRequest === null ? true : !s.fullscreenRequest }));
  }

  function toggleMute() {
    if (!controlsEnabled) return;
    isMuted = !isMuted;
  }
  let isDraggingVolume = $state(false);

  function calculateVolume(e: PointerEvent, element: HTMLElement) {
    const rect = element.getBoundingClientRect();
    let newVolume = (e.clientX - rect.left) / rect.width;
    newVolume = Math.max(0, Math.min(1, newVolume));
    volume = newVolume;
    
    if (volume === 0) {
      isMuted = true;
    } else {
      isMuted = false;
    }
  }

  function onPointerDown(e: PointerEvent) {
    if (!controlsEnabled) return;
    isDraggingVolume = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    calculateVolume(e, e.currentTarget as HTMLElement);
  }

  function onPointerMove(e: PointerEvent) {
    if (!isDraggingVolume) return;
    calculateVolume(e, e.currentTarget as HTMLElement);
  }

  function onPointerUp(e: PointerEvent) {
    isDraggingVolume = false;
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
  }
</script>

<div class="right-section">
  <!-- Speed Control -->
  <button 
    class="speed-badge"
    class:active={playbackSpeed !== 1 || $activeMenu}
    class:disabled={!controlsEnabled}
    disabled={!controlsEnabled}
    onclick={openSpeedMenu}
    aria-label="Playback speed"
  >
    {playbackSpeed}x
  </button>

  <!-- Subtitles (Video Only) -->
  {#if showSubtitles}
    <button 
      class="icon-btn-large subtitles-btn" 
      aria-label="Subtitles"
      disabled={!controlsEnabled}
    >
      <svg viewBox="0 0 24 24" fill="none" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <rect x="2" y="6" width="20" height="12" rx="2" stroke="currentColor" />
        <path d="M9 10.5 A 1.5 1.5 0 0 0 6 12 A 1.5 1.5 0 0 0 9 13.5" stroke="currentColor" />
        <path d="M18 10.5 A 1.5 1.5 0 0 0 15 12 A 1.5 1.5 0 0 0 18 13.5" stroke="currentColor" />
      </svg>
    </button>
  {/if}

  <!-- PiP (Video Only) -->
  {#if showPiP}
    <button 
      class="icon-btn-large pip-btn" 
      class:active={isPiPActive}
      aria-label="Picture-in-Picture"
      onclick={togglePiP}
    >
      <svg viewBox="0 0 24 24" fill="none" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <rect x="2" y="4" width="20" height="16" rx="2" stroke="currentColor" />
        <rect x="11" y="11" width="9" height="7" rx="1" fill={isPiPActive ? "var(--secondary)" : "currentColor"} stroke="none" />
      </svg>
    </button>
  {/if}

  <!-- Visualizer (Audio Only) -->
  {#if showVisualizer}
    <button 
      class="icon-btn-large visualizer-btn" 
      aria-label="Visualizer"
      disabled={!controlsEnabled}
      onclick={() => console.log('Open Visualizer Options')}
    >
      <svg viewBox="0 0 24 24" fill="none" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 2v20M17 5v14M7 8v8M22 10v4M2 11v2" stroke="currentColor" />
      </svg>
    </button>
  {/if}

  <!-- Fullscreen -->
  <button 
    class="icon-btn-large fullscreen-btn" 
    class:active={isFullscreen}
    class:disabled={!controlsEnabled}
    aria-label="Fullscreen"
    disabled={!controlsEnabled}
    onclick={toggleFullscreen}
  >
    {#if isFullscreen}
      <!-- Exit Fullscreen -->
      <svg viewBox="0 0 24 24" fill="none" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M8 3v3a2 2 0 0 1-2 2H3" stroke="currentColor" />
        <path d="M21 8h-3a2 2 0 0 1-2-2V3" stroke="currentColor" />
        <path d="M3 16h3a2 2 0 0 1 2 2v3" stroke="currentColor" />
        <path d="M16 21v-3a2 2 0 0 1 2-2h3" stroke="currentColor" />
      </svg>
    {:else}
      <!-- Enter Fullscreen -->
      <svg viewBox="0 0 24 24" fill="none" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
        <path d="M15 3h6v6" stroke="currentColor" />
        <path d="M9 21H3v-6" stroke="currentColor" />
        <path d="M21 14v7h-7" stroke="currentColor" />
        <path d="M3 10V3h7" stroke="currentColor" />
      </svg>
    {/if}
  </button>

  <!-- Volume Bars with Sliding Logic -->
  <div class="volume-container">
    <div 
      class="volume-bars-wrapper"
      class:disabled={!controlsEnabled}
      onpointerdown={onPointerDown}
      onpointermove={onPointerMove}
      onpointerup={onPointerUp}
      onpointercancel={onPointerUp}
      ondblclick={toggleMute}
      aria-label="Volume Slider"
      role="slider"
      aria-valuemin="0"
      aria-valuemax="100"
      aria-valuenow={Math.round(volume * 100)}
      tabindex="0"
    >
      <div class="volume-bars">
        {#each Array(5) as _, i}
          <div 
            class="volume-bar" 
            class:active={!isMuted && volume >= (i + 1) / 5.5}
            class:muted={isMuted}
            style="height: {10 + i * 5}px"
            data-level={i + 1}
          ></div>
        {/each}
        {#if isMuted}
          <div class="mute-x">×</div>
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .right-section {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 16px;
    flex: 0 0 auto;
    min-width: 0;
  }

  .speed-badge {
    background: var(--glass-bg-mid);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    font-size: 11px;
    font-weight: 800;
    padding: 6px 10px;
    border-radius: 6px;
    text-transform: uppercase;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .speed-badge:not(.disabled):hover {
    border-color: var(--secondary);
  }

  .speed-badge.active {
    color: var(--secondary);
    border-color: var(--secondary);
  }

  .speed-badge.disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .icon-btn-large {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .icon-btn-large:not(:disabled):hover {
    color: var(--text-main);
    transform: scale(1.08);
  }

  .icon-btn-large:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    pointer-events: none;
  }

  .icon-btn-large svg {
    width: 32px;
    height: 32px;
  }

  .icon-btn-large.active {
    color: var(--secondary);
  }

  .volume-container {
    display: flex;
    align-items: center;
    margin-bottom: 12px;
  }

  .volume-bars-wrapper {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
  }

  .volume-bars-wrapper.disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .volume-bars {
    display: flex;
    align-items: flex-end;
    gap: 4px;
    height: 40px;
    width: 50px;
    position: relative;
  }

  .volume-bar {
    width: 6px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    transition: all 0.2s ease;
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-end;
  }

  .volume-bar.active {
    background: var(--text-main);
  }

  .volume-bar.active[data-level="1"] { background: #4caf50; }
  .volume-bar.active[data-level="2"] { background: #8bc34a; }
  .volume-bar.active[data-level="3"] { background: #ffeb3b; }
  .volume-bar.active[data-level="4"] { background: #ff9800; }
  .volume-bar.active[data-level="5"] { background: #f44336; }

  .volume-bar.muted {
    background: rgba(255, 255, 255, 0.2);
  }

  .mute-x {
    position: absolute;
    bottom: -6px;
    right: -4px;
    font-size: 20px;
    font-weight: 900;
    color: var(--primary);
    line-height: 1;
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.8);
    pointer-events: none;
    z-index: 2;
  }

  /* Responsive Display Logic */
  @media (max-width: 1000px) {
    .speed-badge, .subtitles-btn, .visualizer-btn {
      display: none !important;
    }
  }

  @media (max-width: 850px) {
    .pip-btn, .fullscreen-btn {
      display: none !important;
    }
    .right-section {
      gap: 8px;
    }
  }
</style>
