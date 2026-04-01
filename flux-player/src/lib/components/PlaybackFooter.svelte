<script lang="ts">
  import Scrubber from './footer/Scrubber.svelte';
  import MediaInfo from './footer/MediaInfo.svelte';
  import PlaybackControls from './footer/PlaybackControls.svelte';
  import QueueStack from './footer/QueueStack.svelte';
  import RightActions from './footer/RightActions.svelte';
  import { activeMedia, playbackState } from '$lib/stores/playback';
  import { mediaItems, toggleFavorite } from '$lib/stores/media';

  
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
  let isPiPActive = $state(false);
  let isFullscreen = $state(false);

  // Queue management
  type QueueItem = { title: string; poster?: string };
  let queueHistory = $state<QueueItem[]>([]);
  let queue = $state<QueueItem[]>([]);
  let hasQueue = $derived(queue.length > 0 || queueHistory.length > 0);

  // Computed: Media-aware control visibility
  let hasMedia = $derived(currentMedia.type !== null);
  let isVideo = $derived(currentMedia.type === 'video');
  let isAudio = $derived(currentMedia.type === 'audio');
  
  // Show/hide controls based on media type
  let showPiP = $derived(hasMedia && isVideo);
  let showSubtitles = $derived(hasMedia && isVideo);
  
  // Enable/disable states
  let controlsEnabled = $derived(hasMedia);

  // Track Navigation Stubs
  function nextTrack() {
    if (!controlsEnabled) return;
    window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Next Track', icon: 'skip-next' } }));
    console.log('Action: Next Track');
  }

  function prevTrack() {
    if (!controlsEnabled) return;
    window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Previous Track', icon: 'skip-previous' } }));
    console.log('Action: Previous Track');
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
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: isMuted ? 'Muted' : 'Unmuted', icon: isMuted ? 'volume-mute' : 'volume-up' } }));
        break;
      case 'f':
      case 'F':
      case 'F11':
        if (isVideo) {
          e.preventDefault();
          isFullscreen = !isFullscreen;
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: isFullscreen ? 'Fullscreen' : 'Windowed', icon: 'fullscreen' } }));
        }
        break;
      case 'Escape':
        if (isFullscreen) {
          e.preventDefault();
          isFullscreen = false;
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
        e.preventDefault();
        playbackState.update(s => {
          const newVol = Math.min(1, s.volume + 0.1);
          return { ...s, volume: newVol, isMuted: newVol > 0 ? false : s.isMuted };
        });
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume Up`, icon: 'volume-up' } }));
        break;
      case 'ArrowDown':
        e.preventDefault();
        playbackState.update(s => {
          const newVol = Math.max(0, s.volume - 0.1);
          return { ...s, volume: newVol, isMuted: newVol === 0 ? true : s.isMuted };
        });
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume Down`, icon: 'volume-down' } }));
        break;
      case 'ArrowRight':
        e.preventDefault();
        if (e.shiftKey) {
          nextTrack();
        } else {
          playbackState.update(s => {
            const dur = $activeMedia?.duration || 0;
            const currentTime = dur * s.progress;
            return { ...s, seekTo: Math.min(dur, currentTime + 5) };
          });
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Forward 5s', icon: 'seek-forward' } }));
        }
        break;
      case 'ArrowLeft':
        e.preventDefault();
        if (e.shiftKey) {
          prevTrack();
        } else {
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

    <div class="footer-center-group">
      <PlaybackControls 
        {controlsEnabled} 
        shuffleState={$playbackState.shuffleState}
        repeatMode={$playbackState.repeatMode}
        onToggleShuffle={() => playbackState.update(s => ({ ...s, shuffleState: !s.shuffleState }))}
        onToggleRepeat={() => playbackState.update(s => ({ ...s, repeatMode: (s.repeatMode + 1) % 3 }))}
        onTogglePlay={() => playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }))}
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
      <QueueStack {hasMedia} {queueHistory} {queue} {currentMedia} />
      <RightActions 
        {controlsEnabled} 
        playbackSpeed={$playbackState.speed}
        onSpeedChange={(s: number) => playbackState.update(state => ({ ...state, speed: s }))} 
        {showSubtitles} 
        {showPiP} 
        showVisualizer={isAudio}
        bind:isPiPActive 
        bind:isFullscreen 
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