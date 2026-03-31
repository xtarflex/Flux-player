<!--
  @file AudioEngine.svelte
  @description Persistent background audio engine for Flux Player.
  Lives in the root layout so audio continues across all route navigations.
  Uses the native HTMLAudioElement (no Video.js overhead for audio-only tracks).
  Driven entirely by the playbackState and activeMedia stores.
  No visible output — headless.
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { get } from 'svelte/store';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { activeMedia, playbackState, type MediaItem } from '$lib/stores/playback';

  let audioEl: HTMLAudioElement;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  /**
   * Debounced SQLite progress save — 3s after last timeupdate.
   * @param path - Media file DB key.
   * @param current - Current time in seconds.
   * @param duration - Total duration in seconds.
   */
  function scheduleSave(path: string, current: number, duration: number) {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
      try {
        console.log(`[AudioEngine] Scheduled save: ${path} @ ${Math.floor(current)}s`);
        await invoke('save_playback_progress', {
          path,
          seconds: Math.floor(current),
          duration: Math.floor(duration),
        });
        // Update local store so library UI progress bars stay in sync
        import('$lib/stores/media').then(m => m.updateLocalProgress(path, Math.floor(current)));
      } catch (e) {
        console.warn('[AudioEngine] save failed:', e);
      }
    }, 1000); // 1-second debounce (Fix 11.2)
  }

  /**
   * Immediate, non-debounced save for critical events (pause, end).
   */
  async function saveNow(path: string, current: number, duration: number) {
    if (saveTimer) {
      clearTimeout(saveTimer);
      saveTimer = null;
    }
    try {
      console.log(`[AudioEngine] Immediate save: ${path} @ ${Math.floor(current)}s`);
      await invoke('save_playback_progress', {
        path,
        seconds: Math.floor(current),
        duration: Math.floor(duration),
      });
      // Update local store so library UI progress bars stay in sync
      import('$lib/stores/media').then(m => m.updateLocalProgress(path, Math.floor(current)));
    } catch (e) {
      console.warn('[AudioEngine] Immediate save failed:', e);
    }
  }

  onMount(() => {
    /**
     * Synchronizes the OS "Now Playing" metadata and lockscreen controls.
     */
    async function updateMediaSession(item: MediaItem | null) {
      if (!item) {
        if ('mediaSession' in navigator) navigator.mediaSession.playbackState = 'none';
        return;
      }

      // 1. Emit HUD Event (for custom Slanted Desktop HUD)
      import('@tauri-apps/api/event').then(ev => {
        ev.emit('flux-track-change', item);
      });

      if (!('mediaSession' in navigator)) return;

      // 1. Update OS Metadata (Lockscreen/Taskbar)
      const artSrc = item.album_art || item.poster || '/flux2d.png';
      navigator.mediaSession.metadata = new MediaMetadata({
        title: item.title,
        artist: item.artist || 'Flux Player',
        album: item.album || '',
        artwork: [
          { src: convertFileSrc(artSrc), sizes: '512x512', type: 'image/png' }
        ]
      });

      // 2. Set up OS Action Handlers (Global Play/Pause/Skip)
      navigator.mediaSession.setActionHandler('play', () => playbackState.update(s => ({ ...s, isPlaying: true })));
      navigator.mediaSession.setActionHandler('pause', () => playbackState.update(s => ({ ...s, isPlaying: false })));
      navigator.mediaSession.setActionHandler('seekbackward', () => {
        playbackState.update(s => ({ ...s, seekProgressRequest: Math.max(0, s.progress - 0.05) }));
      });
      navigator.mediaSession.setActionHandler('seekforward', () => {
        playbackState.update(s => ({ ...s, seekProgressRequest: Math.min(1, s.progress + 0.05) }));
      });
      
      // These will be wired to the upcoming Queue Controller
      navigator.mediaSession.setActionHandler('previoustrack', () => {
        window.dispatchEvent(new CustomEvent('flux-prev-track'));
      });
      navigator.mediaSession.setActionHandler('nexttrack', () => {
        window.dispatchEvent(new CustomEvent('flux-next-track'));
      });
    }

    // ── Subscribe to activeMedia ─────────────────────────────────────────────
    const unsubMedia = activeMedia.subscribe(item => {
      // Update the OS lockscreen/thumbnail metadata
      updateMediaSession(item);

      // Only manage audio items here; video is handled by PlayerEngine in /playing
      if (!item || item.type !== 'audio') {
        audioEl?.pause();
        return;
      }
      const src = convertFileSrc(item.path);
      if (audioEl.src !== src) {
        audioEl.src = src;
        audioEl.load();

        // ── Resume logic (Fix 11.3: Fetch fresh DB value) ───────────────────
        (async () => {
          const state = get(playbackState);
          let seekTarget = state.seekTo || 0;
          let dbSavedPos = 0;

          if (seekTarget === 0 && item.path) {
            try {
              dbSavedPos = await invoke<number>('get_playback_progress', { path: item.path });
              if (dbSavedPos > 0) seekTarget = dbSavedPos;
            } catch (e) {
              console.warn('[AudioEngine] Could not fetch progress:', e);
            }
          }

          console.log(`[AudioEngine] Play initiated for: ${item.title}`);
          console.log(`[AudioEngine] DB saved position: ${dbSavedPos}s`);
          console.log(`[AudioEngine] Final seek target: ${seekTarget}s`);

          if (seekTarget > 0) {
            audioEl.currentTime = seekTarget;
            playbackState.update(s => ({ ...s, seekTo: null }));
          }

          if (get(playbackState).isPlaying) {
            audioEl.play().catch(() => {});
          }
        })();
      }
    });

    // ── Subscribe to playbackState ───────────────────────────────────────────
    const unsubState = playbackState.subscribe(state => {
      const media = get(activeMedia);

      // Sync OS playback state
      if ('mediaSession' in navigator) {
        navigator.mediaSession.playbackState = state.isPlaying ? 'playing' : 'paused';
      }

      // Update Window Title (Blueprint §2)
      import('@tauri-apps/api/window').then(v => {
        const title = media ? `${media.title} — ${media.artist || 'Flux'} | Flux` : 'Flux';
        v.getCurrentWindow().setTitle(title);
      });

      if (!media || media.type !== 'audio') return;

      if (state.isPlaying && audioEl.paused) {
        audioEl.play().catch(() => {});
      } else if (!state.isPlaying && !audioEl.paused) {
        audioEl.pause();
      }
      audioEl.volume = state.isMuted ? 0 : state.volume;
      audioEl.playbackRate = state.speed;

      // Handle live scrubbing/seeking
      if (state.seekProgressRequest !== null && state.seekProgressRequest !== undefined) {
        if (audioEl.duration > 0) {
          audioEl.currentTime = state.seekProgressRequest * audioEl.duration;
        }
        // Clear from store after successfully seeking
        playbackState.update(s => ({ ...s, seekProgressRequest: null }));
      }
    });

    // ── Native events → store sync ───────────────────────────────────────────
    function onTimeUpdate() {
      const media = get(activeMedia);
      if (!media || media.type !== 'audio') return;
      const d = audioEl.duration || 0;
      const t = audioEl.currentTime;
      if (d > 0) {
        playbackState.update(s => ({ ...s, progress: t / d }));
        scheduleSave(media.path, t, d);
      }
    }

    function onPlay() { playbackState.update(s => ({ ...s, isPlaying: true })); }
    function onPause() { 
      // If we are in Repeat One mode, ignore the native pause event triggered 
      // by the browser's loop transition. This prevents UI flickering.
      if (get(playbackState).repeatMode === 2) return;
      
      const media = get(activeMedia);
      if (media && media.type === 'audio') {
        saveNow(media.path, audioEl.currentTime, audioEl.duration);
      }

      playbackState.update(s => ({ ...s, isPlaying: false })); 
    }
    function onEnded() { 
      const media = get(activeMedia);
      if (media && media.type === 'audio') {
        saveNow(media.path, 0, audioEl.duration); // Reset to 0 for Smart Progress (Fix 11.2)
      }

      // If we are in Repeat One, the native 'loop' attribute handles the reset.
      // We ignore this event to maintain the 'Playing' state in the UI.
      if (get(playbackState).repeatMode === 2) return;

      playbackState.update(s => ({ ...s, isPlaying: false, progress: 0 })); 
      activeMedia.set(null);

      // Fix 9.1: If we are in the Now Playing view, reroute to library on completion
      if (get(page).url.pathname === '/playing') {
        goto('/library');
      }
    }

    audioEl.addEventListener('timeupdate', onTimeUpdate);
    audioEl.addEventListener('play', onPlay);
    audioEl.addEventListener('pause', onPause);
    audioEl.addEventListener('ended', onEnded);

    return () => {
      unsubMedia();
      unsubState();
      if (saveTimer) clearTimeout(saveTimer);
      audioEl.removeEventListener('timeupdate', onTimeUpdate);
      audioEl.removeEventListener('play', onPlay);
      audioEl.removeEventListener('pause', onPause);
      audioEl.removeEventListener('ended', onEnded);
    };
  });
</script>

<!-- svelte-ignore a11y_media_has_caption -->
<audio 
  bind:this={audioEl} 
  preload="auto" 
  style="display:none;" 
  loop={$playbackState.repeatMode === 2}
></audio>
