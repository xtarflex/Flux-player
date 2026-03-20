<script lang="ts">
  import { onMount } from 'svelte';
  import { spring } from 'svelte/motion';
  import { fade } from 'svelte/transition';

  // State Management
  let currentState = $state('idle'); // idle, status, audio
  let currentLetter = $state(0);
  const letters = ['F', 'L', 'U', 'X'];

  // Spring Physics per skill_aesthetic_enforcer.md (Stiffness: 0.15, Damping: 0.35)
  const size = spring({ w: 56, h: 56 }, {
    stiffness: 0.15,
    damping: 0.35
  });

  // Dominant color for adaptive tinting (Static for now, will be dynamic later)
  let borderColor = $state('rgba(255, 255, 255, 0.1)'); 
  let isOffline = $state(false);

  // Handle Offline State
  $effect(() => {
    if (isOffline) {
      borderColor = '#ff0000';
    } else {
      borderColor = 'rgba(255, 255, 255, 0.1)';
    }
  });

  // Cycle letters in Idle state
  onMount(() => {
    const interval = setInterval(() => {
      if (currentState === 'idle') {
        currentLetter = (currentLetter + 1) % letters.length;
      }
    }, 4000); // 4 seconds per letter
    return () => clearInterval(interval);
  });

  // Morph values based on state
  $effect(() => {
    switch (currentState) {
      case 'idle': size.set({ w: 56, h: 56 }); break;
      case 'status': size.set({ w: 320, h: 48 }); break;
      case 'audio': size.set({ w: 240, h: 56 }); break;
    }
  });
</script>

<div 
  class="dynamic-island"
  style="width: {$size.w}px; height: {$size.h}px; border-color: {borderColor};"
  onmouseenter={() => currentState = 'status'}
  onmouseleave={() => currentState = 'idle'}
  role="button"
  tabindex="0"
