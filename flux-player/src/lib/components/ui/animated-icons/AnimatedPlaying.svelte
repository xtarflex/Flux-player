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
  style="overflow: visible;"
>
  <rect x="4" y="10" width="3" height="10" rx="1" class="bar bar1" />
  <rect x="9" y="6" width="3" height="14" rx="1" class="bar bar2" />
  <rect x="14" y="8" width="3" height="12" rx="1" class="bar bar3" />
  <rect x="19" y="11" width="3" height="9" rx="1" class="bar bar4" />
</svg>

<style>
  .bar {
    transform-origin: bottom;
    transform-box: fill-box;
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* Static states - a gentle wave curve */
  .animated-playing:not(.animating) .bar1 { transform: scaleY(0.4); }
  .animated-playing:not(.animating) .bar2 { transform: scaleY(0.6); }
  .animated-playing:not(.animating) .bar3 { transform: scaleY(0.5); }
  .animated-playing:not(.animating) .bar4 { transform: scaleY(0.3); }

  /* Animation on active/hover */
  .animating .bar1 { animation: pulse 0.7s ease-in-out infinite alternate; }
  .animating .bar2 { animation: pulse 0.9s ease-in-out infinite alternate 0.2s; }
  .animating .bar3 { animation: pulse 0.8s ease-in-out infinite alternate 0.1s; }
  .animating .bar4 { animation: pulse 1.0s ease-in-out infinite alternate 0.3s; }

  @keyframes pulse {
    0% { transform: scaleY(0.3); }
    100% { transform: scaleY(1.1); }
  }
</style>
