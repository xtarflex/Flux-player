<script lang="ts">
  /**
   * AnimatedDiscovery.svelte
   *
   * A dynamic radar or discovery lens icon for finding new media.
   * On hover, the "beam" or outer circle smoothly animates, creating
   * a searching/radar micro-interaction.
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
  class="animated-discovery {isAnimating ? 'animating' : ''}"
  style="overflow: visible;"
>
  <circle class="radar-dot" cx="12" cy="12" r="3" />
  <path class="radar-beam" d="M19 12a7 7 0 0 0-7-7 7 7 0 0 0-7 7" />
  <path class="radar-bottom" d="M19 12a7 7 0 0 1-7 7 7 7 0 0 1-7-7" />
  <path class="radar-tick" d="M12 2v3M12 19v3M22 12h-3M5 12H2" />
</svg>

<style>
  .radar-dot, .radar-beam, .radar-bottom, .radar-tick {
    transition: transform 0.4s ease, stroke-dashoffset 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* The top beam drawing animation */
  .radar-beam {
    stroke-dasharray: 22;
    stroke-dashoffset: 0;
  }

  /* The bottom beam drawing animation */
  .radar-bottom {
    stroke-dasharray: 22;
    stroke-dashoffset: 0;
  }

  .animated-discovery:not(.animating) .radar-beam {
    stroke-dashoffset: 22; /* Hidden when not active/hovered to look like a simpler icon */
  }

  .animated-discovery.animating .radar-beam {
    stroke-dashoffset: 0;
    transition: stroke-dashoffset 0.5s ease-out;
  }

  .animated-discovery.animating .radar-dot {
    transform: scale(1.3);
    transform-origin: center;
    transform-box: fill-box;
  }

  .animated-discovery.animating .radar-tick {
    transform: scale(0.9);
    transform-origin: center;
    transform-box: fill-box;
    opacity: 0.5;
  }
</style>
