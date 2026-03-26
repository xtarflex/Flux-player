<script lang="ts">
  /**
   * AnimatedPlaying.svelte
   *
   * An animated "Now Playing" icon featuring a vertical waveform.
   * On active or hover state, the waveform bars animate up and down
   * using CSS transforms to simulate playing audio/video media.
   *
   * Designed for the Sidebar navigation, adhering to the Cyber Dark theme
   * (crisp 2px strokes, no ambient glows).
   */
  let { active = false, hovered = false } = $props<{
    active?: boolean;
    hovered?: boolean;
  }>();

  let isAnimating = $derived(active || hovered);
</script>

<svg
  viewBox="0 0 24 24"
  width="20"
  height="20"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  class="animated-playing {isAnimating ? 'animating' : ''}"
>
  <rect x="5" y="10" width="3" height="4" class="bar bar1" />
  <rect x="10.5" y="6" width="3" height="12" class="bar bar2" />
  <rect x="16" y="8" width="3" height="8" class="bar bar3" />
</svg>

<style>
  .bar {
    transform-origin: bottom;
    transition: transform 0.2s ease;
  }

  /* Static states (non-playing) */
  .animated-playing:not(.animating) .bar1 { transform: scaleY(0.5); }
  .animated-playing:not(.animating) .bar2 { transform: scaleY(0.3); }
  .animated-playing:not(.animating) .bar3 { transform: scaleY(0.4); }

  /* Animation on active/hover */
  .animating .bar1 { animation: pulse 0.8s ease-in-out infinite alternate; }
  .animating .bar2 { animation: pulse 1s ease-in-out infinite alternate 0.3s; }
  .animating .bar3 { animation: pulse 0.9s ease-in-out infinite alternate 0.1s; }

  @keyframes pulse {
    0% { transform: scaleY(0.3); }
    100% { transform: scaleY(1.2); }
  }
</style>
