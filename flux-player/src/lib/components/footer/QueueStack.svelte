<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { resolveResource } from '$lib/utils/media';
  import type { MediaItem } from '$lib/stores/media';
  
  let { 
    hasMedia, 
    queueHistory = [], 
    queue = [], 
    currentMedia 
  } = $props<{
    hasMedia: boolean;
    queueHistory?: MediaItem[];
    queue?: MediaItem[];
    currentMedia: MediaItem | null;
  }>();

  let hasQueue = $derived(queue.length > 0 || queueHistory.length > 0);
</script>

<div class="queue-section">
  <button 
    class="card-stack-wrapper"
    class:has-queue={hasMedia}
    aria-label="Open Queue Panel"
    onclick={() => console.log('Opens Queue Panel')}
  >
    <div class="card-stack">
      {#if !hasMedia}
        <!-- Case 1: No media playing (Empty placeholders) -->
        <div class="queue-card side-card prev-card"></div>
        <div class="queue-card side-card next-card"></div>
        <div class="queue-card center-card"></div>
      {:else}
        <!-- Cases 2/3/4: Media Playing -->
        <div class="queue-card side-card prev-card">
          {#if queueHistory.length > 0}
            <img 
              src={resolveResource(queueHistory[queueHistory.length - 1].poster || queueHistory[queueHistory.length - 1].album_art || queueHistory[queueHistory.length - 1].backdrop)} 
              alt="Previous" 
              class="card-image" 
            />
          {/if}
        </div>

        <div class="queue-card side-card next-card">
          {#if queue.length > 0}
            <img 
              src={resolveResource(queue[0].poster || queue[0].album_art || queue[0].backdrop)} 
              alt="Next" 
              class="card-image" 
            />
          {/if}
        </div>

        <div class="queue-card center-card">
          {#if currentMedia}
            <img 
              src={resolveResource(currentMedia.poster || currentMedia.album_art || currentMedia.backdrop)} 
              alt="Current" 
              class="card-image" 
            />
          {/if}
        </div>
      {/if}
    </div>
  </button>
</div>

<style>
  .queue-section {
    width: 180px;
    display: flex;
    justify-content: center;
    flex-shrink: 0;
  }

  .card-stack {
    position: relative;
    width: 80px;
    height: 56px;
  }

  .queue-card {
    position: absolute;
    width: 48px;
    height: 56px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    border: 1px solid var(--glass-border-high);
    top: 0;
    overflow: hidden;
  }

  .center-card {
    left: 50%;
    transform: translateX(-50%);
    z-index: 3;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    background: linear-gradient(135deg, var(--bg-surface), #2a2a2f);
  }

  .side-card {
    z-index: 1;
    opacity: 0.4;
    transform: scale(0.85);
  }

  .prev-card { left: 0; }
  .next-card { right: 0; }

  .card-stack-wrapper {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    outline: none;
  }

  .card-stack-wrapper:hover {
    transform: translateY(-2px);
  }

  .card-stack-wrapper.has-queue:hover .center-card {
    border-color: var(--secondary);
  }

  .card-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 5px; 
    background: transparent;
    pointer-events: none;
  }
</style>
