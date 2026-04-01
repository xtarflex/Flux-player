<!--
 * @file PlayerEngine.svelte
 * @description Headless Video.js engine for Flux Player.
 * Manages dual-instance pre-loading for gapless transitions (Blueprint §14).
 * Translates $playbackState store changes → Video.js API calls.
 * Translates Video.js events → store updates.
 * Handles the 90% watched rule and debounced progress saving via Tauri.
-->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { get } from 'svelte/store';
  import { goto } from '$app/navigation';
  import {
    activeMedia,
    playbackState,
    playerEngineRef,
    type MediaItem,
  } from '$lib/stores/playback';

  // ── Props ──────────────────────────────────────────────────────────────────
  interface Props {
    /** Called when Video.js has loaded and is ready to play */
    onReady?: () => void;
  }
  let { onReady }: Props = $props();

  // ── DOM refs ───────────────────────────────────────────────────────────────
  let videoEl: HTMLVideoElement;
  let containerEl: HTMLDivElement;

  // ── Video.js instances (dual for gapless transitions per Blueprint §14) ────
  let player: any = null;
  let nextPlayer: any = null; // Pre-loaded instance for next queue item

  // ── Debounce timer for progress saving ─────────────────────────────────────
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  // ── Inactivity timer for auto-hide (Blueprint §8) ──────────────────────────
  let idleTimer: ReturnType<typeof setTimeout> | null = null;

  // ── Adaptive tint extraction (Blueprint §1) ─────────────────────────────────
  /**
   * Extracts the dominant colour from an image using an off-screen 64×64 canvas.
   * Averages 1000 sampled pixels and writes the result to --island-adaptive-tint.
   * @param src - asset:// URL of the poster/album art.
   */
  function extractDominantColor(src: string) {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => {
      const canvas = document.createElement('canvas');
      canvas.width = 64;
      canvas.height = 64;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;
      ctx.drawImage(img, 0, 0, 64, 64);
      const data = ctx.getImageData(0, 0, 64, 64).data;
      let r = 0, g = 0, b = 0, count = 0;
      const total = data.length / 4;
      const step = Math.max(1, Math.floor(total / 1000));
      for (let i = 0; i < total; i += step) {
        r += data[i * 4];
        g += data[i * 4 + 1];
        b += data[i * 4 + 2];
        count++;
      }
      const tint = `rgb(${Math.round(r / count)}, ${Math.round(g / count)}, ${Math.round(b / count)})`;
      document.documentElement.style.setProperty('--island-adaptive-tint', tint);
    };
    img.src = src;
  }

  /**
   * Debounced save of the playback position to SQLite.
   * @param path - Media file path (DB primary key).
   * @param currentTime - Current playback time in seconds.
   * @param duration - Total media duration in seconds.
   */
  function scheduleSave(path: string, currentTime: number, duration: number) {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
      try {
        console.log(`[PlayerEngine] Scheduled save: ${path} @ ${Math.floor(currentTime)}s`);
        await invoke('save_playback_progress', {
          path,
          seconds: Math.floor(currentTime),
          duration: Math.floor(duration),
        });
        // Update local store so library UI progress bars stay in sync
        import('$lib/stores/media').then(m => m.updateLocalProgress(path, Math.floor(currentTime)));
      } catch (e) {
        console.warn('[PlayerEngine] Failed to save progress:', e);
      }
    }, 1000); // 1-second debounce (Fix 11.2)
  }

  /**
   * Immediate, non-debounced save for critical events (pause, exit, end).
   */
  async function saveNow(path: string, currentTime: number, duration: number) {
    if (saveTimer) {
      clearTimeout(saveTimer);
      saveTimer = null;
    }
    try {
      console.log(`[PlayerEngine] Immediate save: ${path} @ ${Math.floor(currentTime)}s`);
      await invoke('save_playback_progress', {
        path,
        seconds: Math.floor(currentTime),
        duration: Math.floor(duration),
      });
      // Update local store so library UI progress bars stay in sync
      import('$lib/stores/media').then(m => m.updateLocalProgress(path, Math.floor(currentTime)));
    } catch (e) {
      console.warn('[PlayerEngine] Immediate save failed:', e);
    }
  }

  /** Resets the idle timer for auto-hide UI (Blueprint §8). */
  function resetIdleTimer() {
    playbackState.update(s => ({ ...s, isIdle: false }));
    if (idleTimer) clearTimeout(idleTimer);
    // Only auto-hide during active video playback
    const state = get(playbackState);
    const media = get(activeMedia);
    if (state.isPlaying && media?.type !== 'audio') {
      idleTimer = setTimeout(() => { 
        playbackState.update(s => ({ ...s, isIdle: true }));
      }, 1000);
    }
  }

  /**
   * Binds a MediaItem to the given Video.js player instance.
   * Resolves the file path to the asset:// protocol for Tauri.
   * @param p - Video.js player instance.
   * @param item - The MediaItem to load.
   */
  function loadItemIntoPlayer(p: any, item: MediaItem) {
    const src = convertFileSrc(item.path);
    p.src({ src, type: 'video/mp4' }); // Video.js handles type detection
    p.load();
  }

  /**
   * Configures all Video.js event listeners for the current player.
   * @param p - Video.js player instance.
   * @param item - The active MediaItem.
   */
  function bindPlayerEvents(p: any, item: MediaItem) {
    // Sync progress to store
    p.on('timeupdate', () => {
      const t = p.currentTime() ?? 0;
      const d = p.duration() ?? 0;
      if (d > 0) {
        playbackState.update(s => ({ ...s, progress: t / d }));
        scheduleSave(item.path, t, d);
      }
      resetIdleTimer();
    });

    // Sync play/pause state
    p.on('play', () => playbackState.update(s => ({ ...s, isPlaying: true })));
    p.on('pause', () => {
      // If we are in Repeat One mode, ignore the native pause event triggered 
      // by the loop transition. This prevents UI flickering.
      if (get(playbackState).repeatMode === 2) return;
      saveNow(item.path, p.currentTime(), p.duration());
      playbackState.update(s => ({ ...s, isPlaying: false }));
    });

    // Handle end of media
    p.on('ended', () => {
      saveNow(item.path, 0, p.duration()); // Reset to 0 for Smart Progress (Fix 11.2)
      
      // If we are in Repeat One mode, the native loop handles the reset.
      // We ignore this event to maintain the 'Playing' state in the UI.
      if (get(playbackState).repeatMode === 2) return;

      playbackState.update(s => ({ ...s, isPlaying: false, progress: 0, isTheaterMode: false, isMiniPlayer: false }));
      activeMedia.set(null);
      goto('/library');
    });

    // Error handling
    p.on('error', () => {
      console.error('[PlayerEngine] Video.js error:', p.error());
    });
  }

  // ── Teardown refs (populated by the async setup IIFE) ────────────────────
  let unsubState: (() => void) | null = null;
  let unsubMedia: (() => void) | null = null;
  let removeMouseMove: (() => void) | null = null;

  onMount(() => {
    // Kick off async setup in an IIFE so onMount stays synchronous
    (async () => {
      // Lazy-import video.js only on mount to avoid SSR issues
      const videojs = (await import('video.js')).default;

      // Create the primary Video.js player (invisible controls — Flux has its own UI)
      player = videojs(videoEl, {
        controls: false,
        autoplay: false,
        preload: 'auto',
        fluid: false,
        responsive: false,
        fill: true,
        techOrder: ['html5'],
      });

      // Expose instance via store so MiniPlayer can share it
      playerEngineRef.set(player);
      onReady?.();

      // ── Load the active media if one is already set ──────────────────────
      const media = get(activeMedia);
      const state = get(playbackState);

      if (media) {
        loadItemIntoPlayer(player, media);
        bindPlayerEvents(player, media);

        // Extract adaptive tint from poster/album art
        const artSrc = media.album_art || media.poster;
        if (artSrc) extractDominantColor(convertFileSrc(artSrc));

        // Resume from last saved position or seekTo target
        player.ready(async () => {
          let seekTarget = state.seekTo;
          let dbSessionProgress = 0;

          if (seekTarget === null && media.path) {
            try {
              dbSessionProgress = await invoke<number>('get_playback_progress', { path: media.path });
              if (dbSessionProgress > 0) seekTarget = dbSessionProgress;
            } catch (e) {
              console.warn('[PlayerEngine] Could not fetch progress:', e);
            }
          }
          
          console.log(`[PlayerEngine] Play initiated for: ${media.title}`);
          console.log(`[PlayerEngine] DB saved position: ${dbSessionProgress}s`);
          console.log(`[PlayerEngine] Final seek target: ${seekTarget ?? 0}s`);

          if (seekTarget && seekTarget > 0) player.currentTime(seekTarget);
          // Clear seekTo after consuming it
          playbackState.update(s => ({ ...s, seekTo: null }));
          if (state.isPlaying) player.play().catch(() => {});
        });
      }

      // ── Subscribe: react to store-driven state changes ─────────────────
      unsubState = playbackState.subscribe(st => {
        if (!player) return;
        if (st.isPlaying && player.paused()) player.play().catch(() => {});
        if (!st.isPlaying && !player.paused()) player.pause();
        player.volume(st.volume);
        player.muted(st.isMuted);
        player.playbackRate(st.speed);
        player.loop(st.repeatMode === 2);

        // Handle live scrubbing/seeking from the footer UI
        if (st.seekProgressRequest !== null && st.seekProgressRequest !== undefined) {
          const dur = player.duration() ?? 0;
          if (dur > 0) {
            player.currentTime(st.seekProgressRequest * dur);
          }
          playbackState.update(s => ({ ...s, seekProgressRequest: null }));
        }
      });

      unsubMedia = activeMedia.subscribe(item => {
        if (!player || !item) return;
        loadItemIntoPlayer(player, item);
        bindPlayerEvents(player, item);
        const artSrc = item.album_art || item.poster;
        if (artSrc) extractDominantColor(convertFileSrc(artSrc));
      });

      // ── Mouse move → reset idle timer ───────────────────────────────────
      const handleMouseMove = () => resetIdleTimer();
      window.addEventListener('mousemove', handleMouseMove);
      removeMouseMove = () => window.removeEventListener('mousemove', handleMouseMove);
    })();

    // Return a synchronous cleanup for Svelte
    return () => {
      unsubState?.();
      unsubMedia?.();
      removeMouseMove?.();
    };
  });

  onDestroy(() => {
    if (saveTimer) clearTimeout(saveTimer);
    if (idleTimer) clearTimeout(idleTimer);
    if (player) { player.dispose(); player = null; }
    if (nextPlayer) { nextPlayer.dispose(); nextPlayer = null; }
    playerEngineRef.set(null);
    // Reset CSS tint to default
    document.documentElement.style.setProperty('--island-adaptive-tint', 'rgba(255,255,255,0.1)');
  });
</script>

<!-- Video element that Video.js will attach to -->
<div bind:this={containerEl} class="engine-container" class:is-idle={$playbackState.isIdle}>
  <!-- svelte-ignore a11y_media_has_caption -->
  <video bind:this={videoEl} class="video-js vjs-flux" playsinline></video>
</div>

<style>
  .engine-container {
    width: 100%;
    height: 100%;
    position: relative;
    background: #000;
  }

  .engine-container.is-idle {
    cursor: none;
  }

  /* Strip all default Video.js chrome — Flux provides its own controls */
  :global(.video-js.vjs-flux) {
    width: 100%;
    height: 100%;
    background: transparent;
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

  /* Hide absolutely everything but the video element itself and captions */
  :global(.vjs-flux > *:not(.vjs-tech):not(.vjs-text-track-display)) {
    display: none !important;
  }
</style>