>
  <!-- Idle Content Layer -->
  {#if currentState === 'idle'}
    <div class="island-layer" transition:fade={{ duration: 300 }}>
      <div class="letter-box">
        {#if currentLetter === 0}
          <svg viewBox="0 0 1024 1024" class="brand-svg">
            <path fill="var(--secondary)" d="M705.94 119.87c17.46 17.47 28.3 41.56 28.3 68.06s-10.84 50.59-28.3 68.04c-17.47 17.47-41.55 28.31-68.06 28.31-77.92 0-142.44 57.34-153.68 132.12-1.13 7.6-1.73 15.37-1.73 23.29v-58.17c0-71.85 21.77-138.61 59.07-194.05a349.612 349.612 0 0 1 96.33-95.89h0.01c26.51-0.01 50.59 10.84 68.06 28.29z"/>
            <path fill="var(--primary)" d="M734.24 556.77c0 1.67-0.04 3.33-0.12 4.97-1.27 24.54-11.81 46.71-28.17 63.08-17.47 17.46-41.55 28.3-68.06 28.3H482.47v182.95c0 52.99-43.35 96.35-96.35 96.35s-96.35-43.37-96.35-96.35V460.42h0.61c-0.41-6.85-0.61-13.76-0.61-20.73 0-136.41 78.45-254.49 192.71-311.59 46.77-23.37 99.55-36.53 155.4-36.53-38.04 25.34-70.83 57.96-96.33 95.89-37.3 55.44-59.07 122.2-59.07 194.05v78.9h155.41c52.99 0.01 96.35 43.36 96.35 96.36z"/>
          </svg>
        {:else if currentLetter === 1}
          <svg viewBox="0 0 1024 1024" class="brand-svg">
            <path fill="var(--secondary)" d="M415.65 673.55c0 53 43.35 96.35 96.35 96.35 26.49 0 50.59 10.84 68.04 28.31 17.47 17.47 28.31 41.55 28.31 68.04 0 26.51-10.84 50.59-28.31 68.06-17.46 17.46-41.55 28.3-68.04 28.3-53 0-96.35-43.35-96.35-96.35V673.55z"/>
            <path fill="var(--primary)" d="M608.35 157.74v708.52c0-26.49-10.84-50.58-28.31-68.04-17.46-17.47-41.55-28.31-68.04-28.31-53 0-96.35-43.35-96.35-96.35V157.74c0-52.99 43.35-96.35 96.35-96.35 26.49 0 50.59 10.84 68.04 28.31 17.47 17.46 28.31 41.55 28.31 68.04z"/>
          </svg>
        {:else if currentLetter === 2}
          <svg viewBox="0 0 1024 1024" class="brand-svg">
            <path fill="var(--secondary)" d="M356.6 514.01c0.25 45.32 9.16 88.58 25.16 128.22 39.77 98.54 123.35 174.71 226.59 204.37-30.6 8.8-62.92 13.51-96.35 13.51-55.85 0-108.64-13.15-155.41-36.53C242.33 766.49 163.88 648.41 163.88 512V260.24c0-22.4 7.75-43.08 20.71-59.49v-0.01c17.68-22.4 45.06-36.85 75.64-36.85 26.49 0 50.59 10.84 68.06 28.31a97.143 97.143 0 0 1 13.12 16.29c9.6 14.97 15.18 32.75 15.18 51.76V512c0 0.67 0 1.34 0.01 2.01z"/>
            <path fill="var(--primary)" d="M846.6 415.65c8.8-30.6 13.51-62.92 13.51-96.35V512c0 136.41-78.45 254.49-192.71 311.59a344.091 344.091 0 0 1-59.06 23.01C505.1 816.94 421.52 740.77 381.75 642.23c-16-39.64-24.92-82.9-25.16-128.22 1.07 84.23 69.16 152.31 153.38 153.38 0.67 0.01 1.34 0.01 2.01 0.01 0.67 0 1.34 0 2.01-0.01 55.11-0.31 107.18-13.43 153.4-36.52 86.28-43.09 152.12-120.96 179.21-215.22z"/>
          </svg>
        {:else}
          <svg viewBox="0 0 1024 1024" class="brand-svg">
            <path fill="var(--secondary)" d="M721.08 538.59c80.74 40.49 136.67 123.12 138.96 219.04v0.17c0.02 0.99 0.04 1.98 0.05 2.97 0.02 0.99 0.02 1.99 0.02 2.98 0 26.49-10.84 50.59-28.31 68.04-17.47 17.47-41.55 28.31-68.04 28.31-26.51 0-50.59-10.84-68.06-28.31-17.46-17.46-28.3-41.55-28.3-68.04 0-85.84-69.59-155.41-155.41-155.41-85.08 0-154.18 68.34-155.4 153.12 0.56-63.46 24.6-121.32 63.85-165.29 46.11-51.66 113.2-84.18 187.9-84.18 40.55 0.01 78.83 9.58 112.74 26.6z"/>
            <path fill="var(--primary)" d="M860.04 757.63c-2.29-95.92-58.22-178.55-138.96-219.04C687.17 521.57 648.89 512 608.35 512c-74.7 0-141.78 32.52-187.9 84.18-39.25 43.97-63.3 101.84-63.85 165.29-0.01 0.76-0.01 1.53-0.01 2.29 0 26.49-10.84 50.59-28.31 68.04-17.47 17.47-41.55 28.31-68.04 28.31-26.51 0-50.59-10.84-68.06-28.31-17.46-17.46-28.3-41.55-28.3-68.04 0-99.01 41.34-188.38 107.71-251.76-65-62.08-105.95-149.07-107.63-245.63v-0.17c-0.01-0.99-0.02-1.98-0.05-2.97-0.02-0.99-0.02-1.99-0.02-2.98 0-53 43.35-96.35 96.35-96.35 26.49 0 50.58 10.84 68.04 28.3 17.47 17.47 28.31 41.56 28.31 68.06 0 85.82 69.57 155.41 155.41 155.41 85.07 0 154.17-68.36 155.4-153.12 0.01-0.77 0.01-1.53 0.01-2.29 0-53 43.35-96.35 96.35-96.35 26.49 0 50.58 10.84 68.04 28.3 17.47 17.47 28.31 41.56 28.31 68.06 0 99.01-41.34 188.37-107.71 251.76 65.01 62.05 105.97 149.04 107.64 245.6z"/>
          </svg>
        {/if}
      </div>
    </div>
  {/if}

  <!-- Status Content Layer -->
  {#if currentState === 'status'}
    <div class="island-layer" transition:fade={{ duration: 300 }}>
      <div class="status-content">
        <div class="search-bar">
          <svg viewBox="0 0 24 24" class="search-icon"><path d="M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM21 21l-4.35-4.35" stroke="currentColor" fill="none" stroke-width="2" /></svg>
          <span>SEARCH LIBRARY...</span>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .dynamic-island {
    position: absolute;
    top: 40px;
    left: 50%;
    transform: translateX(-50%);
    background: #000;
    border-radius: 30px; /* Aligned with guide Radius 28-30 */
    border: 1px solid rgba(255, 255, 255, 0.1);
    z-index: 9999;
    overflow: hidden;
    cursor: pointer;
    transition: border-color 0.5s ease;
  }

  /* Absolute Layering to prevent glitchy layout jumps */
  .island-layer {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .letter-box {
    width: 40px; /* Increased from 32px */
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .brand-svg {
    width: 80%; /* Centered and well-balanced */
    height: 80%;
  }

  .brand-svg {
    width: 100%;
    height: 100%;
  }

  .status-content {
    width: 100%;
    padding: 0 16px;
    display: flex;
    align-items: center;
  }

  .search-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text-muted);
    font-family: var(--font-heading);
    font-size: 0.7rem;
    letter-spacing: 0.1em;
  }

  .search-icon {
    width: 16px;
    height: 16px;
  }
</style>
