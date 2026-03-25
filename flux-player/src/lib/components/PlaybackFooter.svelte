<script lang="ts">
  import Scrubber from './footer/Scrubber.svelte';
  import MediaInfo from './footer/MediaInfo.svelte';
  import PlaybackControls from './footer/PlaybackControls.svelte';
  import QueueStack from './footer/QueueStack.svelte';
  import RightActions from './footer/RightActions.svelte';
  // Playback State
  let volume = $state(0.7);
  let progress = $state(0.35);
  let isPlaying = $state(false);
  
  // Media Info
  let currentMedia = $state<{
    type: 'video' | 'audio' | null;
    title: string;
    duration: string;
    currentTime: string;
    poster?: string;
  }>({
    type: null, 
    title: '',
    duration: '',
    currentTime: ''
  });
  
  // Control States
  let isLiked = $state(false);
  let shuffleState = $state(false);
  let repeatMode = $state(0); // 0: off, 1: all, 2: one
  let isMuted = $state(false);
  let playbackSpeed = $state(1);
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

    // 0-9: Jump to 0%-90%
    if (e.key >= '0' && e.key <= '9') {
      e.preventDefault();
      progress = parseInt(e.key) / 10;
      window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Jump to ${e.key}0%`, icon: 'playing' } }));
      return;
    }

    switch (e.key) {
      case ' ':
      case 'k':
      case 'K':
        e.preventDefault();
        isPlaying = !isPlaying;
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: isPlaying ? 'Play' : 'Pause', icon: isPlaying ? 'play' : 'pause' } }));
        break;
      case 'm':
      case 'M':
        e.preventDefault();
        isMuted = !isMuted;
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
        progress = Math.max(0, progress - 0.1);
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Rewind 10%', icon: 'seek-backward' } }));
        break;
      case 'l':
      case 'L':
        e.preventDefault();
        progress = Math.min(1, progress + 0.1);
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Forward 10%', icon: 'seek-forward' } }));
        break;
      case 'Home':
        e.preventDefault();
        progress = 0;
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Jump to Start', icon: 'skip-previous' } }));
        break;
      case 'End':
        e.preventDefault();
        progress = 1;
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Jump to End', icon: 'skip-next' } }));
        break;
      case 'ArrowUp':
        e.preventDefault();
        volume = Math.min(1, volume + 0.1);
        if (volume > 0) isMuted = false;
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume ${Math.round(volume * 100)}%`, icon: 'volume-up' } }));
        break;
      case 'ArrowDown':
        e.preventDefault();
        volume = Math.max(0, volume - 0.1);
        if (volume === 0) isMuted = true;
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: `Volume ${Math.round(volume * 100)}%`, icon: 'volume-down' } }));
        break;
      case 'ArrowRight':
        e.preventDefault();
        if (e.shiftKey) {
          nextTrack();
        } else {
          progress = Math.min(1, progress + 0.05);
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Seek Forward', icon: 'seek-forward' } }));
        }
        break;
      case 'ArrowLeft':
        e.preventDefault();
        if (e.shiftKey) {
          prevTrack();
        } else {
          progress = Math.max(0, progress - 0.05);
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Seek Backward', icon: 'seek-backward' } }));
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
  <Scrubber bind:progress disabled={!hasMedia} />

  <div class="footer-content">
    <MediaInfo {currentMedia} {hasMedia} bind:isLiked={isLiked} />
    <PlaybackControls {controlsEnabled} bind:shuffleState bind:repeatMode bind:isPlaying />
    <QueueStack {hasMedia} {queueHistory} {queue} {currentMedia} />
    <RightActions 
      {controlsEnabled} 
      bind:playbackSpeed 
      {showSubtitles} 
      {showPiP} 
      showVisualizer={isAudio}
      bind:isPiPActive 
      bind:isFullscreen 
      bind:volume 
      bind:isMuted 
    />
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
    padding-top: 4px;
  }
</style>