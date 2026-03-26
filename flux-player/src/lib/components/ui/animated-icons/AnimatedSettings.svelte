<script lang="ts">
  /**
   * AnimatedSettings.svelte
   *
   * A cogwheel/settings icon.
   * On hover or active, the cog smoothly rotates, signifying
   * configuration and machinery.
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
  class="animated-settings {isAnimating ? 'animating' : ''}"
>
  <circle cx="12" cy="12" r="3" class="cog-inner" />
  <path class="cog-outer" d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
</svg>

<style>
  .cog-outer {
    transform-origin: 12px 12px;
    transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .cog-inner {
    transform-origin: 12px 12px;
    transition: transform 0.4s ease;
  }

  .animated-settings.animating .cog-outer {
    transform: rotate(90deg);
  }

  .animated-settings.animating .cog-inner {
    transform: scale(1.2);
  }
</style>
