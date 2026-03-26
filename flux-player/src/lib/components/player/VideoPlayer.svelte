<script lang="ts">
  /**
   * @file VideoPlayer.svelte
   * @description Ultra-dark, premium Video.js wrapper for Flux Player.
   * Features: Automatic progress saving, custom play/resume logic, and glass overlays.
   */
  import { onMount, onDestroy } from 'svelte';
  import videojs from 'video.js';
  import type { Player } from 'video.js';
  import 'video.js/dist/video-js.css';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { playerStore, playerActions } from '$lib/stores/player';
  import { updatePlaybackProgress } from '$lib/stores/media';
  import Icon from '../ui/Icon.svelte';

  let videoElement: HTMLVideoElement;
  let player: Player;
  let saveInterval: any;

  // Track the store's active item
  let activeItem = $derived($playerStore.activeItem);
  let isPlaying = $derived($playerStore.isPlaying);

  onMount(() => {
    // Basic Video.js initialization
    player = videojs(videoElement, {
      autoplay: false,
      controls: false, // Custom controls to come later, but for 1.6 we'll show VJS controls
      responsive: true,
      fluid: true,
      playbackRates: [0.5, 1, 1.25, 1.5, 2],
      userActions: {
        hotkeys: true
      }
    }, () => {
      console.log('[FluxPlayer] Video.js is ready.');
    });

    // Subscriptions to sync internal VJS state to playerStore
    player.on('timeupdate', () => {
      playerActions.updateTime(player.currentTime() || 0);
    });

    player.on('durationchange', () => {
      playerActions.setDuration(player.duration() || 1);
    });

    player.on('play', () => {
      playerStore.update(s => ({ ...s, isPlaying: true }));
    });

    player.on('pause', () => {
      playerStore.update(s => ({ ...s, isPlaying: false }));
    });

    player.on('ended', () => {
      if (activeItem) {
        updatePlaybackProgress(activeItem.path, 0, true);
      }
      playerActions.stop();
    });

    // Debounced Progress Saving (every 5 seconds)
    saveInterval = setInterval(async () => {
      if (activeItem && player && !player.paused()) {
        const currentTime = player.currentTime() || 0;
        const duration = player.duration() || Infinity;
        const isWatched = (currentTime / duration) > 0.95; // 95% threshold for "Watched"
        
        await updatePlaybackProgress(activeItem.path, Math.floor(currentTime), isWatched);
      }
    }, 5000);
  });

  // Watch for activeItem changes to swap source
  $effect(() => {
    if (player && activeItem && activeItem.type === 'video') {
      const src = convertFileSrc(activeItem.path);
      player.src({ src, type: 'video/mp4' }); // TBD: more types or let VJS decide
      
      // Handle resuming
      if (activeItem.last_position > 5) {
        player.currentTime(activeItem.last_position);
      } else {
        player.currentTime(0);
      }
      
      if ($playerStore.isPlaying) {
        player.play().catch((e: any) => console.error('[FluxPlayer] Autoplay blocked or failed:', e));
      }
    }
  });

  // Watch for isPlaying store change to sync VJS
  $effect(() => {
    if (!player) return;
    if (isPlaying && player.paused()) {
      player.play().catch(() => {});
    } else if (!isPlaying && !player.paused()) {
      player.pause();
    }
  });

  onDestroy(() => {
    if (player) {
      player.dispose();
    }
    if (saveInterval) clearInterval(saveInterval);
  });

  function togglePlay() {
    playerActions.toggle();
  }

  function goBack() {
    playerActions.stop();
  }
</script>

<div class="flux-video-shell" class:hidden={!activeItem}>
  <div class="player-controls-overlay">
    <!-- Top-pinned Header -->
    <header class="player-header">
      <button class="back-btn" onclick={goBack}>
        <Icon name="grid-view" size={20} />
      </button>

      <div class="playing-info">
        <span class="label">NOW PLAYING</span>
        <h3 class="playing-title">{activeItem?.title}</h3>
      </div>
    </header>

    <!-- Center Play/Pause Overlay Toggle (Premium Feedback) -->
    <button class="center-play-area" onclick={togglePlay}>
      {#if !isPlaying}
        <div class="status-icon">
          <svg viewBox="0 0 24 24" fill="currentColor">
            <path d="M8 5v14l11-7z" />
          </svg>
        </div>
      {/if}
    </button>
  </div>

  <div class="vjs-container" data-vjs-player>
    <video bind:this={videoElement} class="video-js vjs-big-play-centered"></video>
  </div>
</div>

<style>
  .flux-video-shell {
    position: fixed;
    inset: 0;
    z-index: 10000;
    background: black;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .flux-video-shell.hidden {
    display: none;
  }

  .vjs-container {
    width: 100%;
    height: 100%;
    flex: 1;
    display: flex;
    align-items: center;
    background: #000;
  }

  /* Override VJS basic styles to align with brand */
  :global(.video-js) {
    width: 100%;
    height: 100%;
    background-color: transparent !important;
  }

  /* Hide default big play button - we build our own UI */
  :global(.vjs-big-play-button) {
    display: none !important;
  }

  .player-controls-overlay {
    position: absolute;
    inset: 0;
    z-index: 10;
    pointer-events: none;
    background: linear-gradient(to bottom, rgba(0,0,0,0.6) 0%, transparent 20%, transparent 80%, rgba(0,0,0,0.6) 100%);
    opacity: 0;
    transition: opacity 0.3s cubic-bezier(0.19, 1, 0.22, 1);
  }

  /* Reveal controls on mouse move within shell */
  .flux-video-shell:hover .player-controls-overlay {
    opacity: 1;
  }

  .player-header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    padding: 32px 48px;
    display: flex;
    align-items: center;
    gap: 24px;
    pointer-events: auto;
  }

  .back-btn {
    width: 48px;
    height: 48px;
    border-radius: 14px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    backdrop-filter: blur(12px);
    transition: all 0.2s;
  }

  .back-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: scale(1.05);
  }

  .playing-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.15em;
    color: var(--secondary);
    opacity: 0.8;
  }

  .playing-title {
    margin: 0;
    font-size: 20px;
    font-weight: 600;
    color: white;
    text-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  }

  /* Center Interaction Area */
  .center-play-area {
    position: absolute;
    inset: 100px 0; /* buffer for header/footer */
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
  }

  .status-icon {
    width: 80px;
    height: 80px;
    background: rgba(138, 43, 226, 0.2);
    border: 1px solid var(--primary);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    backdrop-filter: blur(12px);
    animation: zoomPulse 2s infinite ease-in-out;
  }

  .status-icon svg {
    width: 32px;
    height: 32px;
    margin-left: 4px; /* offset play triangle to center */
  }

  @keyframes zoomPulse {
    0%, 100% { transform: scale(1); opacity: 0.9; }
    50% { transform: scale(1.1); opacity: 1; box-shadow: 0 0 30px rgba(138, 43, 226, 0.4); }
  }
</style>
