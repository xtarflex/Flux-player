<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { goto } from '$app/navigation';
  import ProfileAvatar from './ProfileAvatar.svelte';
  import { tooltip } from '$lib/actions/tooltip';
  
  let appWindow: any;
  if (typeof window !== 'undefined' && ('__TAURI_INTERNALS__' in window)) {
    appWindow = getCurrentWindow();
  }

  let isOnline = $state(true);
  let pcName = $state("FLUX-DEVICE");
  let displayName = $state("");

  async function loadProfile() {
    try {
      pcName = await invoke('get_computer_name');
      displayName = pcName; // Set OS Name as first fallback
      
      const saved = await invoke<string | null>('get_setting', { key: 'display_name' });
      if (saved) displayName = saved;
    } catch (e) {
      console.warn("Flux Titlebar: Failed to fetch DB profile (expected on clean boot):", e);
    }
  }

  onMount(() => {
    loadProfile();
    console.log("Flux Titlebar: Initialized");

    window.addEventListener('flux-settings-updated', loadProfile);
    return () => window.removeEventListener('flux-settings-updated', loadProfile);
  });

  const minimize = async () => appWindow.minimize();
  const toggleMaximize = async () => appWindow.toggleMaximize();
  const close = async () => appWindow.close();
  const refresh = () => window.location.reload();
  const openSettings = () => goto('/settings');
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="left-section" data-tauri-drag-region>
    <div class="user-profile" data-tauri-drag-region>
      <ProfileAvatar size="small" />
      <span class="pc-name" data-tauri-drag-region>{displayName}</span>
    </div>
  </div>

  <div class="center-section" data-tauri-drag-region>
    <span class="app-name" data-tauri-drag-region>FLUX</span>
  </div>

  <div class="right-section">
    <div class="action-group">
      <button class="refresh-btn" onclick={refresh} aria-label="Refresh Library" use:tooltip={{ content: 'Refresh Library', shortcut: 'Ctrl R', placement: 'bottom' }}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 2v6h-6M3 12a9 9 0 0 1 15-6.7L21 8M3 22v-6h6m12-4a9 9 0 0 1-15 6.7L3 16" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
      
      <div class="connectivity-wrapper" class:offline={!isOnline} aria-label={isOnline ? "Online" : "Offline"} use:tooltip={{ content: isOnline ? 'Online' : 'Offline', placement: 'bottom' }}>
        <div class="signal-bars">
          <div class="bar bar-1"></div>
          <div class="bar bar-2"></div>
          <div class="bar bar-3"></div>
          <div class="bar bar-4"></div>
        </div>
      </div>

      <button class="audio-device-btn" aria-label="System Speakers" use:tooltip={{ content: 'System Speakers (Realtek HD Audio)', placement: 'bottom' }}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M11 5L6 9H2v6h4l5 4V5z" stroke="var(--secondary)" />
          <path d="M15.54 8.46a5 5 0 0 1 0 7.07" stroke="var(--primary)" opacity="0.8" />
          <path d="M19.07 4.93a10 10 0 0 1 0 14.14" stroke="var(--primary)" opacity="0.4" />
        </svg>
      </button>

      <button class="settings-btn" onclick={openSettings} aria-label="Settings Hub" use:tooltip={{ content: 'Settings Hub', shortcut: 'Ctrl ,', placement: 'bottom' }}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </button>
    </div>

    <div class="window-controls">
      <button class="control-btn minimize" onclick={minimize} aria-label="Minimize">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="2.5" stroke-linecap="round">
          <path d="M5 12H13" stroke="var(--secondary)" />
          <path d="M11 12H19" stroke="var(--primary)" opacity="0.9"/>
        </svg>
      </button>

      <button class="control-btn maximize" onclick={toggleMaximize} aria-label="Maximize">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect class="win-max-violet" x="4" y="8" width="12" height="12" rx="2.5" stroke="var(--primary)" />
          <path class="win-max-cyan" d="M8 4H17C18.6569 4 20 5.34315 20 7V16" stroke="var(--secondary)" />
        </svg>
      </button>

      <button class="control-btn close" onclick={close} aria-label="Close">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <path class="win-close-cyan" d="M7 7 L 10.5 10.5 M 13.5 13.5 L 17 17" stroke="var(--secondary)" />
          <path class="win-close-violet" d="M17 7 L 7 17" stroke="var(--primary)" />
        </svg>
      </button>
    </div>
  </div>
