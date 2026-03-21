<script lang="ts">
  import { fade } from "svelte/transition";

  let { mediaState, bufferingProgress, onClose } = $props<{ 
    mediaState: string; 
    bufferingProgress: number;
    onClose: () => void;
  }>();
</script>

<div class="island-layer" transition:fade={{ duration: 300 }}>
  <div class="status-content">
    {#if mediaState === "loading"}
      <div class="loading-state">
        <div class="spinner-box">
          <svg class="flux-spinner" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path class="flux-spin-cyan" d="M12 22 A10 10 0 0 1 12 2 A5 5 0 0 1 12 12 A5 5 0 0 0 12 22 Z" fill="var(--secondary)"/>
            <path class="flux-spin-violet" d="M12 2 A10 10 0 0 1 12 22 A5 5 0 0 1 12 12 A5 5 0 0 0 12 2 Z" fill="var(--primary)"/>
          </svg>
        </div>
        <span class="status-msg">LOADING MEDIA...</span>
      </div>
    {:else if mediaState === "buffering"}
      <div class="buffering-state">
        <span class="status-msg">BUFFERING {bufferingProgress}%</span>
        <div class="buffer-bar-container">
          <div class="buffer-bar" style="width: {bufferingProgress}%"></div>
        </div>
      </div>
    {:else}
      <div class="status-info">
        <span>FLUX SYSTEM STATUS: OPTIMAL</span>
      </div>
    {/if}
    <button 
      class="close-btn" 
      onclick={(e) => { e.stopPropagation(); onClose(); }}
      aria-label="Close"
    >
      <svg viewBox="0 0 24 24" class="close-icon">
        <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" fill="none" stroke-width="2" />
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

  .status-content {
    width: 100%;
    padding: 0 16px;
    display: flex;
    align-items: center;
  }

  .status-info {
    font-family: var(--font-heading);
    font-size: 0.7rem;
    color: var(--secondary);
    letter-spacing: 0.1em;
  }

  .loading-state, .buffering-state {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
  }

  .status-msg {
    font-family: var(--font-heading);
    font-size: 0.6rem;
    color: var(--text-white);
    letter-spacing: 0.1em;
  }

  .spinner-box {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  @keyframes flux-spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  @keyframes flux-breathe-cyan {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(-1.5px, -1.5px) scale(0.9); opacity: 1; }
  }
  @keyframes flux-breathe-violet {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(1.5px, 1.5px) scale(0.9); opacity: 1; }
  }

  .flux-spinner {
    width: 100%;
    height: 100%;
    animation: flux-spin 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
  }
  .flux-spin-cyan {
    animation: flux-breathe-cyan 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }
  .flux-spin-violet {
    animation: flux-breathe-violet 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }

  .buffer-bar-container {
    flex-grow: 1;
    height: 4px;
    background: var(--glass-bg-high);
    border-radius: 2px;
    overflow: hidden;
    margin-right: 12px;
  }

  .buffer-bar {
    height: 100%;
    background: var(--secondary);
    transition: width 0.3s ease;
  }

  .close-btn {
    margin-left: auto;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border-radius: 50%;
    transition: background 0.2s ease, color 0.2s ease;
  }

  .close-btn:hover {
    background: var(--glass-bg-high);
    color: var(--text-white);
  }

  .close-icon {
    width: 14px;
    height: 14px;
  }
</style>
