<script lang="ts">
  import '../app.css';
  import { onMount, untrack } from 'svelte';
  import { goto, beforeNavigate } from '$app/navigation';
  import { get } from 'svelte/store';
  import { page } from '$app/stores';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import DynamicIsland from '$lib/components/DynamicIsland.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import PlaybackFooter from '$lib/components/PlaybackFooter.svelte';
  import AudioEngine from '$lib/components/player/AudioEngine.svelte';
  import PlayerEngine from '$lib/components/player/PlayerEngine.svelte';
  import { activeMedia, playbackState, activateMiniPlayer, deactivateMiniPlayer } from '$lib/stores/playback';
  import { isScanning, loadLibraryFromDb } from '$lib/stores/media';
  import { activeMenu, closeMenu, isSidebarCollapsed } from '$lib/stores/ui';
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import Tooltip from '$lib/components/ui/Tooltip.svelte';
  import ContextMenu from '$lib/components/ui/ContextMenu.svelte';
  import WelcomeSplash from '$lib/components/ui/WelcomeSplash.svelte';
  import OnboardingOverlay from '$lib/components/ui/OnboardingOverlay.svelte';
  import ApiLimitModal from '$lib/components/ui/ApiLimitModal.svelte';
  import { onboarding, triggerTour } from '$lib/stores/onboarding';
  import { listen } from '@tauri-apps/api/event';

  let { children } = $props();
  let showShortcutsRef = $state(false);
  let showApiLimitModal = $state(false);


  // ── Theater & PiP Mode State ──────────────────────────────────────────────
  let isTheaterMode = $derived($playbackState.isTheaterMode);
  let isPiP = $derived($playbackState.isPiP);
  let sidebarReveal = $state(false);
  let footerReveal = $state(false);
  let uiIdle = $derived($playbackState.isIdle);
  let idleTimer: ReturnType<typeof setTimeout> | null = null;

  // ── beforeNavigate: intercept leaving /playing ──────────
  beforeNavigate(({ to }) => {
    const leavingPlayer = $page.url.pathname === '/playing';
    const hasActiveVideo = !!$activeMedia && $activeMedia.type !== 'audio';
    const goingToPlayer = to?.url.pathname === '/playing';
    
    if (leavingPlayer && !goingToPlayer) {
      if (hasActiveVideo) {
          activateMiniPlayer();
        } else {
        // Just turn off theater mode for audio
        playbackState.update(s => ({ ...s, isTheaterMode: false }));
      }
    }
  });

  /**
   * Handles edge-hover for theater mode UI reveal (Blueprint §2).
   * @param e - Mouse move event from the app-container.
   */
  function handleTheaterMouseMove(e: MouseEvent) {
    if (!isTheaterMode) return;
    const { clientX, clientY } = e;
    sidebarReveal = clientX < Math.max(260, window.innerWidth * 0.2); // 20% or physical sidebar width
    footerReveal = clientY > Math.min(window.innerHeight - 100, window.innerHeight * 0.7); // 30% or physical footer height
    
    // Reset idle state on movement
    if ($playbackState.isIdle) {
      playbackState.update(s => ({ ...s, isIdle: false }));
    }
    
    if (idleTimer) clearTimeout(idleTimer);
    idleTimer = setTimeout(() => {
      // Only go idle if we are still playing and in theater mode
      const currentState = get(playbackState);
      if (currentState.isPlaying && currentState.isTheaterMode) {
        playbackState.update(s => ({ ...s, isIdle: true }));
      }
    }, 3000); // 3-second delay, more relaxed than 1.5s
  }

  // ── Pause Reveal: ensure UI shows when playback stops ──────────────────────
  $effect(() => {
    // Accessing $playbackState.isPlaying and isTheaterMode makes this effect 
    // rerun when they change.
    if (!$playbackState.isPlaying && isTheaterMode) {
      // 1. Guard check: only update if we are currently idle
      // 2. Use untrack to prevent the update itself from registering as a dependency
      untrack(() => {
        if ($playbackState.isIdle) {
          playbackState.update(s => ({ ...s, isIdle: false }));
        }
        footerReveal = true;
        if (idleTimer) clearTimeout(idleTimer);
      });
    }
  });

  async function importFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Media Folder to Scan'
    });

    if (selected && typeof selected === 'string') {
      isScanning.set(true);
      // Global scanning state handles feedback
      
      try {
        await invoke('start_library_scan', { dir: selected });
        window.dispatchEvent(new CustomEvent('flux-library-updated'));
        window.dispatchEvent(new CustomEvent('flux-toast', { 
          detail: { label: 'Library Updated', icon: 'library' } 
        }));
      } catch (e) {
        console.error('Failed to start scan:', e);
      } finally {
        isScanning.set(false);
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
        case 'q': e.preventDefault(); deactivateMiniPlayer(); goto('/playing'); dispatchToast('Now Playing', 'playing'); break;
        case ',': e.preventDefault(); goto('/settings'); dispatchToast('Settings', 'settings'); break;
        case 'r':
          e.preventDefault();
          if (e.shiftKey) {
            dispatchToast('Global Reload', 'refresh');
            setTimeout(() => window.location.reload(), 500);
          } else {
            dispatchToast('Refreshing Section', 'refresh');
            window.dispatchEvent(new CustomEvent('flux-refresh-context'));
          }
          break;
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
        case 'b': e.preventDefault(); isSidebarCollapsed.update(v => !v); break;
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
    // Orchestrate Onboarding Flow
    if (!$onboarding.completedSections.includes('welcome')) {
      triggerTour('welcome');
    } else if (!$onboarding.completedSections.includes('global') && !$onboarding.isActive) {
      triggerTour('global');
    }
    
    // Subscribe to onboarding changes to cascade from Welcome -> Global automatically
    const unsubOnboarding = onboarding.subscribe(state => {
      if (!state.isActive && state.completedSections.includes('welcome') && !state.completedSections.includes('global')) {
        setTimeout(() => triggerTour('global'), 500); // Small delay for smooth transition
      }
    });

    window.addEventListener('keydown', handleGlobalKeydown);
    
    // API Limit Listener (Milestone 2.5)
    const unsubLimit = listen('flux-require-api-key', () => {
      showApiLimitModal = true;
    });

    // Bridge Tauri Library Updates to Store and Browser Window
    const unsubLibrary = listen('flux-library-updated', () => {
      console.log('[Flux] Received background library update event');
      loadLibraryFromDb(); // Sync store immediately
      window.dispatchEvent(new CustomEvent('flux-library-updated'));
    });

    // Centralized folder picker trigger
    const handleImportEvent = () => importFolder();
    window.addEventListener('flux-import-folder', handleImportEvent);

    return () => {
      window.removeEventListener('keydown', handleGlobalKeydown);
      window.removeEventListener('flux-import-folder', handleImportEvent);
      if (idleTimer) clearTimeout(idleTimer);
      unsubOnboarding();
      unsubLimit.then(u => u());
      unsubLibrary.then(u => u());
    };
  });
