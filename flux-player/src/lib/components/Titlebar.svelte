<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import ProfileAvatar from './ProfileAvatar.svelte';
  const appWindow = getCurrentWindow();

  let isOnline = $state(true); // This will be connected to a store later

  const minimize = () => appWindow.minimize();
  const toggleMaximize = () => appWindow.toggleMaximize();
  const close = () => appWindow.close();
  const refresh = () => window.location.reload();
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="left-section" data-tauri-drag-region>
    <div class="status-indicators">
      <button class="refresh-btn" onclick={refresh} title="Global Refresh">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 2v6h-6M3 12a9 9 0 0 1 15-6.7L21 8M3 22v-6h6m12-4a9 9 0 0 1-15 6.7L3 16" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </button>
      
      <div class="connectivity-wrapper" class:offline={!isOnline} title={isOnline ? "Online" : "Offline"}>
        <div class="signal-bars">
          <div class="bar bar-1"></div>
          <div class="bar bar-2"></div>
          <div class="bar bar-3"></div>
          <div class="bar bar-4"></div>
        </div>
      </div>
    </div>
  </div>

  <div class="center-section" data-tauri-drag-region>
    <span class="app-name">FLUX</span>
  </div>

  <div class="right-section">
    <div class="user-actions">
      <button class="settings-btn" title="Settings Hub">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3" />
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
        </svg>
      </button>
      <ProfileAvatar />
    </div>

    <div class="window-controls">
      <button class="control-btn minimize" onclick={minimize} aria-label="Minimize">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="3" stroke-linecap="round">
          <path d="M5 12H13" stroke="var(--secondary)" />
          <path d="M11 12H19" stroke="var(--primary)" opacity="0.9"/>
        </svg>
      </button>

      <button class="control-btn maximize" onclick={toggleMaximize} aria-label="Maximize">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <rect class="win-max-violet" x="4" y="8" width="12" height="12" rx="3" stroke="var(--primary)" />
          <path class="win-max-cyan" d="M8 4H17C18.6569 4 20 5.34315 20 7V16" stroke="var(--secondary)" />
        </svg>
      </button>

      <button class="control-btn close" onclick={close} aria-label="Close">
        <svg viewBox="0 0 24 24" fill="none" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
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
    background: var(--bg-base);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 12px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    user-select: none;
    z-index: 10000;
  }

  .left-section, .center-section, .right-section {
    display: flex;
    align-items: center;
  }

  .left-section { flex: 1; }
  .center-section { 
    flex: 1; 
    justify-content: center;
  }
  .right-section { 
    flex: 1; 
    justify-content: flex-end;
    gap: 16px;
  }

  /* Status Indicators */
  .status-indicators {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .refresh-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    display: flex;
    transition: color 0.2s, transform 0.3s;
  }

  .refresh-btn:hover {
    color: var(--secondary);
    transform: rotate(180deg);
  }

  .refresh-btn svg { width: 18px; height: 18px; }

  /* Signal Bar Connectivity Indicator */
  .connectivity-wrapper {
    display: flex;
    align-items: center;
    padding: 0 4px;
  }

  .signal-bars {
    display: flex;
    align-items: flex-end;
    gap: 3px;
    height: 16px;
  }

  .bar {
    width: 3px;
    background: var(--text-muted);
    border-radius: 1.5px;
    transition: all 0.3s ease;
  }

  .bar-1 { height: 6px; }
  .bar-2 { height: 9px; }
  .bar-3 { height: 12px; }
  .bar-4 { height: 16px; }

  .connectivity-wrapper:not(.offline) .bar {
    background: var(--secondary); /* Cyan for online */
  }

  .connectivity-wrapper.offline .bar {
    background: #ff0000; /* Sharp Red for offline */
  }

  /* App Branding */
  .app-name {
    font-family: var(--font-heading);
    font-size: 0.75rem;
    color: var(--text-muted);
    letter-spacing: 0.25em;
    font-weight: 700;
  }

  /* User Actions */
  .user-actions {
    display: flex;
    align-items: center;
    gap: 12px;
    padding-right: 12px;
    border-right: 1px solid rgba(255, 255, 255, 0.1);
  }

  .settings-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    display: flex;
    transition: color 0.2s, transform 0.5s;
  }

  .settings-btn:hover {
    color: var(--primary);
    transform: rotate(90deg);
  }

  .settings-btn svg { width: 16px; height: 16px; }

  /* Window Controls */
  .window-controls {
    display: flex;
    gap: 4px;
  }

  .control-btn {
    background: none;
    border: none;
    width: 36px;
    height: var(--titlebar-height);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.2s;
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
    width: 12px;
    height: 12px;
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
