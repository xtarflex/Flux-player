<script lang="ts">
  import { fade } from "svelte/transition";
  import { playbackState } from "$lib/stores/playback";

  let { mediaState } = $props<{ mediaState: string }>();

  function togglePlay() {
    playbackState.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  }

  function nextTrack() {
    window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Next Track', icon: 'skip-next' } }));
  }

  function prevTrack() {
    window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Previous Track', icon: 'skip-previous' } }));
  }
</script>

<div class="island-layer" transition:fade={{ duration: 200 }}>
  <div class="controls-content">
    <button class="control-btn" aria-label="Previous" onclick={prevTrack}>
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="feather"
      >
        <line x1="5" y1="6" x2="5" y2="18" stroke="var(--secondary)" />
        <path d="M19,7.5 V16.5 C19,18 17.5,19 16.5,18.2 L10,13.7 C8.8,12.9 8.8,11.1 10,10.3 L16.5,5.8 C17.5,5 19,6 19,7.5Z" stroke="var(--primary)" fill="none" />
      </svg>
    </button>

    <button class="control-btn main-btn" aria-label="Play/Pause" onclick={togglePlay}>
      {#if mediaState === "playing"}
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke-width="2.2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="feather"
        >
          <line x1="9" y1="6" x2="9" y2="18" stroke="var(--secondary)" />
          <line x1="15" y1="6" x2="15" y2="18" stroke="var(--primary)" />
        </svg>
      {:else}
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke-width="2.2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="feather"
        >
          <line x1="5" y1="6" x2="5" y2="18" stroke="var(--secondary)" />
          <path d="M10,7.5 V16.5 C10,18 11.5,19 12.5,18.2 L19,13.7 C20.2,12.9 20.2,11.1 19,10.3 L12.5,5.8 C11.5,5 10,6 10,7.5Z" stroke="var(--primary)" fill="none"/>
        </svg>
      {/if}
    </button>

    <button class="control-btn" aria-label="Next" onclick={nextTrack}>
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        class="feather"
      >
        <path d="M5,7.5 V16.5 C5,18 6.5,19 7.5,18.2 L14,13.7 C15.2,12.9 15.2,11.1 14,10.3 L7.5,5.8 C6.5,5 5,6 5,7.5Z" stroke="var(--primary)" fill="none"/>
        <line x1="19" y1="6" x2="19" y2="18" stroke="var(--secondary)" />
      </svg>
    </button>
  </div>
</div>

<style>
  .island-layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .controls-content {
    display: flex;
    align-items: center;
    gap: 24px;
    padding: 0 20px;
  }

  .control-btn {
    background: none;
    border: none;
    color: var(--text-white);
    cursor: pointer;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: transform 0.2s cubic-bezier(0.34, 1.56, 0.64, 1);
    padding: 0;
  }

  .control-btn:hover {
    transform: scale(1.2);
    color: var(--adaptive-tint);
  }

  .control-btn svg {
    width: 20px;
    height: 20px;
    display: block;
    fill: none;
  }

  .control-btn.main-btn svg {
    width: 28px;
    height: 28px;
  }
</style>
