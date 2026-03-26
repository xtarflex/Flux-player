<script lang="ts">
  /**
   * AnimatedPlayPause.svelte
   *
   * A premium morphological SVG animation that seamlessly transitions
   * between a "Play" triangle and a "Pause" double-bar using exact
   * point matching and CSS `path()` transitions.
   *
   * Usage: Provide an `isPlaying` boolean prop.
   */
  let {
    isPlaying = false,
    size = 24,
    class: className = "",
  } = $props<{
    isPlaying?: boolean;
    size?: number;
    class?: string;
  }>();
</script>

<svg
  viewBox="0 0 24 24"
  width={size}
  height={size}
  fill="currentColor"
  class="animated-playpause {className}"
  class:is-playing={isPlaying}
>
  <!--
    The magic relies on both states having the exact same number of points.
    We use two shapes that morph.

    Play state: two halves of a triangle joined together.
    Pause state: two vertical rectangular bars.
  -->
  <path class="shape left" />
  <path class="shape right" />
</svg>

<style>
  .shape {
    transition: d 0.4s cubic-bezier(0.85, 0, 0.15, 1);
  }

  /* PLAY STATE (Triangle split in half) */
  .animated-playpause:not(.is-playing) .left {
    /* Top-left point of triangle down to middle */
    d: path("M 5 5 L 12 9.5 L 12 14.5 L 5 19 Z");
  }
  .animated-playpause:not(.is-playing) .right {
    /* Middle of triangle out to the right point */
    d: path("M 12 9.5 L 19 12 L 19 12 L 12 14.5 Z");
  }

  /* PAUSE STATE (Two vertical bars) */
  .animated-playpause.is-playing .left {
    d: path("M 6 5 L 10 5 L 10 19 L 6 19 Z");
  }
  .animated-playpause.is-playing .right {
    d: path("M 14 5 L 18 5 L 18 19 L 14 19 Z");
  }
</style>
