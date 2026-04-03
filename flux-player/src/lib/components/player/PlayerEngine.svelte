<script lang="ts">
  /**
   * @file PlayerEngine.svelte
   * @description Headless Video.js engine for Flux Player.
   * Manages dual-instance pre-loading for gapless transitions (Blueprint §14).
   * Refactored for modularity: logic in player.ts, dragging in draggable.ts, UI in MiniPlayerOverlay.svelte.
   */
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { get } from 'svelte/store';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import MiniPlayerOverlay from './MiniPlayerOverlay.svelte';
  import { draggable } from '$lib/actions/draggable';
  import {
    savePlaybackProgress,
    loadItemIntoPlayer
  } from '$lib/utils/player';
  import {
    activeMedia,
    playbackState,
    playerEngineRef,
    deactivateMiniPlayer,
    type MediaItem,
  } from '$lib/stores/playback';
  import { nextTrack } from '$lib/stores/queue';

  // ── Props ──────────────────────────────────────────────────────────────────
  interface Props {
    /** Called when Video.js has loaded and is ready to play */
    onReady?: () => void;
  }
  let { onReady }: Props = $props();

  // ── DOM refs ───────────────────────────────────────────────────────────────
  let videoEl = $state<HTMLVideoElement>();
  let containerEl = $state<HTMLDivElement>();

  // ── Video.js instances ─────────────────────────────────────────────────────
  let player: any = null;
  let nextPlayer: any = null;

  // ── Animation & UI State ───────────────────────────────────────────────────
  let saveTimer = $state<ReturnType<typeof setTimeout> | null>(null);
  let isDragging = $state(false);
  let miniPos = $state({ 
    x: typeof window !== 'undefined' ? window.innerWidth - 384 - 16 : 0, 
    y: typeof window !== 'undefined' ? window.innerHeight - 216 - 112 : 0 
  });

  // ── Derived States ─────────────────────────────────────────────────────────
  const media = $derived($activeMedia);
  const isVideo = $derived(media?.type === 'video' || media?.type === 'mixed');
  const isMini = $derived($playbackState.isMiniPlayer);
  const isPlaying = $derived($playbackState.isPlaying);
  
  const onPlayingRoute = $derived(page.url.pathname === '/playing');
  const isVisible = $derived(isVideo && (onPlayingRoute || isMini));

  // ── Interaction Handlers ───────────────────────────────────────────────────
  function handleStageClick() {
    if (isMini) return;
    playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  }

  function handleStageDoubleClick() {
    if (isMini) return;
    playbackState.update(s => ({ 
      ...s, 
      fullscreenRequest: s.fullscreenRequest === null ? !s.isFullscreen : !s.fullscreenRequest 
    }));
  }

  function expandFromMini(e: MouseEvent) {
    e.stopPropagation();
    deactivateMiniPlayer();
    goto('/playing');
  }

  function closeMiniPlayer(e: MouseEvent) {
    e.stopPropagation();
    activeMedia.set(null);
    playbackState.update(s => ({ ...s, isPlaying: false, isMiniPlayer: false }));
  }

  // ── Persistence ────────────────────────────────────────────────────────────
  function scheduleSave(path: string, currentTime: number, duration: number) {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => savePlaybackProgress(path, currentTime, duration), 1000);
  }

  async function saveNow(path: string, currentTime: number, duration: number) {
    if (saveTimer) { clearTimeout(saveTimer); saveTimer = null; }
    await savePlaybackProgress(path, currentTime, duration);
  }

  // ── Lifecycle ──────────────────────────────────────────────────────────────
  let unsubState: (() => void) | null = null;
  let unsubMedia: (() => void) | null = null;

  function clampPosition(pos: { x: number, y: number }) {
    if (typeof window === 'undefined') return pos;
    const padding = 16;
    const footerHeight = 112; 
    
    // MiniPlayer size is 384x216
    const maxX = window.innerWidth - 384 - padding;
    const maxY = window.innerHeight - 216 - footerHeight;
    
    return {
      x: Math.min(Math.max(padding, pos.x), maxX),
      y: Math.min(Math.max(padding, pos.y), maxY)
    };
  }

  function handleResize() {
    if (isMini) {
      miniPos = clampPosition(miniPos);
    }
  }

  onMount(() => {
    window.addEventListener('resize', handleResize);
    (async () => {
      const videojs = (await import('video.js')).default;
      if (!videoEl) return;

      player = videojs(videoEl, {
        controls: false,
        autoplay: false,
        preload: 'auto',
        fill: true,
        techOrder: ['html5'],
      });

      playerEngineRef.set(player);
      onReady?.();

      // ── Event Binding ─────────────────────────────────────────────────────
      player.on('timeupdate', () => {
        const item = get(activeMedia);
        if (!item || item.type !== 'video') return;
        const t = player.currentTime() ?? 0;
        const d = player.duration() ?? 0;
        if (d > 0) {
          playbackState.update(s => ({ ...s, progress: t / d }));
          scheduleSave(item.path, t, d);
        }
      });

      player.on('play', () => {
        const item = get(activeMedia);
        if (item?.type === 'video' || item?.type === 'mixed') {
          playbackState.update(s => ({ ...s, isPlaying: true }));
        }
      });
      player.on('pause', () => {
        const item = get(activeMedia);
        const isVideoOwner = item?.type === 'video' || item?.type === 'mixed';
        if (get(playbackState).repeatMode !== 2 && item && isVideoOwner) {
          saveNow(item.path, player.currentTime(), player.duration());
        }
        // Only update global play state if we still own the media session
        if (isVideoOwner) {
          playbackState.update(s => ({ ...s, isPlaying: false }));
        }
      });

      player.on('ended', () => {
        const item = get(activeMedia);
        const isVideoOwner = item?.type === 'video' || item?.type === 'mixed';
        if (item && isVideoOwner) saveNow(item.path, 0, player.duration());
        // Milestone 2.2: Auto-advance to next track in queue
        if (get(playbackState).repeatMode === 1 || get(playbackState).repeatMode === 0) {
          nextTrack();
        } else if (get(playbackState).repeatMode === 2) {
          // Manual fallback if native loop fails
          player.currentTime(0);
          player.play().catch(() => {});
        }
      });

      player.on('fullscreenchange', () => playbackState.update(s => ({ ...s, isFullscreen: player.isFullscreen() })));

      if (videoEl) {
        videoEl.addEventListener('enterpictureinpicture', async () => {
          playbackState.update(s => ({ ...s, isPiP: true }));
          try { await getCurrentWindow().minimize(); } catch (e) {}
        });
        videoEl.addEventListener('leavepictureinpicture', async () => {
          playbackState.update(s => ({ ...s, isPiP: false }));
          setTimeout(async () => {
            try {
              const win = getCurrentWindow();
              await win.unminimize();
              await win.show();
              await win.setFocus();
            } catch (e) {}
          }, 150);
        });
      }

      // ── Initial Sync removed — handled by activeMedia subscription below ──

      // ── Store Listeners ───────────────────────────────────────────────────
      unsubState = playbackState.subscribe(st => {
        const media = get(activeMedia);
        if (!player || !media || media.type === 'audio') return;

        if (st.isPlaying && player.paused()) player.play().catch(() => {});
        if (!st.isPlaying && !player.paused()) player.pause();
        player.volume(st.volume);
        player.muted(st.isMuted);
        player.playbackRate(st.speed);
        player.loop(st.repeatMode === 2);

        if (st.seekProgressRequest != null) {
          const dur = player.duration() ?? 0;
          if (dur > 0) player.currentTime(st.seekProgressRequest * dur);
          playbackState.update(s => ({ ...s, seekProgressRequest: null }));
        }
        if (st.seekTo != null) {
          player.currentTime(st.seekTo);
          playbackState.update(s => ({ ...s, seekTo: null }));
        }
        if (st.fullscreenRequest != null) {
          if (st.fullscreenRequest) { if (!player.isFullscreen()) player.requestFullscreen(); }
          else { if (player.isFullscreen()) player.exitFullscreen(); }
          playbackState.update(s => ({ ...s, fullscreenRequest: null }));
        }
        if (st.pipRequest != null) {
          if (st.pipRequest && videoEl) videoEl.requestPictureInPicture().catch(() => {});
          else if (document.pictureInPictureElement) document.exitPictureInPicture().catch(() => {});
          playbackState.update(s => ({ ...s, pipRequest: null }));
        }
      });

      unsubMedia = activeMedia.subscribe(item => {
        if (!player) return;

        // If we switch to audio or null, stop the video player instance immediately (Fix 12.1)
        if (!item || item.type !== 'video') {
          if (!player.paused()) player.pause();
          return;
        }
        
        // 1. Resolve source and load into Video.js
        loadItemIntoPlayer(player, item);

        // 2. Resume logic (Fix 11.3: Wait for metadata to ensure seek works)
        player.one('loadedmetadata', async () => {
          const state = get(playbackState);
          let seekTarget = state.seekTo;

          // If no specific seek was requested by the UI (e.g. from DetailPanel), check DB
          if (seekTarget === null && item.path) {
            try {
              const progress = await invoke<number>('get_playback_progress', { path: item.path });
              if (progress > 0) seekTarget = progress;
            } catch (e) {
              console.warn('[PlayerEngine] Could not fetch progress:', e);
            }
          }

          if (seekTarget && seekTarget > 0) {
            console.log(`[PlayerEngine] Resuming ${item.title} to ${seekTarget}s`);
            player.currentTime(seekTarget);
          }

          // Clear seek target from store
          playbackState.update(s => ({ ...s, seekTo: null }));

          // Ensure play state is synchronized
          if (get(playbackState).isPlaying) {
            player.play().catch(() => {});
          }
        });
      });
    })();

    return () => { 
      unsubState?.(); 
      unsubMedia?.(); 
      window.removeEventListener('resize', handleResize);
    };
  });

  onDestroy(() => {
    if (saveTimer) clearTimeout(saveTimer);
    if (player) { player.dispose(); player = null; }
    if (nextPlayer) { nextPlayer.dispose(); nextPlayer = null; }
    playerEngineRef.set(null);
    document.documentElement.style.setProperty('--island-adaptive-tint', 'rgba(255,255,255,0.1)');
  });
