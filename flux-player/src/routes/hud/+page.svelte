<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import { getCurrentWindow, LogicalPosition, LogicalSize, primaryMonitor } from '@tauri-apps/api/window';
  import { fly, fade } from 'svelte/transition';
  import type { MediaItem } from '$lib/stores/playback';

  let currentMedia = $state<MediaItem | null>(null);
  let isVisible = $state(false);
  let hideTimer: ReturnType<typeof setTimeout> | null = null;

  async function showHUD(media: MediaItem) {
    const win = getCurrentWindow();
    
    // 1. Position HUD in bottom-right of primary monitor
    try {
      const monitor = await primaryMonitor();
      if (monitor) {
        const { width, height } = monitor.size;
        const scaleFactor = monitor.scaleFactor;
        // Logical HUD size is 380x100
        const x = (width / scaleFactor) - 400; // 20px padding
        const y = (height / scaleFactor) - 130; // 30px padding
        await win.setPosition(new LogicalPosition(x, y));
      }
    } catch (e) { console.error('HUD Position error:', e); }

    currentMedia = media;
    isVisible = true;

    await win.show();
    
    if (hideTimer) clearTimeout(hideTimer);
    hideTimer = setTimeout(async () => {
      isVisible = false;
      // Wait for exit animation
      setTimeout(() => win.hide(), 500);
    }, 4000);
  }

  let unlisten: (() => void) | null = null;

  onMount(() => {
    const win = getCurrentWindow();
    
    // Set click-through behavior so HUD doesn't block the user (Blueprint §8)
    win.setIgnoreCursorEvents(true).catch(e => console.error('Click-through error:', e));

    // Listen for global track changes emitted from the main window 
    // using an IIFE since onMount must be synchronous
    (async () => {
      unlisten = await listen<MediaItem>('flux-track-change', (event) => {
        showHUD(event.payload);
      });
    })();

    return () => {
      if (unlisten) unlisten();
      if (hideTimer) clearTimeout(hideTimer);
    };
  });
</script>

{#if isVisible && currentMedia}
  <main 
    class="hud-container" 
    in:fly={{ y: 20, duration: 400 }} 
    out:fade={{ duration: 300 }}
  >
    <!-- Layer 1: Blurred Background Image -->
    <div 
      class="blurred-bg" 
      style:background-image="url({resolveResource(currentMedia.album_art || currentMedia.poster)})"
    ></div>

    <!-- Layer 2: Slanted Gradient Overlay (Transparent left, Colored right) -->
    <div class="slanted-overlay"></div>

    <!-- Layer 3: Content -->
    <div class="content">
      <div class="metadata-section">
        <h1 class="title">{currentMedia.title}</h1>
        <p class="artist">{currentMedia.artist || 'Unknown Artist'}</p>
      </div>

      <div class="artwork-section">
        <img 
          src={resolveResource(currentMedia.album_art || currentMedia.poster)} 
          alt="Artwork" 
          class="sharp-artwork"
        />
      </div>
    </div>
  </main>
{/if}

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    overflow: hidden;
    background: transparent !important;
    font-family: 'Inter', sans-serif;
  }

  .hud-container {
    position: relative;
    width: 380px;
    height: 100px;
    border-radius: 16px;
    overflow: hidden;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    background: rgba(10, 10, 10, 0.2);
    display: flex;
  }

  .blurred-bg {
    position: absolute;
    inset: -20px;
    background-size: cover;
    background-position: center;
    filter: blur(40px) brightness(0.6);
    z-index: 0;
  }

  /* 
    The "Slant": 
    Transparent on the left, transitions to accent color on the right.
    Angle: 110deg for a sleek forward slant.
  */
  .slanted-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(
      110deg, 
      transparent 65%, 
      rgba(var(--accent-rgb, 138, 43, 226), 0.4) 65%,
      rgba(var(--accent-rgb, 138, 43, 226), 0.6) 100%
    );
    backdrop-filter: blur(10px);
    z-index: 1;
  }

  .content {
    position: relative;
    z-index: 2;
    display: flex;
    width: 100%;
    height: 100%;
    align-items: center;
    padding: 0 16px;
    justify-content: space-between;
  }

  .metadata-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 4px;
    padding-right: 20px;
    max-width: 240px;
  }

  .title {
    margin: 0;
    font-size: 16px;
    font-weight: 700;
    color: white;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-shadow: 0 2px 4px rgba(0,0,0,0.5);
  }

  .artist {
    margin: 0;
    font-size: 13px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.7);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .artwork-section {
    width: 80px;
    height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    margin-right: -4px; /* Tighten to the edge */
  }

  .sharp-artwork {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
</style>
