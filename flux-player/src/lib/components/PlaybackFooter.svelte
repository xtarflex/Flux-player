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

  // Keyboard Shortcuts
  function handleKeydown(e: KeyboardEvent) {
    if (!controlsEnabled) return;

    // Ignore if user is typing in an input or textarea
    const activeEl = document.activeElement as HTMLElement;
    if (activeEl && (activeEl.tagName === 'INPUT' || activeEl.tagName === 'TEXTAREA' || activeEl.isContentEditable)) {
      return;
    }

    switch (e.key) {
      case ' ':
      case 'k':
        e.preventDefault();
        isPlaying = !isPlaying;
        break;
      case 'm':
      case 'M':
        e.preventDefault();
        isMuted = !isMuted;
        break;
      case 'f':
      case 'F':
        if (isVideo) {
          e.preventDefault();
          isFullscreen = !isFullscreen;
        }
        break;
      case 'ArrowUp':
        e.preventDefault();
        volume = Math.min(1, volume + 0.1);
        if (volume > 0) isMuted = false;
        break;
      case 'ArrowDown':
        e.preventDefault();
        volume = Math.max(0, volume - 0.1);
        if (volume === 0) isMuted = true;
        break;
      case 'ArrowRight':
        e.preventDefault();
        progress = Math.min(1, progress + 0.05);
        break;
      case 'ArrowLeft':
        e.preventDefault();
        progress = Math.max(0, progress - 0.05);
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