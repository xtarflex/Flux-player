<!--
  @file +page.svelte (/playing)
  @description Immersive Now Playing view for Flux.
  - Video mode: full-screen theater with PlayerEngine.svelte.
  - Audio (Vinyl) mode: rotating album art disc with canvas-based adaptive tinting.
  The Theater Mode CSS grid transformation is controlled by the $playbackState.isTheaterMode flag
  which the layout reads and applies .player-mode to .app-container.
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import { goto } from '$app/navigation';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import { activeMedia, playbackState } from '$lib/stores/playback';

  // ── Derived reactive state ──────────────────────────────────────────────────
  let media = $derived($activeMedia);
  let isVideo = $derived(media?.type === 'video' || media?.type === 'mixed');
  let isAudio = $derived(media?.type === 'audio');

  /** The album art URL resolved to asset:// for the vinyl disc */
  let albumArtSrc = $derived(resolveResource(media?.album_art || media?.poster));

  /** Vinyl rotation pauses when player is paused */
  let vinylSpinning = $derived($playbackState.isPlaying && isAudio);

  // Derive Tonearm angle: 0deg = Rest, 20deg = Outer Edge, 45deg = Inner Groove.
  let tonearmAngle = $derived.by(() => {
    if (!isAudio) return 0;
    // When paused at the very beginning or completely stopped, park it at 0.
    if (!$playbackState.isPlaying && $playbackState.progress <= 0.01) return 0;
    // Drop needle and track dynamically 20 to 45 degrees
    return 20 + ($playbackState.progress * 25);
  });

  onMount(() => {
    // Activate theater mode on route entry for both video and audio
    playbackState.update(s => ({ ...s, isTheaterMode: true }));
    return () => {
      // Do NOT deactivate theater mode here — the beforeNavigate handler in
      // layout.svelte will switch to mini-player mode instead of destroying it.
    };
  });
</script>

<svelte:head>
  <title>Now Playing — Flux</title>
  <meta name="description" content="Flux Player — Now Playing" />
</svelte:head>

