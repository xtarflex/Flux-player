<script lang="ts">
  import Scrubber from './footer/Scrubber.svelte';
  import MediaInfo from './footer/MediaInfo.svelte';
  import PlaybackControls from './footer/PlaybackControls.svelte';
  import QueueStack from './footer/QueueStack.svelte';
  import RightActions from './footer/RightActions.svelte';
  import { activeMedia, playbackState } from '$lib/stores/playback';
  import { mediaItems, toggleFavorite, toggleWatched } from '$lib/stores/media';
  import { nextTrack as nextAction, prevTrack as prevAction, queue as queueStore, toggleShuffle } from '$lib/stores/queue';
  import { onMount } from 'svelte';
  import { onboarding, triggerTour } from '$lib/stores/onboarding';

  onMount(() => {
    // Automatically trigger player tour when media loads (if library tour is done)
    const unsub = activeMedia.subscribe(m => {
      if (m) {
        const state = $onboarding;
        if (!state.isActive && state.completedSections.includes('library') && !state.completedSections.includes('player')) {
          setTimeout(() => triggerTour('player'), 800);
        }
      }
    });
    return unsub;
  });

  
  // Media Info
  let currentMedia = $derived.by(() => {
    if (!$activeMedia) return { type: null, title: '', duration: '', currentTime: '' };
    
    const formatTime = (secs: number) => {
      const h = Math.floor(secs / 3600);
      const m = Math.floor((secs % 3600) / 60);
      const s = Math.floor(secs % 60);
      if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
      return `${m}:${s.toString().padStart(2, '0')}`;
    };

    const durationStr = $activeMedia.duration ? formatTime($activeMedia.duration) : '--:--';
    const currentStr = $activeMedia.duration ? formatTime($activeMedia.duration * $playbackState.progress) : '0:00';

    return {
      type: $activeMedia.type === 'mixed' ? 'video' : $activeMedia.type,
      title: $activeMedia.title,
      duration: durationStr,
      currentTime: currentStr,
      poster: $activeMedia.poster || $activeMedia.backdrop || undefined
    };
  });
  
  // Control States
  let isLiked = $derived($mediaItems.find(i => i.path === $activeMedia?.path)?.is_favorite ?? false);
  let isMuted = $state(false);

  // Queue management (derived from global store)
  let queueHistory = $derived($queueStore.items.slice(0, $queueStore.index));
  let queue = $derived($queueStore.items.slice($queueStore.index + 1));
  let hasQueue = $derived($queueStore.items.length > 0);

  // Computed: Media-aware control visibility
  let hasMedia = $derived(currentMedia.type !== null);
  let isVideo = $derived(currentMedia.type === 'video');
  let isAudio = $derived(currentMedia.type === 'audio');
  
  // Show/hide controls based on media type
  let showPiP = $derived(hasMedia && isVideo);
  let showSubtitles = $derived(hasMedia && isVideo);
  
  // Enable/disable states
  let controlsEnabled = $derived(hasMedia);

  // Track Navigation
  function nextTrack() {
    if (!controlsEnabled) return;
    nextAction();
  }
  
  function prevTrack() {
    if (!controlsEnabled) return;
    prevAction();
  }

  // Keyboard Shortcuts
  function handleKeydown(e: KeyboardEvent) {
    if (!controlsEnabled) return;

    // Ignore if user is typing in an input or textarea
    const activeEl = document.activeElement as HTMLElement;
    if (activeEl && (activeEl.tagName === 'INPUT' || activeEl.tagName === 'TEXTAREA' || activeEl.isContentEditable)) {
      return;
    }

    const key = e.key.toLowerCase();
    const hasModifier = e.ctrlKey || e.metaKey || e.altKey;

    // Ignore single-letter shortcuts if a modifier (Ctrl/Cmd/Alt) is held
    // to prevent conflicts with global navigation like Ctrl+L
    if (hasModifier && e.key.length === 1 && !['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(e.key)) {
      return;
    }

    // 0-9: Jump to 0%-90%
    if (e.key >= '0' && e.key <= '9') {
      e.preventDefault();
      playbackState.update(s => ({ ...s, progress: parseInt(e.key) / 10 }));
      window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Jump to ${e.key}0%`, icon: 'playing' } }));
      return;
    }

    switch (e.key) {
      case ' ':
      case 'k':
      case 'K':
        e.preventDefault();
        playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
        // No toast — the Dynamic Island morph is the feedback channel for audio
        break;
      case 'm':
      case 'M':
        e.preventDefault();
        playbackState.update(s => ({ ...s, isMuted: !s.isMuted }));
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: $playbackState.isMuted ? 'Muted' : 'Unmuted', icon: $playbackState.isMuted ? 'volume-mute' : 'volume-up' } }));
        break;
      case 's':
      case 'S':
        e.preventDefault();
        const nextShuffle = !$playbackState.shuffleState;
        playbackState.update(s => ({ ...s, shuffleState: nextShuffle }));
        toggleShuffle(nextShuffle);
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: nextShuffle ? 'Shuffle On' : 'Shuffle Off', icon: 'shuffle' } }));
        break;
      case 'w':
      case 'W':
        e.preventDefault();
        if ($activeMedia) {
          toggleWatched($activeMedia.path);
        }
        break;
      case 'r':
      case 'R':
        e.preventDefault();
        playbackState.update(s => {
          const nextRepeat = (s.repeatMode + 1) % 3;
          const labels = ['Repeat Off', 'Repeat All', 'Repeat One'];
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: labels[nextRepeat], icon: 'repeat' } }));
          return { ...s, repeatMode: nextRepeat };
        });
        break;
      case 'n':
      case 'N':
      case 'MediaTrackNext':
        e.preventDefault();
        nextTrack();
        break;
      case 'p':
      case 'P':
      case 'MediaTrackPrevious':
        e.preventDefault();
        prevTrack();
        break;
      case 'f':
      case 'F':
      case 'F11':
        if (isVideo) {
          e.preventDefault();
          playbackState.update(s => ({ ...s, fullscreenRequest: s.fullscreenRequest === null ? true : !s.fullscreenRequest }));
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: $playbackState.isFullscreen ? 'Windowed' : 'Fullscreen', icon: 'fullscreen' } }));
        }
        break;
      case 'Escape':
        if ($playbackState.isFullscreen) {
          e.preventDefault();
          playbackState.update(s => ({ ...s, fullscreenRequest: false }));
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Exited Fullscreen', icon: 'fullscreen' } }));
        }
        break;
      case 'j':
      case 'J':
        e.preventDefault();
        playbackState.update(s => {
          const dur = $activeMedia?.duration || 0;
          const currentTime = dur * s.progress;
          return { ...s, seekTo: Math.max(0, currentTime - 10) };
        });
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Rewind 10s', icon: 'seek-backward' } }));
        break;
      case 'l':
      case 'L':
        e.preventDefault();
        playbackState.update(s => {
          const dur = $activeMedia?.duration || 0;
          const currentTime = dur * s.progress;
          return { ...s, seekTo: Math.min(dur, currentTime + 10) };
        });
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Forward 10s', icon: 'seek-forward' } }));
        break;
      case 'Home':
        e.preventDefault();
        playbackState.update(s => ({ ...s, seekProgressRequest: 0 }));
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Jump to Start', icon: 'skip-previous' } }));
        break;
      case 'End':
        e.preventDefault();
        playbackState.update(s => ({ ...s, seekProgressRequest: 1 }));
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Jump to End', icon: 'skip-next' } }));
        break;
      case 'ArrowUp':
        if (hasModifier || !activeEl.closest('.media-card')) {
          e.preventDefault();
          playbackState.update(s => {
            const newVol = Math.min(1, s.volume + 0.1);
            return { ...s, volume: newVol, isMuted: newVol > 0 ? false : s.isMuted };
          });
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume Up`, icon: 'volume-up' } }));
        }
        break;
      case 'ArrowDown':
        if (hasModifier || !activeEl.closest('.media-card')) {
          e.preventDefault();
          playbackState.update(s => {
            const newVol = Math.max(0, s.volume - 0.1);
            return { ...s, volume: newVol, isMuted: newVol === 0 ? true : s.isMuted };
          });
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume Down`, icon: 'volume-down' } }));
        }
        break;
      case 'ArrowRight':
        if (e.shiftKey) {
          e.preventDefault();
          nextTrack();
        } else if (e.ctrlKey || e.metaKey || !activeEl.closest('.media-card')) {
          e.preventDefault();
          playbackState.update(s => {
            const dur = $activeMedia?.duration || 0;
            const currentTime = dur * s.progress;
            return { ...s, seekTo: Math.min(dur, currentTime + 5) };
          });
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Forward 5s', icon: 'seek-forward' } }));
        }
        break;
      case 'ArrowLeft':
        if (e.shiftKey) {
          e.preventDefault();
          prevTrack();
        } else if (e.ctrlKey || e.metaKey || !activeEl.closest('.media-card')) {
          e.preventDefault();
          playbackState.update(s => {
            const dur = $activeMedia?.duration || 0;
            const currentTime = dur * s.progress;
            return { ...s, seekTo: Math.max(0, currentTime - 5) };
          });
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Backward 5s', icon: 'seek-backward' } }));
        }
        break;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div 
  class="playback-footer" 
  role="contentinfo"
>
  <Scrubber 
    bind:progress={$playbackState.progress} 
    disabled={!hasMedia} 
    onSeek={(p) => playbackState.update(s => ({ ...s, seekProgressRequest: p }))} 
  />

  <div class="footer-content">
    <div class="footer-left-group">
      <MediaInfo 
        {currentMedia} 
        {hasMedia} 
        {isLiked} 
        onToggleLike={() => $activeMedia && toggleFavorite($activeMedia.path)} 
      />
    </div>

    <div id="onboard-player-controls" class="footer-center-group">
      <PlaybackControls 
        {controlsEnabled} 
        shuffleState={$playbackState.shuffleState}
        repeatMode={$playbackState.repeatMode}
        onToggleShuffle={() => {
          const newState = !$playbackState.shuffleState;
          playbackState.update(s => ({ ...s, shuffleState: newState }));
          toggleShuffle(newState);
        }}
        onToggleRepeat={() => playbackState.update(s => ({ ...s, repeatMode: (s.repeatMode + 1) % 3 }))}
        onTogglePlay={() => playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }))}
        onNext={nextTrack}
        onPrev={prevTrack}
        onSeekForward={() => {
          playbackState.update(s => {
            const dur = $activeMedia?.duration || 0;
            const currentTime = dur * s.progress;
            return { ...s, seekTo: Math.min(dur, currentTime + 5) };
          });
        }}
        onSeekBackward={() => {
          playbackState.update(s => {
            const dur = $activeMedia?.duration || 0;
            const currentTime = dur * s.progress;
            return { ...s, seekTo: Math.max(0, currentTime - 5) };
          });
        }}
        isPlaying={$playbackState.isPlaying} 
      />
    </div>

    <div class="footer-right-group">
      <QueueStack {hasMedia} {queueHistory} {queue} currentMedia={$activeMedia} />
      <RightActions 
        {controlsEnabled} 
        playbackSpeed={$playbackState.speed}
        onSpeedChange={(s: number) => playbackState.update(state => ({ ...state, speed: s }))} 
        {showSubtitles} 
        {showPiP} 
        showVisualizer={isAudio}
        isPiPActive={$playbackState.isPiP} 
        isFullscreen={$playbackState.isFullscreen} 
        bind:volume={$playbackState.volume} 
        bind:isMuted={$playbackState.isMuted} 
      />
    </div>
  </div>
</div>

<style>
  .playback-footer {
    grid-area: footer;
    height: var(--footer-height);
    width: 100%;
    background: var(--glass-bg-mid);
    backdrop-filter: blur(25px) saturate(160%);
    -webkit-backdrop-filter: blur(25px) saturate(160%);
    border-top: 1px solid var(--glass-border-low);
    z-index: 9997;
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 0 16px;
    font-family: var(--font-body);
  }

  .footer-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 100%;
    padding-top: 4px;
    gap: 16px;
  }

  .footer-left-group {
    flex: 1 1 0%;
    min-width: 0;
    display: flex;
    justify-content: flex-start;
  }

  .footer-center-group {
    flex: 0 0 auto;
    display: flex;
    justify-content: center;
    /* This ensures a true center even if left/right have different contents */
  }

  .footer-right-group {
    flex: 1 1 0%;
    min-width: 0;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 24px;
  }

  @media (max-width: 1000px) {
    .footer-right-group {
      gap: 12px;
    }
  }
</style>