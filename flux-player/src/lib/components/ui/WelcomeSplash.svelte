<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Icon from '$lib/components/ui/Icon.svelte';
  import { onboarding, completeTourSection } from '$lib/stores/onboarding';
  import { isScanning, loadLibraryFromDb } from '$lib/stores/media';
  import { fade, scale } from 'svelte/transition';
  import { backOut } from 'svelte/easing';

  type FolderEntry = { path: string; type: 'video' | 'audio' };

  let defaultFolders = $state<FolderEntry[]>([]);
  let checkingFolders = $state(true);
  
  // Animation states
  let showRings = $state(false);
  let showFlash = $state(false);
  let showContent = $state(false);
  
  onMount(async () => {
    // 1. Kick off intro animation sequence
    setTimeout(() => showRings = true, 300);
    setTimeout(() => {
      showRings = false;
      showFlash = true;
    }, 1800);
    setTimeout(() => {
      showFlash = false;
      showContent = true;
    }, 2200);

    // 2. Fetch default folders in background
    try {
      defaultFolders = await invoke<FolderEntry[]>('get_default_media_folders');
    } catch (e) {
      console.error('Splash: Failed to fetch default folders:', e);
    } finally {
      checkingFolders = false;
    }
  });

  async function confirmAndStart() {
    if (defaultFolders.length > 0) {
      try {
        // Save these folders as the initial library setting
        await invoke('save_setting', { key: 'library_folders', value: JSON.stringify(defaultFolders) });
        
        // Kick off scanning for the first folder to get things started (tauri handles background threading)
        isScanning.set(true);
        invoke('start_library_scan', { dir: defaultFolders[0].path })
          .then(async () => {
            await loadLibraryFromDb();
            window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Library Initialized', icon: 'library' } }));
          })
          .catch(e => console.error("Initial scan failed", e))
          .finally(() => isScanning.set(false));
      } catch (e) {
        console.error('Failed to save default folders on first run', e);
      }
    }
    
    // Close splash and move to global tour
    completeTourSection();
  }

  function modifyFolders() {
    // Close splash, user will be left in whatever route they are on (Library by default)
    // and can open settings manually.
    completeTourSection();
    window.dispatchEvent(new CustomEvent('flux-import-folder'));
  }

  function handleKeydown(e: KeyboardEvent) {
    if (showContent && e.key === 'Enter') {
      confirmAndStart();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="splash-overlay glass-dark">
  
  {#if showRings}
    <div class="anim-container" out:fade={{ duration: 200, delay: 0 }}>
      <div class="ring primary-ring"></div>
      <div class="ring secondary-ring"></div>
    </div>
  {/if}

  {#if showFlash}
    <div class="flash" out:fade={{ duration: 400 }}></div>
  {/if}

  {#if showContent}
    <div class="zenith-card glass" in:scale={{ duration: 600, easing: backOut, start: 0.9 }}>
      
      <div class="branding">
        <img src="/flux.png" alt="Flux Logo" class="brand-logo" />
        <h1 class="brand-name">FLUX</h1>
      </div>

      <div class="welcome-text">
        <h2>Welcome to the Beta.</h2>
        <p>Flux is a high-performance, local-first media player. We keep your data private and only scan the folders you authorize.</p>
      </div>

      <div class="folder-discovery">
        {#if checkingFolders}
          <div class="discovery-loading">
            <div class="spinner"></div>
            <span>Locating standard media folders...</span>
          </div>
        {:else}
          <div class="discovery-results" in:fade={{ duration: 300 }}>
            <span>We found your system folders:</span>
            <div class="folder-list">
              {#each defaultFolders as folder}
                <div class="folder-chip">
                  <Icon name={folder.type === 'video' ? 'movie' : 'music'} size={14} />
                  {folder.path}
                </div>
              {/each}
              {#if defaultFolders.length === 0}
                <div class="folder-chip empty">No default folders found.</div>
              {/if}
            </div>
          </div>
        {/if}
      </div>

      <div class="actions">
        {#if defaultFolders.length > 0}
          <button class="action-btn primary" onclick={confirmAndStart}>
            <span>Confirm & Start Scan</span>
            <Icon name="refresh-spark" size={16} />
          </button>
        {/if}
        <button class="action-btn secondary" onclick={modifyFolders}>
          Choose Custom Folder
        </button>
      </div>
      
    </div>
  {/if}

</div>

<style>
  .splash-overlay {
    position: fixed;
    inset: 0;
    z-index: 100000;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: rgba(10, 10, 12, 0.95);
  }

  /* ── 1. The Rings (Zenith Convergence) ── */
  .anim-container {
    position: absolute;
    width: 200px;
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .ring {
    position: absolute;
    border-radius: 50%;
    border: 2px solid transparent;
  }

  .primary-ring {
    width: 150px;
    height: 150px;
    border-top: 2px solid var(--primary);
    border-right: 2px solid var(--primary);
    animation: rotate-r 1.2s cubic-bezier(0.5, 0, 0.5, 1) forwards;
  }

  .secondary-ring {
    width: 120px;
    height: 120px;
    border-bottom: 2px solid var(--secondary);
    border-left: 2px solid var(--secondary);
    animation: rotate-l 1s cubic-bezier(0.5, 0, 0.5, 1) forwards;
  }

  @keyframes rotate-r {
    0% { transform: rotate(0deg) scale(1); filter: blur(0px); opacity: 0; }
    20% { opacity: 1; }
    80% { filter: blur(2px); }
    100% { transform: rotate(720deg) scale(0.1); filter: blur(8px); opacity: 0; }
  }

  @keyframes rotate-l {
    0% { transform: rotate(0deg) scale(1); filter: blur(0px); opacity: 0; }
    20% { opacity: 1; }
    80% { filter: blur(2px); }
    100% { transform: rotate(-720deg) scale(0.1); filter: blur(8px); opacity: 0; }
  }

  /* ── 2. The Flash ── */
  .flash {
    position: absolute;
    width: 10px;
    height: 10px;
    background: #fff;
    border-radius: 50%;
    box-shadow: 0 0 100px 50px rgba(255, 255, 255, 0.8),
                0 0 200px 100px var(--secondary);
    filter: blur(4px);
  }

  /* ── 3. The Content ── */
  .zenith-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 60px;
    border-radius: 32px;
    width: 600px;
    max-width: 90vw;
    background: rgba(14, 14, 16, 0.85); /* Darker glass */
    box-shadow: 0 40px 100px rgba(0, 0, 0, 0.9), 
                inset 0 1px 1px rgba(255, 255, 255, 0.1);
  }

  .branding {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-bottom: 32px;
  }

  .brand-logo {
    width: 96px;
    height: 96px;
    object-fit: contain;
    margin-bottom: 16px;
    filter: drop-shadow(0 0 20px rgba(138, 43, 226, 0.4));
  }

  .brand-name {
    font-family: var(--font-heading);
    font-size: 2rem;
    font-weight: 800;
    letter-spacing: 0.15em;
    color: var(--text-main);
    line-height: 1;
    margin: 0;
  }

  .welcome-text {
    text-align: center;
    margin-bottom: 40px;
  }

  .welcome-text h2 {
    font-size: 1.5rem;
    margin-bottom: 12px;
    color: var(--text-main);
  }

  .welcome-text p {
    font-size: 1rem;
    color: var(--text-muted);
    line-height: 1.5;
    max-width: 400px;
    margin: 0 auto;
  }

  .folder-discovery {
    width: 100%;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 24px;
    margin-bottom: 40px;
    min-height: 100px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .discovery-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    color: var(--secondary);
    font-weight: 500;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(0, 255, 255, 0.2);
    border-top-color: var(--secondary);
    border-radius: 50%;
    animation: sideSpin 0.8s linear infinite;
  }

  @keyframes sideSpin {
    to { transform: rotate(360deg); }
  }

  .discovery-results > span {
    display: block;
    font-size: 0.85rem;
    color: var(--text-muted);
    margin-bottom: 12px;
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
  }

  .folder-chip {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
    border-radius: 8px;
    font-family: monospace;
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .folder-chip :global(svg) {
    color: var(--secondary);
    opacity: 0.8;
  }

  .folder-chip.empty {
    font-family: var(--font-body);
    font-style: italic;
    color: var(--text-muted);
    background: transparent;
    border-style: dashed;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 100%;
    max-width: 320px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 14px 24px;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-family: var(--font-body);
  }

  .action-btn.primary {
    background: linear-gradient(135deg, rgba(0, 255, 255, 0.1), rgba(138, 43, 226, 0.2));
    color: var(--text-main);
    border: 1px solid rgba(0, 255, 255, 0.3);
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.1);
  }

  .action-btn.primary:hover {
    background: linear-gradient(135deg, rgba(0, 255, 255, 0.2), rgba(138, 43, 226, 0.3));
    border-color: var(--secondary);
    box-shadow: 0 0 30px rgba(0, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .action-btn.secondary {
    background: transparent;
    color: var(--text-muted);
    border: 1px solid transparent;
  }

  .action-btn.secondary:hover {
    color: var(--text-main);
    background: var(--glass-bg-low);
  }
</style>
