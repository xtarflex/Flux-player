<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import DynamicIsland from '$lib/components/DynamicIsland.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import PlaybackFooter from '$lib/components/PlaybackFooter.svelte';
  
  let { children } = $props();
  let showShortcutsRef = $state(false);

  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';

  async function importFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Media Folder to Scan'
    });

    if (selected && typeof selected === 'string') {
      window.dispatchEvent(new CustomEvent('flux-toast', { 
        detail: { label: 'Scanning Library...', icon: 'refresh' } 
      }));
      
      try {
        await invoke('start_library_scan', { dir: selected });
        window.dispatchEvent(new CustomEvent('flux-library-updated'));
        window.dispatchEvent(new CustomEvent('flux-toast', { 
          detail: { label: 'Library Updated', icon: 'library' } 
        }));
      } catch (e) {
        console.error('Failed to start scan:', e);
      }
    }
  }

  function handleGlobalKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    const isEditing = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;
    if (isEditing) {
      if (e.key === 'Escape') target.blur();
      return;
    }

    const isCmd = e.ctrlKey || e.metaKey;
    const key = e.key.toLowerCase();

    // 1. App Navigation
    if (isCmd) {
      const dispatchToast = (label: string, icon: string) => {
        window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label, icon } }));
      };

      switch(key) {
        case 'l': e.preventDefault(); goto('/library'); dispatchToast('Library Navigation', 'library'); break;
        case 'd': e.preventDefault(); goto('/discovery'); dispatchToast('Discovery Navigation', 'discovery'); break;
        case 'p': e.preventDefault(); goto('/playlists'); dispatchToast('Playlists', 'playlists'); break;
        case 'q': e.preventDefault(); goto('/playing'); dispatchToast('Now Playing', 'playing'); break;
        case ',': e.preventDefault(); goto('/settings'); dispatchToast('Settings', 'settings'); break;
        case 'r': e.preventDefault(); dispatchToast('Refreshing Library', 'refresh'); break;
        case 'f': e.preventDefault(); window.dispatchEvent(new CustomEvent('flux-search-focus')); break;
        case 'n': 
          e.preventDefault();
          if (e.shiftKey) { dispatchToast('New Smart Playlist', 'discovery'); } // Use discovery as star
          else { dispatchToast('New Playlist', 'new-playlist'); }
          break;
        case 'o':
          e.preventDefault();
          importFolder();
          break;
        case 's':
          e.preventDefault();
          if (e.shiftKey) { dispatchToast('Captured Screenshot', 'screenshot'); }
          else { dispatchToast('Saved Queue as Playlist', 'save'); }
          break;
        case 'w': e.preventDefault(); try { (window as any).__TAURI__?.window.getCurrent().close(); } catch(e) {} break;
        case '/': e.preventDefault(); showShortcutsRef = !showShortcutsRef; break;
      }
    }

    // 2. Escape Handling (Global Level)
    if (e.key === 'Escape') {
      if (showShortcutsRef) {
        showShortcutsRef = false;
        return;
      }
    }

    // Playback hotkeys are handled in PlaybackFooter.svelte
  }

  onMount(() => {
    window.addEventListener('keydown', handleGlobalKeydown);
    return () => window.removeEventListener('keydown', handleGlobalKeydown);
  });
</script>

<div class="app-container">
  <Titlebar />
  <Sidebar />
  <DynamicIsland />
  <main class="main-content">
    {@render children()}
  </main>
  <PlaybackFooter />

  <!-- Shortcuts Reference Modal -->
  {#if showShortcutsRef}
    <div 
      class="shortcuts-overlay" 
      onclick={() => showShortcutsRef = false}
      onkeydown={(e) => e.key === 'Escape' && (showShortcutsRef = false)}
      role="button"
      tabindex="-1"
      aria-label="Close shortcuts"
    >
      <div 
        class="shortcuts-card glass" 
        onclick={(e) => e.stopPropagation()} 
        role="none"
      >
        <h2>Keyboard Shortcuts</h2>
        <div class="shortcuts-grid">
          <div class="col">
            <h3>App & Nav</h3>
            <div class="row"><span>Ctrl + L</span> Library</div>
            <div class="row"><span>Ctrl + D</span> Discovery</div>
            <div class="row"><span>Ctrl + P</span> Playlists</div>
            <div class="row"><span>Ctrl + Q</span> Queue</div>
            <div class="row"><span>Ctrl + ,</span> Settings</div>
            <div class="row"><span>Ctrl + R</span> Refresh</div>
            <div class="row"><span>Ctrl + F</span> Search</div>
            <div class="row"><span>Ctrl + N</span> New Playlist</div>
            <div class="row"><span>Ctrl + O</span> Import Folder</div>
            <div class="row"><span>Ctrl + S</span> Save Queue</div>
          </div>
          <div class="col">
            <h3>Library Mode</h3>
            <div class="row"><span>Ctrl + A</span> Select All</div>
            <div class="row"><span>V</span> Cycle View</div>
            <div class="row"><span>Enter</span> Open Details</div>
            <div class="row"><span>Ctrl + E</span> Play Item</div>
            <div class="row"><span>Arrows</span> Grid Nav</div>
            <div class="row"><span>Shift + Click</span> Range Select</div>
            <div class="row"><span>ESC</span> Clear / Exit</div>
          </div>
          <div class="col">
            <h3>Playback</h3>
            <div class="row"><span>Space / K</span> Play/Pause</div>
            <div class="row"><span>Arrows ←/→</span> Seek 5%</div>
            <div class="row"><span>J / L</span> Seek 10%</div>
            <div class="row"><span>Shift + ←/→</span> Next/Prev</div>
            <div class="row"><span>Arrows ↑/↓</span> Volume</div>
            <div class="row"><span>M</span> Mute</div>
            <div class="row"><span>0-9</span> Jump to %</div>
            <div class="row"><span>F / F11</span> Fullscreen</div>
          </div>
        </div>
        <button class="close-ref" onclick={() => showShortcutsRef = false}>✕</button>
      </div>
    </div>
  {/if}
</div>

<style>
  .main-content {
    grid-area: main;
    overflow-y: auto;
    padding: 20px;
  }

  /* Shortcuts Reference */
  .shortcuts-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(8px);
    z-index: 9000;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: fadeIn 0.3s ease;
  }

  .shortcuts-card {
    background: rgba(14, 14, 16, 0.9);
    border: 1px solid var(--glass-border-mid);
    padding: 40px;
    border-radius: 32px;
    width: 800px;
    position: relative;
    box-shadow: 0 40px 100px rgba(0, 0, 0, 0.8);
  }

  .shortcuts-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 32px;
    margin-top: 32px;
  }

  .col h3 {
    font-size: 14px;
    color: var(--secondary);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin-bottom: 16px;
  }

  .row {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
    padding: 6px 0;
    border-bottom: 1px solid var(--glass-border-low);
  }

  .row span {
    font-family: monospace;
    color: var(--text-muted);
  }

  .close-ref {
    position: absolute;
    top: 24px;
    right: 24px;
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 20px;
    cursor: pointer;
  }

  @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }
</style>