</script>

<div 
  class="app-container"
  class:player-mode={isTheaterMode}
  class:pip-mode={isPiP}
  class:sidebar-reveal={sidebarReveal && isTheaterMode}
  class:footer-reveal={footerReveal && isTheaterMode}
  class:ui-idle={uiIdle && isTheaterMode}
  onmousemove={handleTheaterMouseMove}
  role="presentation"
>
  <div data-titlebar class="titlebar-slot"><Titlebar /></div>
  <div data-sidebar class="sidebar-slot"><Sidebar /></div>
  {#if !isPiP}
    <DynamicIsland />
  {/if}
  <main class="main-content" class:is-settings-page={$page.url.pathname.startsWith('/settings')}>
    {@render children()}
  </main>
  <div data-footer class="footer-slot"><PlaybackFooter /></div>
  <PlayerEngine />
  <AudioEngine />
  <Tooltip />

  {#if $onboarding.isActive && $onboarding.currentSection === 'welcome'}
    <WelcomeSplash />
  {:else if $onboarding.isActive}
    <OnboardingOverlay />
  {/if}

  {#if showApiLimitModal}
    <ApiLimitModal onclose={() => showApiLimitModal = false} />
  {/if}

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
            <div class="row"><span>Ctrl + R</span> Refresh Context</div>
            <div class="row"><span>Ctrl+Shift+R</span> Global Reload</div>
            <div class="row"><span>Ctrl + F</span> Search</div>
            <div class="row"><span>Ctrl + N</span> New Playlist</div>
            <div class="row"><span>Ctrl + O</span> Import Folder</div>
            <div class="row"><span>Ctrl + S</span> Save Queue</div>
            <div class="row"><span>Ctrl + B</span> Toggle Sidebar</div>
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

{#if $activeMenu}
  <ContextMenu 
    x={$activeMenu.x} 
    y={$activeMenu.y} 
    items={$activeMenu.items} 
    onclose={closeMenu} 
  />
{/if}

<style>
  /* Theater Mode slot wrappers — transparent to grid layout in normal mode */
  .titlebar-slot, .sidebar-slot, .footer-slot {
    display: contents;
  }

  :global(.app-container.pip-mode) .titlebar-slot,
  :global(.app-container.pip-mode) .sidebar-slot,
  :global(.app-container.pip-mode) .footer-slot,
  :global(.app-container.pip-mode) .main-content {
    display: none !important;
  }

  :global(.app-container.pip-mode) {
    grid-template-areas: "main";
    grid-template-columns: 1fr;
    grid-template-rows: 1fr;
    background: #000;
  }

  /* 
    Theater Mode Breakout: 
    Override display: contents because it prevents fixed positioning 
    and transforms from working correctly on these wrappers.
  */
  :global(.app-container.player-mode) .titlebar-slot,
  :global(.app-container.player-mode) .sidebar-slot,
  :global(.app-container.player-mode) .footer-slot {
    display: block;
  }

  .main-content {
    grid-area: main;
    overflow-y: auto;
    padding: 20px;
    transition: padding 0.3s ease;
  }
  .main-content.is-settings-page {
    padding: 0;
    overflow: hidden;
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
