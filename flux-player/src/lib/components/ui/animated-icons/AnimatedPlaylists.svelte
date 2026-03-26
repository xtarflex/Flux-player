<script lang="ts">
  /**
   * AnimatedPlaylists.svelte
   *
   * A layered media stack icon representing collections of media.
   * On hover or active, the layered cards gently separate or "reshuffle",
   * adding a premium micro-interaction feel.
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
  class="animated-playlists {isAnimating ? 'animating' : ''}"
>
  <!-- Background/Back Layer Card -->
  <path class="back-card" d="M16 6H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2z" />

  <!-- Foreground Layer Card / Music Note / Details -->
  <path class="front-card" d="M8 2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2" />
  <circle class="note-dot" cx="8" cy="14" r="2" />
  <path class="note-stem" d="M10 14V8l4-1" />
</svg>

<style>
  .back-card, .front-card, .note-dot, .note-stem {
    transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1), stroke-dashoffset 0.5s ease;
  }

  .animated-playlists.animating .front-card {
    transform: translate(1px, -1px);
  }

  .animated-playlists.animating .back-card {
    transform: translate(-1px, 1px);
  }

  .animated-playlists.animating .note-dot {
    transform: scale(1.1);
    transform-origin: 8px 14px;
  }

  .animated-playlists.animating .note-stem {
    transform: translateY(-1px);
  }
</style>