</script>

<!-- 
  The engine-container is our main shell. When isMini=true, it uses the draggable action 
  to handle positioning and emits its draggable state.
-->
<div 
  bind:this={containerEl} 
  class="engine-container" 
  class:is-visible={isVisible}
  class:is-idle={$playbackState.isIdle}
  class:is-mini={isMini}
  class:is-dragging={isDragging}
  style={isMini ? `left: ${miniPos.x}px; top: ${miniPos.y}px;` : ''}
  use:draggable={{
    initialPos: miniPos,
    onDragStart: () => { if (isMini) isDragging = true; },
    onDragMove: (pos) => { if (isMini) miniPos = pos; },
    onDragEnd: () => { isDragging = false; }
  }}
  onclick={handleStageClick}
  ondblclick={handleStageDoubleClick}
  role="presentation"
>
  <!-- svelte-ignore a11y_media_has_caption -->
  <video bind:this={videoEl} class="video-js vjs-flux" playsinline></video>

  {#if isMini}
    <MiniPlayerOverlay 
      {isPlaying}
      onExpand={expandFromMini} 
      onClose={closeMiniPlayer} 
    />
  {/if}
</div>

<style>
  .engine-container {
    width: 100%;
    height: 100%;
    position: fixed;
    inset: 0;
    z-index: 0;
    background: #000;
    opacity: 0;
    visibility: hidden;
    pointer-events: none;
    transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1), 
                height 0.4s cubic-bezier(0.4, 0, 0.2, 1), 
                transform 0.4s cubic-bezier(0.4, 0, 0.2, 1),
                border-radius 0.4s ease,
                left 0.1s linear, 
                top 0.1s linear,
                opacity 0.3s ease;
  }

  .engine-container.is-visible {
    opacity: 1;
    visibility: visible;
    pointer-events: auto;
  }

  .engine-container.is-mini {
    width: 384px;
    height: 216px;
    z-index: 10000;
    border-radius: 20px;
    border: 1px solid rgba(255, 255, 255, 0.15);
    background: #000;
    box-shadow: 0 32px 80px rgba(0,0,0,0.9), 
                0 0 0 1px rgba(255,255,255,0.05);
    overflow: hidden;
    cursor: grab;
  }

  .engine-container.is-dragging {
    cursor: grabbing;
    transition: width 0.4s, height 0.4s, border-radius 0.4s;
  }

  .engine-container.is-idle:not(.is-mini) {
    cursor: none;
  }

  :global(.vjs-flux .vjs-control-bar),
  :global(.vjs-flux .vjs-big-play-button),
  :global(.vjs-flux .vjs-loading-spinner),
  :global(.vjs-flux .vjs-error-display),
  :global(.vjs-flux .vjs-modal-dialog),
  :global(.vjs-flux .vjs-text-track-settings),
  :global(.vjs-flux .vjs-hidden) {
    display: none !important;
  }

  :global(.video-js.vjs-flux) {
    width: 100% !important;
    height: 100% !important;
    display: block !important;
  }

  :global(.vjs-flux .vjs-tech) {
    display: block !important;
    position: relative;
    width: 100% !important;
    height: 100% !important;
    object-fit: contain;
  }
</style>