<div class="playing-page">
  {#if !media}
    <!-- ── Empty State ─────────────────────────────────────────────────── -->
    <EmptyState
      variant="player"
      title="Ready for Input"
      description="The Flux player is in standby. Select a media file from your library to begin."
      actionLabel="Open Library"
      onAction={() => goto('/library')}
    />
  {:else if isVideo}
    <!-- ── Video Mode ──────────────────────────────────────────────────── -->
    <div class="video-wrapper">
      <!-- PlayerEngine is now globally managed in +layout.svelte -->
    </div>
  {:else if isAudio}
    <!-- ── Vinyl Mode ──────────────────────────────────────────────────── -->
    <div class="vinyl-stage">
      <!-- Ambient backdrop glow -->
      <div 
        class="vinyl-backdrop"
        style={albumArtSrc ? `background-image: url('${albumArtSrc}')` : ''}
      ></div>

      <!-- Turntable Deck (contains Disc & Tonearm) -->
      <div class="turntable">
        <!-- Rotating disc -->
        <div class="disc-wrapper" class:spinning={vinylSpinning}>
          <div class="disc">
            
            <div class="disc-rotator">
              <div class="disc-grooves"></div>
            <div class="disc-reflection"></div>
              {#if albumArtSrc}
                <img src={albumArtSrc} alt={media?.title} class="disc-art" />
              {:else}
                <div class="disc-art disc-art--placeholder">
                  <img src="/flux2d.png" alt="Flux" />
                </div>
              {/if}
            </div>

            <div class="disc-cap"></div>
          </div>
        </div>

        <!-- Animated Tonearm -->
        <div class="tonearm" style="transform: rotate({tonearmAngle}deg)">
          <div class="pivot-base"></div>
          <div class="pivot-cap"></div>
          <div class="arm-rod"></div>
          <div class="headshell">
            <div class="stylus"></div>
          </div>
        </div>
      </div>

      <!-- Track Info below the disc -->
      <div class="vinyl-info">
        <p class="vinyl-title">{media.title}</p>
        {#if media.artist}
          <p class="vinyl-artist">{media.artist}</p>
        {/if}
        {#if media.album}
          <p class="vinyl-album">{media.album}</p>
        {/if}
      </div>


    </div>
  {/if}
</div>

<style>
  .playing-page {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
    background: #000;
  }

  /* ── Video Mode ─────────────────────────────────────────────────────────── */
  .video-wrapper {
    position: absolute;
    inset: 0;
  }

  /* ── Vinyl Mode ─────────────────────────────────────────────────────────── */
  .vinyl-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 40px;
    width: 100%;
    height: 100%;
    position: relative;
    overflow: hidden;
  }

  /* Blurred cover art as ambient backdrop */
  .vinyl-backdrop {
    position: absolute;
    inset: -40px;
    background-size: cover;
    background-position: center;
    filter: blur(60px) brightness(0.25) saturate(2);
    z-index: 0;
    transition: background-image 0.8s ease;
  }

  /* ── Turntable & Disc ── */
  .turntable {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .disc-wrapper {
    position: relative;
    z-index: 1;
    width: min(55vw, 55vh, 480px);
    aspect-ratio: 1;
    filter: drop-shadow(0 0 60px var(--island-adaptive-tint, rgba(138,43,226,0.5)));
    transition: filter 1s ease;
  }

  .disc {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: #111;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 0 0 2px rgba(255,255,255,0.06),
      0 32px 80px rgba(0,0,0,0.8);
    overflow: hidden; /* Keeps gradients clean at the edges */
  }

  /* ── The Spinning Rotator ── */
  .disc-rotator {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    /* Hardware acceleration for buttery smooth 60fps rotation */
    will-change: transform; 
    animation: vinyl-spin 4s linear infinite;
    animation-play-state: paused; 
  }

  .disc-wrapper.spinning .disc-rotator {
    animation-play-state: running;
  }

/* Concentric groove rings */
  .disc-grooves {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    /* Try Widening the gap and soften the edges to prevent Moiré patterns */
    background: repeating-radial-gradient(
      circle at center,
      transparent 0,
      transparent 20px,
      rgba(255, 255, 255, 0.02) 21px,
      transparent 22px
    );
    pointer-events: none;
    opacity: 0.9;
    /* Optional: A subtle base noise can make it look like real pressed wax */
    filter: contrast(1.2); 
  }
  
  /* Static overhead gloss */
  .disc::after {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background: radial-gradient(circle at 30% 30%, rgba(255,255,255,0.08) 0%, transparent 50%);
    pointer-events: none;
    z-index: 5;
  }
  
  /* Anisotropic Reflection */
  .disc-reflection {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background: conic-gradient(
      from 0deg,
      transparent 0%,
      rgba(255, 255, 255, 0.12) 10%,
      transparent 20%,
      transparent 25%,
      rgba(255, 255, 255, 0.08) 35%,
      transparent 45%,
      transparent 50%,
      rgba(255, 255, 255, 0.12) 60%,
      transparent 70%,
      transparent 75%,
      rgba(255, 255, 255, 0.08) 85%,
      transparent 95%
    );
    z-index: 4; /* Moved up to sit over the album art slightly */
    pointer-events: none;
    mix-blend-mode: color-dodge; /* Makes the highlight interact nicely with album art colors */
    opacity: 0.6;
    mask-image: radial-gradient(circle, transparent 20%, black 25%);
    -webkit-mask-image: radial-gradient(circle, transparent 20%, black 25%);
  }

  /* Centre label / album art */
  .disc-art {
    width: 42%;
    height: 42%;
    border-radius: 50%;
    object-fit: cover;
    position: relative;
    z-index: 2;
    border: 3px solid rgba(255,255,255,0.08);
  }

  .disc-art--placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255,255,255,0.04);
  }

  .disc-art--placeholder img {
    width: 55%;
    opacity: 0.4;
  }

  /* Centre spindle cap (Static) */
  .disc-cap {
    position: absolute;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: radial-gradient(circle, #000000 0%, #333 100%);
    border: 2px solid rgba(255,255,255,0.15);
    z-index: 3;
  }

  /* ── Tonearm Assembly ── */
  .tonearm {
    position: absolute;
    top: -5%;
    right: -10%;
    height: 100%;
    z-index: 10;
    transform-origin: 22px 22px; /* Center of the pivot */
    transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1); /* Bouncy drop/lift */
    pointer-events: none; /* Ignore clicks */
    filter: drop-shadow(0 20px 30px rgba(0,0,0,0.8));
  }

  .pivot-base {
    position: absolute;
    top: 0;
    left: 0;
    width: 44px;
    height: 44px;
    border-radius: 50%;
    background: linear-gradient(135deg, #444, #1a1a1a);
    border: 2px solid rgba(255,255,255,0.1);
    box-shadow: inset 0 2px 4px rgba(255,255,255,0.2), 0 10px 20px rgba(0,0,0,0.6);
  }

  .pivot-cap {
    position: absolute;
    top: 12px;
    left: 12px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: radial-gradient(circle, #888, #222);
    border: 1px solid rgba(255,255,255,0.05);
  }

  .arm-rod {
    position: absolute;
    top: 22px;
    left: 20px;
    width: 8px;
    height: 60%; /* Reaches into the record */
    background: linear-gradient(to right, #666 0%, #aaa 30%, #444 100%);
    border-radius: 4px;
    box-shadow: inset 1px 0 2px rgba(255,255,255,0.4), -4px 0 10px rgba(0,0,0,0.5);
    /* Slight bend in the tonearm */
    transform-origin: top center;
  }

  .headshell {
    position: absolute;
    top: calc(60% + 20px);
    left: 10px;
    width: 28px;
    height: 45px;
    background: linear-gradient(135deg, #333, #111);
    border: 1px solid rgba(255,255,255,0.15);
    border-radius: 4px;
    transform-origin: top center;
    transform: rotate(25deg);
    box-shadow: inset 1px 1px 2px rgba(255,255,255,0.2), 0 8px 16px rgba(0,0,0,0.7);
  }

  .stylus {
    position: absolute;
    bottom: -4px;
    left: 12px;
    width: 4px;
    height: 10px;
    background: #ff3366; /* Vibrant cyberpunk accent color for the needle */
    border-radius: 2px;
    box-shadow: 0 0 10px #ff3366;
  }

  /* ── Vinyl track info ── */
  .vinyl-info {
    position: relative;
    z-index: 1;
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .vinyl-title {
    font-family: var(--font-heading);
    font-size: clamp(18px, 3vw, 32px);
    font-weight: 700;
    color: var(--text-main);
    letter-spacing: 0.04em;
    margin: 0;
  }

  .vinyl-artist {
    font-size: clamp(14px, 2vw, 18px);
    color: var(--secondary);
    margin: 0;
    font-weight: 600;
  }

  .vinyl-album {
    font-size: clamp(12px, 1.5vw, 15px);
    color: var(--text-muted);
    margin: 0;
  }

  /* ── Spin animation (Blueprint §3) ── */
  @keyframes vinyl-spin {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }
</style>
