<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  const appWindow = getCurrentWindow();

  const minimize = () => appWindow.minimize();
  const toggleMaximize = () => appWindow.toggleMaximize();
  const close = () => appWindow.close();
</script>

<div class="titlebar" data-tauri-drag-region>
  <div class="app-info" data-tauri-drag-region>
    <span class="app-name">FLUX</span>
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

<style>
  .titlebar {
    grid-area: titlebar;
    height: 32px;
    background: var(--bg-surface);
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 12px;
    border-bottom: 1px solid var(--border);
    user-select: none;
  }

  .app-name {
    font-family: var(--font-heading);
    font-size: 0.65rem;
    color: var(--text-muted);
    letter-spacing: 0.1em;
  }

  .window-controls {
    display: flex;
    gap: 4px;
  }

  .control-btn {
    background: none;
    border: none;
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: background 0.2s ease;
    border-radius: 4px;
    padding: 4px;
  }

  .control-btn:hover {
    background: rgba(255, 255, 255, 0.05);
  }

  .control-btn.close:hover {
    background: rgba(255, 50, 50, 0.2);
  }

  .control-btn svg {
    width: 14px;
    height: 14px;
  }

  /* Specific Hover Animations from Icon Forge */
  .control-btn.maximize:hover .win-max-cyan { transform: translate(1px, -1px); }
  .control-btn.maximize:hover .win-max-violet { transform: translate(-1px, 1px); }
  
  .win-max-cyan, .win-max-violet {
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

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
