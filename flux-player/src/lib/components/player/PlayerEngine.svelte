<script lang="ts">
  /**
 * @file PlayerEngine.svelte
 * @description Headless Video.js engine for Flux Player.
 * Manages dual-instance pre-loading for gapless transitions (Blueprint §14).
 * Translates $playbackState store changes → Video.js API calls.
 * Translates Video.js events → store updates.
 * Handles the 90% watched rule and debounced progress saving via Tauri.
   */
  import { onMount, onDestroy } from 'svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
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



  /**
   * Binds a MediaItem to the given Video.js player instance.
   * Resolves the file path to the asset:// protocol for Tauri.
   * @param p - Video.js player instance.
   * @param item - The MediaItem to load.
   */
  function loadItemIntoPlayer(p: any, item: MediaItem) {
    const src = resolveResource(item.path);
    if (p.src() !== src) {
      p.src({ src, type: 'video/mp4' });
      const artSrc = item.album_art || item.poster;
      if (artSrc) {
        const resolvedArt = resolveResource(artSrc);
        p.poster(resolvedArt);
        extractDominantColor(resolvedArt);
      }
    }
    p.load();
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

      // ── One-Time Event Binding ───────────────────────────────────────────
      // We bind these once. They use get(activeMedia) to stay context-aware.
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

      player.on('play', () => playbackState.update(s => ({ ...s, isPlaying: true })));

      player.on('pause', () => {
        const item = get(activeMedia);
        if (get(playbackState).repeatMode === 2) return;
        if (item) saveNow(item.path, player.currentTime(), player.duration());
        playbackState.update(s => ({ ...s, isPlaying: false }));
      });

      player.on('ended', () => {
        const item = get(activeMedia);
        if (item) saveNow(item.path, 0, player.duration());
        if (get(playbackState).repeatMode === 2) return;
        playbackState.update(s => ({ ...s, isPlaying: false, progress: 0, isTheaterMode: false, isMiniPlayer: false }));
        activeMedia.set(null);
        goto('/library');
      });

      player.on('error', () => console.error('[PlayerEngine] Video.js error:', player.error()));

      // ── Initial State Sync ────────────────────────────────────────────────
      const media = get(activeMedia);
      const state = get(playbackState);

      if (media && media.type === 'video') {
        loadItemIntoPlayer(player, media);

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
          
          if (seekTarget && seekTarget > 0) player.currentTime(seekTarget);
          playbackState.update(s => ({ ...s, seekTo: null }));
          if (state.isPlaying) player.play().catch(() => {});
        });
      }

      // ── Subscriptions ───────────────────────────────────────────────────
      unsubState = playbackState.subscribe(st => {
        if (!player) return;
        if (st.isPlaying && player.paused()) player.play().catch(() => {});
        if (!st.isPlaying && !player.paused()) player.pause();
        player.volume(st.volume);
        player.muted(st.isMuted);
        player.playbackRate(st.speed);
        player.loop(st.repeatMode === 2);

        if (st.seekProgressRequest !== null && st.seekProgressRequest !== undefined) {
          const dur = player.duration() ?? 0;
          if (dur > 0) player.currentTime(st.seekProgressRequest * dur);
          playbackState.update(s => ({ ...s, seekProgressRequest: null }));
        }

        if (st.seekTo !== null && st.seekTo !== undefined) {
          player.currentTime(st.seekTo);
          playbackState.update(s => ({ ...s, seekTo: null }));
        }
      });

      unsubMedia = activeMedia.subscribe(item => {
        if (!player || !item || item.type !== 'video') return;
        loadItemIntoPlayer(player, item);
      });

    })();

    // Return a synchronous cleanup for Svelte
    return () => {
      unsubState?.();
      unsubMedia?.();
    };
  });

  onDestroy(() => {
    if (saveTimer) clearTimeout(saveTimer);
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

  /* Strip all default Video.js chrome — Flux provides its own premium controls */
  :global(.vjs-flux .vjs-control-bar),
  :global(.vjs-flux .vjs-big-play-button),
  :global(.vjs-flux .vjs-loading-spinner),
  :global(.vjs-flux .vjs-error-display),
  :global(.vjs-flux .vjs-modal-dialog),
  :global(.vjs-flux .vjs-text-track-settings),
  :global(.vjs-flux .vjs-hidden) {
    display: none !important;
  }

  /* Ensure the Video.js wrapper fills the container fully */
  :global(.video-js.vjs-flux) {
    width: 100% !important;
    height: 100% !important;
    display: block !important;
  }

  :global(.vjs-flux .vjs-tech) {
    display: block !important; /* Ensure the actual video remains visible */
    position: relative;
    width: 100% !important;
    height: 100% !important;
    object-fit: contain; /* Preserve aspect ratio within the 100% container */
  }

  /* Robust fix: Hide absolutely everything but the video element itself and captions */
  :global(.vjs-flux > *:not(.vjs-tech):not(.vjs-text-track-display)) {
    display: none !important;
  }
</style>
