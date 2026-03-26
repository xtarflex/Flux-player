<script lang="ts">
  /**
   * AnimatedLibrary.svelte
   *
   * A media library (bookshelf/folders) icon.
   * On hover or active, the folders "open" or lift up
   * smoothly, inviting the user into their collection.
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
  class="animated-library {isAnimating ? 'animating' : ''}"
>
  <!-- Folder/Book Spine 1 -->
  <path class="spine1" d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v2H6.5a.5.5 0 0 0 0 1H20" />

  <!-- Folder/Book Spine 2 -->
  <path class="spine2" d="M6.5 2h13.5v17a2 2 0 0 1-2 2H6.5a2.5 2.5 0 0 1-2.5-2.5V5" />
  <path class="page" d="M12 9h4" />
  <path class="page" d="M12 13h4" />
</svg>

<style>
  .spine1, .spine2, .page {
    transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1), stroke-dashoffset 0.6s ease;
  }

  .animated-library.animating .spine1 {
    transform: translateY(-2px);
  }

  .animated-library.animating .spine2 {
    transform: translateX(1px);
  }

  .animated-library.animating .page {
    stroke-dasharray: 4;
    stroke-dashoffset: 0;
    transition: stroke-dashoffset 0.4s ease-out 0.2s;
  }

  .animated-library:not(.animating) .page {
    stroke-dasharray: 4;
    stroke-dashoffset: 4; /* Hide lines when resting */
    transition: stroke-dashoffset 0.2s ease-in;
  }
</style>
