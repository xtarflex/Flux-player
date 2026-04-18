<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import { getCurrentWindow, LogicalPosition, LogicalSize, primaryMonitor, Window } from '@tauri-apps/api/window';
  import { fly, fade } from 'svelte/transition';
  import type { MediaItem } from '$lib/stores/playback';

  let currentMedia = $state<MediaItem | null>(null);
  let isVisible = $state(false);
  let hideTimer: ReturnType<typeof setTimeout> | null = null;
  let exitTimer: ReturnType<typeof setTimeout> | null = null;
  
  let imageError = $state(false);
  let backdropError = $state(false);
  let finalArt = $derived(imageError ? "/flux2d.png" : resolveResource(currentMedia?.album_art || currentMedia?.poster));
  let finalBackdrop = $derived(backdropError ? null : resolveResource(currentMedia?.album_art || currentMedia?.poster));

  function clearTimers() {
    if (hideTimer) clearTimeout(hideTimer);
    if (exitTimer) clearTimeout(exitTimer);
  }

  async function showHUD(media: MediaItem) {
    const win = getCurrentWindow();
    
    // 0. Only show HUD if main window is minimized (Blueprint §18)
    const mainWin = await Window.getByLabel('main');
    if (mainWin) {
      const minimized = await mainWin.isMinimized();
      if (!minimized) return;
    }

    // 1. Position HUD in bottom-right of primary monitor
    try {
      const monitor = await primaryMonitor();
      if (monitor) {
        const { width, height } = monitor.size;
        const scaleFactor = monitor.scaleFactor;
        const x = (width / scaleFactor) - 400; 
        const y = (height / scaleFactor) - 130; 
        await win.setPosition(new LogicalPosition(x, y));
      } else {
        // Fallback for multi-monitor or detection failure
        await win.setPosition(new LogicalPosition(100, 100));
      }
    } catch (e) { 
      console.error('HUD Position error:', e); 
      // Last resort fallback
      await win.setPosition(new LogicalPosition(0, 0));
    }

    // 2. State & Timer Management
    clearTimers();
    currentMedia = media;
    isVisible = true;

    // Reset error states for new media
    imageError = false;
    backdropError = false;

    // Use hardened Rust command to show and re-assert 'Always on Top' Z-order
    await invoke('show_hud');

    
    hideTimer = setTimeout(() => {
      isVisible = false;
      // 500ms allows the 'fade' out transition to complete before calling win.hide()
      exitTimer = setTimeout(() => {
        win.hide().catch(() => {});
      }, 500);
    }, 4000);
  }

  let unlisten: (() => void) | null = null;

  onMount(() => {
    const win = getCurrentWindow();
    
    // Set click-through behavior so HUD doesn't block the user (Blueprint §8)
    win.setIgnoreCursorEvents(true).catch(e => console.error('Click-through error:', e));

    (async () => {
      unlisten = await listen<MediaItem>('flux-track-change', (event) => {
        showHUD(event.payload);
      });
    })();

    return () => {
      if (unlisten) unlisten();
      clearTimers();
    };
  });
</script>

{#if isVisible && currentMedia}
  {#key currentMedia.id}
    <main 
      class="hud-container" 
      in:fly={{ y: 20, duration: 400 }} 
      out:fade={{ duration: 300 }}
    >
      <!-- Layer 1: Blurred Background Image -->
      <div 
        class="blurred-bg" 
        style={finalBackdrop ? `background-image: url('${finalBackdrop}')` : ''}
        onerror={() => backdropError = true}
      ></div>

      <!-- Layer 2: Slanted Gradient Overlay (Transparent left, Colored right) -->
      <div class="slanted-overlay"></div>

      <!-- Layer 3: Content -->
      <div class="content">
        <div class="metadata-section">
          <h1 class="title">{currentMedia.title}</h1>
          <p class="artist-album">
            {currentMedia.artist || 'Unknown Artist'} 
            {#if currentMedia.album}
              <span class="album-sep">•</span> {currentMedia.album}
            {/if}
          </p>
        </div>

        <div class="artwork-section">
          <img 
            src={finalArt} 
            alt="Artwork" 
            class="sharp-artwork"
            onerror={() => imageError = true}
          />
        </div>
      </div>
    </main>
  {/key}
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

  .artist-album {
    margin: 0;
    font-size: 13px;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.7);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .album-sep {
    opacity: 0.4;
    font-size: 10px;
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
