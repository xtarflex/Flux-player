<!--
  @file MiniPlayer.svelte
  @description In-app Mini-Player overlay (Blueprint Step 3).
  A 320×180px draggable video overlay that appears in the bottom-right of the app
  when the user navigates away from /playing while a video is active.
  The Video.js instance is shared via the $playerEngineRef store.
  Clicking the overlay routes back to /playing.
-->
<script lang="ts">
  import { goto } from '$app/navigation';
  import { activeMedia, playbackState, playerEngineRef, deactivateMiniPlayer } from '$lib/stores/playback';

  // ── Reactive ───────────────────────────────────────────────────────────────
  let media = $derived($activeMedia);
  let isVisible = $derived($playbackState.isMiniPlayer && !!media);
  let isPlaying = $derived($playbackState.isPlaying);

  // ── Drag state ─────────────────────────────────────────────────────────────
  let dragging = $state(false);
  let pos = $state({ x: 0, y: 0 }); // Offset from bottom-right anchor
  let dragStart = $state({ mx: 0, my: 0, px: 0, py: 0 });

  // ── Actions ────────────────────────────────────────────────────────────────

  /**
   * Returns to the full /playing view and deactivates the mini-player.
   */
  function expandToFull() {
    deactivateMiniPlayer();
    goto('/playing');
  }

  /**
   * Toggles play/pause via the shared Video.js engine reference.
   * @param e - Click event (stops propagation so the card click doesn't fire).
   */
  function togglePlay(e: MouseEvent) {
    e.stopPropagation();
    playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  }

  // ── Drag logic ─────────────────────────────────────────────────────────────
  function onDragStart(e: MouseEvent) {
    dragging = true;
    dragStart = { mx: e.clientX, my: e.clientY, px: pos.x, py: pos.y };
    window.addEventListener('mousemove', onDragMove);
    window.addEventListener('mouseup', onDragEnd);
  }

  function onDragMove(e: MouseEvent) {
    if (!dragging) return;
    pos = {
      x: dragStart.px + (dragStart.mx - e.clientX), // RTL: positive = move left
      y: dragStart.py + (dragStart.my - e.clientY), // RTB: positive = move up
    };
    // Clamp so it can't leave the visible window
    const maxX = window.innerWidth - 340;
    const maxY = window.innerHeight - 200;
    pos.x = Math.max(0, Math.min(pos.x, maxX));
    pos.y = Math.max(0, Math.min(pos.y, maxY));
  }

  function onDragEnd() {
    dragging = false;
    window.removeEventListener('mousemove', onDragMove);
    window.removeEventListener('mouseup', onDragEnd);
  }
</script>

{#if isVisible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="mini-player"
    style="right: {pos.x + 16}px; bottom: {pos.y + 108}px;"
    class:dragging
    onclick={expandToFull}
    onmousedown={onDragStart}
    role="button"
    tabindex="0"
    aria-label="Mini Player — click to expand"
  >
    <!-- Thumbnail (poster or album art) -->
    <div class="mini-thumb">
      {#if media?.poster}
        <img src={media.poster} alt={media?.title} />
      {:else}
        <div class="mini-thumb--fallback">
          <img src="/flux2d.png" alt="Flux" />
        </div>
      {/if}
    </div>

    <!-- Track info -->
    <div class="mini-info">
      <p class="mini-title">{media?.title ?? 'Playing'}</p>
      {#if media?.artist}
        <p class="mini-sub">{media.artist}</p>
      {/if}
    </div>

    <!-- Controls -->
    <div class="mini-controls">
      <button class="mini-btn" onclick={togglePlay} aria-label={isPlaying ? 'Pause' : 'Play'}>
        {#if isPlaying}
          <!-- Pause icon -->
          <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
            <rect x="6" y="4" width="4" height="16" rx="1"/>
            <rect x="14" y="4" width="4" height="16" rx="1"/>
          </svg>
        {:else}
          <!-- Play icon -->
          <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
            <path d="M8 5v14l11-7z"/>
          </svg>
        {/if}
      </button>

      <button
        class="mini-btn mini-btn--expand"
        onclick={expandToFull}
        aria-label="Expand to full player"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
          <polyline points="15 3 21 3 21 9"/>
          <polyline points="9 21 3 21 3 15"/>
          <line x1="21" y1="3" x2="14" y2="10"/>
          <line x1="3" y1="21" x2="10" y2="14"/>
        </svg>
      </button>
    </div>

    <!-- Adaptive tint border accent -->
    <div class="mini-accent-bar"></div>
  </div>
{/if}

<style>
  .mini-player {
    position: fixed;
    right: 16px;
    bottom: 108px; /* Above the footer */
    width: 320px;
    height: 80px;
    background: rgba(14, 14, 16, 0.92);
    backdrop-filter: blur(24px) saturate(180%);
    -webkit-backdrop-filter: blur(24px) saturate(180%);
    border: 1px solid var(--glass-border-mid);
    border-radius: 14px;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 14px;
    z-index: 9998;
    cursor: pointer;
    box-shadow:
      0 16px 48px rgba(0, 0, 0, 0.7),
      0 0 0 1px rgba(255,255,255,0.04);
    /* Slide-in animation */
    animation: mini-enter 0.35s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
    user-select: none;
    overflow: hidden;
  }

  .mini-player:hover {
    transform: translateY(-2px) scale(1.01);
    box-shadow: 0 24px 64px rgba(0,0,0,0.8), 0 0 0 1px rgba(255,255,255,0.08);
  }

  .mini-player.dragging {
    cursor: grabbing;
    transform: scale(1.02);
    transition: none;
  }

  @keyframes mini-enter {
    from { opacity: 0; transform: translateY(20px) scale(0.95); }
    to   { opacity: 1; transform: translateY(0) scale(1); }
  }

  /* ── Thumbnail ── */
  .mini-thumb {
    width: 56px;
    height: 56px;
    border-radius: 8px;
    overflow: hidden;
    flex-shrink: 0;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.08);
  }

  .mini-thumb img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .mini-thumb--fallback {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.3;
  }

  .mini-thumb--fallback img {
    width: 60%;
  }

  /* ── Info ── */
  .mini-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .mini-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin: 0;
  }

  .mini-sub {
    font-size: 11px;
    color: var(--text-muted);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* ── Controls ── */
  .mini-controls {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .mini-btn {
    background: rgba(255,255,255,0.07);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    width: 34px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: var(--text-main);
    transition: background 0.15s, border-color 0.15s;
  }

  .mini-btn:hover {
    background: rgba(138, 43, 226, 0.25);
    border-color: var(--primary);
  }

  .mini-btn--expand:hover {
    background: rgba(0, 255, 255, 0.15);
    border-color: var(--secondary);
  }

  /* ── Adaptive tint accent bar at the bottom ── */
  .mini-accent-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 2px;
    background: var(--island-adaptive-tint, var(--primary));
    opacity: 0.8;
    border-radius: 0 0 14px 14px;
  }
</style>