</div>

<style>
  .titlebar {
    grid-area: titlebar;
    height: var(--titlebar-height);
    background: var(--glass-bg-low);
    backdrop-filter: blur(15px);
    -webkit-backdrop-filter: blur(15px);
    border-bottom: 1px solid var(--glass-border-low);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    user-select: none;
    z-index: 10000;
  }

  .left-section, .center-section, .right-section {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .left-section { 
    flex: 1; 
    gap: 12px;
  }
  .center-section { 
    flex: 0; 
    justify-content: center;
    min-width: 100px;
  }
  .right-section { 
    flex: 1; 
    justify-content: flex-end;
    gap: 20px;
  }

  /* User Profile at Left */
  .user-profile {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .pc-name {
    font-family: var(--font-body);
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    letter-spacing: 0.02em;
  }

  /* Action Group (Refresh, Connectivity, Settings) */
  .action-group {
    display: flex;
    align-items: center;
    gap: 18px;
    padding-right: 20px;
    border-right: 1px solid rgba(255, 255, 255, 0.1);
  }

  .refresh-btn, .settings-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    display: flex;
    transition: all 0.2s ease;
  }

  .refresh-btn:hover {
    color: var(--secondary);
    transform: rotate(180deg);
  }

  .settings-btn:hover {
    color: var(--primary);
    transform: rotate(90deg);
  }

  .refresh-btn svg { width: 16px; height: 16px; }
  .settings-btn svg { width: 18px; height: 18px; }

  .audio-device-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    display: flex;
    transition: all 0.2s ease;
  }

  .audio-device-btn:hover {
    color: var(--secondary);
    transform: translateY(-1px);
  }

  .audio-device-btn svg {
    width: 18px;
    height: 18px;
  }

  /* Signal Bar Connectivity Indicator */
  .signal-bars {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    height: 14px;
  }

  .bar {
    width: 2.5px;
    background: var(--text-muted);
    border-radius: 1px;
    transition: all 0.3s ease;
  }

  .bar-1 { height: 4px; }
  .bar-2 { height: 7px; }
  .bar-3 { height: 10px; }
  .bar-4 { height: 14px; }

  .connectivity-wrapper:not(.offline) .bar {
    background: var(--secondary);
  }

  .connectivity-wrapper.offline .bar {
    background: #ff0000;
  }

  /* App Branding */
  .app-name {
    font-family: var(--font-heading);
    font-size: 0.75rem;
    color: var(--text-muted);
    letter-spacing: 0.3em;
    font-weight: 700;
    opacity: 0.8;
  }

  /* Window Controls */
  .window-controls {
    display: flex;
    gap: 2px;
    margin-right: -8px; /* Offset to align with edge better */
  }

  .control-btn {
    background: none;
    border: none;
    width: 44px;
    height: var(--titlebar-height);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .control-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .control-btn.close:hover {
    background: #e81123;
  }

  .control-btn.close:hover svg path {
    stroke: white;
  }

  .control-btn svg {
    width: 16px;
    height: 16px;
  }

  /* maximize/close animations */
  .win-max-cyan, .win-max-violet {
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }
  .control-btn.maximize:hover .win-max-cyan { transform: translate(1px, -1px); }
  .control-btn.maximize:hover .win-max-violet { transform: translate(-1px, 1px); }

  .win-close-cyan, .win-close-violet {
    transform-origin: 12px 12px;
  }

  @keyframes close-elastic-cyan {
    0%, 100% { transform: rotate(0deg) scaleY(1); }
    30% { transform: rotate(35deg) scaleY(1.3); } 
    65% { transform: rotate(-25deg) scaleY(0.85); } 
    85% { transform: rotate(10deg) scaleY(1.05); } 
  }
  @keyframes close-elastic-violet {
    0%, 100% { transform: rotate(0deg) scaleY(1); }
    30% { transform: rotate(-35deg) scaleY(1.3); } 
    65% { transform: rotate(25deg) scaleY(0.85); } 
    85% { transform: rotate(-10deg) scaleY(1.05); } 
  }

  .control-btn.close:hover .win-close-cyan {
    animation: close-elastic-cyan 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  .control-btn.close:hover .win-close-violet {
    animation: close-elastic-violet 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
</style>
