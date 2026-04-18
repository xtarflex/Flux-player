<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import Icon from "../ui/Icon.svelte";
  import { checkForUpdates } from "../../utils/version";
  import { hasUpdateAvailable } from "../../stores/ui";
  import { confirm } from '@tauri-apps/plugin-dialog';

  let osInfo = $state('Detecting...');
  let appVersion = import.meta.env.VITE_APP_VERSION; 
  let checking = $state(false);
  let resetting = $state(false);
  let uninstalling = $state(false);

  onMount(() => {
    // Using standard web APIs to avoid extra Tauri plugin dependencies
    const platform = navigator.platform.toLowerCase();
    if (platform.includes('win')) osInfo = 'Windows';
    else if (platform.includes('mac')) osInfo = 'macOS';
    else if (platform.includes('linux')) osInfo = 'Linux';
    else osInfo = platform || 'Unknown OS';
  });

  async function handleCheckUpdates() {
    checking = true;
    try {
      await checkForUpdates();
      if (!$hasUpdateAvailable) {
        window.dispatchEvent(new CustomEvent('flux-toast', {
          detail: { label: 'You are on the latest beta build', icon: 'check' }
        }));
      }
    } catch (err) {
      console.error('Update check failed:', err);
    } finally {
      checking = false;
    }
  }

  function downloadLatest() {
    openUrl('https://github.com/xtarflex/flux-player/releases');
  }

  async function handleResetSettings() {
    const isConfirmed = await confirm(
      'This will revert appearance, username, shortcuts, and TMDB keys to defaults. Your Media Library and Database will be KEPT. Continue?',
      { title: 'Reset App Settings', kind: 'warning' }
    );

    if (isConfirmed) {
      resetting = true;
      try {
        await invoke('factory_reset'); // This now only clears settings table
        
        // Clear frontend persistence to prevent re-syncing old settings back to backend
        localStorage.removeItem('flux_settings');
        
        window.dispatchEvent(new CustomEvent('flux-toast', {
          detail: { label: 'Settings reset. Refreshing...', icon: 'check' }
        }));
        // Soft refresh as requested by user
        setTimeout(() => window.location.reload(), 1500);
      } catch (err) {
        console.error('Reset failed:', err);
      } finally {
        resetting = false;
      }
    }
  }

  async function handleUninstall() {
    const isConfirmed = await confirm(
      'Are you sure you want to uninstall Flux? This will launch the uninstaller.',
      { title: 'Uninstall Flux', kind: 'error' }
    );

    if (isConfirmed) {
      uninstalling = true;
      try {
        await invoke('open_uninstaller');
      } catch (err) {
        console.error('Uninstall failed:', err);
        window.dispatchEvent(new CustomEvent('flux-toast', {
          detail: { label: 'Uninstaller not found. Use Windows Settings.', icon: 'error' }
        }));
      } finally {
        uninstalling = false;
      }
    }
  }

  function openGitHub() {
    openUrl('https://github.com/xtarflex/Flux-player/issues/new/choose');
  }

  function reportBug() {
    const title = encodeURIComponent('[BUG] ');
    const body = encodeURIComponent(
      `## Bug Description\n\n\n## Steps to Reproduce\n1.\n2.\n3.\n\n## Environment\n- **OS:** ${osInfo}\n- **App Version:** ${appVersion}\n- **Metadata Provider:** TMDB`
    );
    openUrl(`https://github.com/xtarflex/Flux-player/issues/new?title=${title}&body=${body}`);
  }

  async function copySystemInfo() {
    try {
      const report = await invoke<{
        app_version: string;
        os: string;
        computer_name: string;
        library_total: number;
        library_enriched: number;
        library_pending: number;
        offline_mode: boolean;
        auto_indexing: boolean;
        has_custom_tmdb_key: boolean;
        theme: string;
      }>('get_diagnostic_report');

      const info = [
        `## Flux Diagnostic Report`,
        `| Key | Value |`,
        `|---|---|`,
        `| App Version | ${report.app_version} |`,
        `| OS | ${report.os} |`,
        `| Device | ${report.computer_name} |`,
        `| Library | ${report.library_total} items (${report.library_enriched} enriched, ${report.library_pending} pending) |`,
        `| Offline Mode | ${report.offline_mode ? 'Yes' : 'No'} |`,
        `| Auto Indexing | ${report.auto_indexing ? 'Yes' : 'No'} |`,
        `| Custom TMDB Key | ${report.has_custom_tmdb_key ? 'Yes' : 'No'} |`,
        `| Theme | ${report.theme} |`,
        `| Locale | ${navigator.language} |`,
      ].join('\n');

      await navigator.clipboard.writeText(info);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Full Diagnostic Copied', icon: 'check' }
      }));
    } catch (err) {
      console.error("Failed to copy diagnostics:", err);
      // Fallback to basic info
      const info = `Flux Version: ${appVersion}\nOS: ${osInfo}\nLocale: ${navigator.language}`;
      await navigator.clipboard.writeText(info);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Basic Info Copied', icon: 'check' }
      }));
    }
  }
</script>

<div class="support-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="help" size={24} class="header-icon" />
      <div>
        <h2>Support & Community</h2>
        <p>Get help, report issues, and help shape the future of Flux.</p>
      </div>
    </div>
  </header>

    <div class="support-grid-columns">
      <div class="primary-column">
        <section class="support-card featured">
          <div class="card-background-glow">
            <div class="card-glow"></div>
          </div>
          
          <div class="text-stack">
            <h3>Found a bug?</h3>
            <p>The Flux Beta relies on user feedback. Reporting issues helps us build a more stable experience for everyone.</p>
          </div>
          

          <div class="featured-actions">
            <button class="btn-primary report-btn" onclick={reportBug}>
              <Icon name="github" size={18} />
              Report Issue on GitHub
            </button>
          </div>
        </section>

        <!-- Software Updates Section -->
        <section class="settings-card update-card">
          <div class="card-header">
            <Icon name="rocket" size={20} />
            <h3>Software Updates</h3>
          </div>
          <div class="update-content">
            <p>Check for the latest beta builds and cinematic improvements. Flux evolves every day.</p>
            
            {#if $hasUpdateAvailable}
              <div class="update-alert">
                <Icon name="star" size={16} />
                <span>Modern updates are waiting for your arrival!</span>
              </div>
            {/if}

            <div class="action-row-buttons">
              <button class="btn-check" onclick={handleCheckUpdates} disabled={checking}>
                {checking ? 'Checking...' : 'Check now'}
              </button>
              {#if $hasUpdateAvailable}
                <button class="btn-download" onclick={downloadLatest}>
                  Download Alpha
                </button>
              {/if}
            </div>
          </div>
        </section>

        <!-- Advanced Maintenance Section -->
        <section class="settings-card maintenance-card">
          <div class="card-header">
            <Icon name="tools" size={20} />
            <h3>Advanced Maintenance</h3>
          </div>
          <p>These actions are designed to resolve persistent UI issues or to clean up your installation environment.</p>
          <div class="advanced-actions">
            <div class="action-item">
              <div>
                <span class="action-label">Reset Application Preferences</span>
                <span class="action-sub">Reverts shortcuts, theme, and API keys. Keeps library data.</span>
              </div>
              <button class="btn-secondary-action" onclick={handleResetSettings} disabled={resetting}>
                {resetting ? 'Resetting...' : 'Reset Defaults'}
              </button>
            </div>
            <div class="action-item danger">
              <div>
                <span class="action-label">Uninstall Flux</span>
                <span class="action-sub">Permanently removes the application and its binaries.</span>
              </div>
              <button class="btn-danger-outline" onclick={handleUninstall} disabled={uninstalling}>
                {uninstalling ? 'Launching...' : 'Uninstall'}
              </button>
            </div>
          </div>
        </section>
      </div>

      <aside class="side-column">
        <section class="settings-card">
          <div class="card-header">
            <Icon name="community" size={20} />
            <h3>Community</h3>
          </div>
          <p>Join the movement, visit the labs, and stay updated on release notes.</p>
          <div class="action-row">
            <button class="btn-text" onclick={() => openUrl('https://github.com/xtarflex/Flux-player')}>Explore GitHub</button>
            <button class="btn-text" onclick={() => openUrl('https://discord.gg/fluxplayer')}>Official Discord</button>
          </div>
        </section>

        <section class="settings-card info-card">
          <div class="card-header">
            <Icon name="performance" size={20} />
            <h3>System Status</h3>
          </div>
          <div class="system-stats">
            <div class="stat">
              <span class="label">Flux Version</span>
              <span class="value">{appVersion} <span class="badge">BETA</span></span>
            </div>
            <div class="stat">
              <span class="label">OS Platform</span>
              <span class="value">{osInfo}</span>
            </div>
          </div>
          <button class="copy-info-btn" onclick={copySystemInfo}>
            <Icon name="copy" size={14} />
            Copy Debug Map
          </button>
        </section>

        <section class="settings-card about-card">
          <div class="card-header">
            <Icon name="info" size={20} />
            <h3>About Flux</h3>
          </div>
          <p>An experimental, high-performance media suite focused on high-fidelity aesthetics. Built with Svelte, Rust, and Love.</p>
          <div class="legal">
            <span>© 2026 Flux Team</span>
            <span class="dot">•</span>
            <button class="btn-text legal-btn" onclick={() => openUrl('https://github.com/xtarflex/Flux-player/blob/main/LICENSE')}>GPL-3.0 License</button>
          </div>
        </section>
      </aside>
    </div>
</div>

<style>
  .support-grid-columns {
    display: grid;
    grid-template-columns: 1.5fr 1fr;
    gap: 1.5rem;
    align-items: start;
  }

  .primary-column, .side-column {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .settings-card {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    position: relative;
    overflow: hidden;
  }

  .settings-card:hover {
    border-color: var(--glass-border-mid);
    background: var(--glass-bg-mid);
  }

  .update-card p {
    margin-bottom: 0.5rem;
  }

  .update-alert {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px;
    background: rgba(0, 255, 255, 0.08);
    border: 1px solid rgba(0, 255, 255, 0.2);
    border-radius: 10px;
    color: var(--secondary);
    font-size: 0.85rem;
    font-weight: 600;
  }

  .action-row-buttons {
    display: flex;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .btn-check {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-check:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    border-color: var(--text-main);
  }

  .btn-download {
    background: var(--secondary);
    color: #000;
    border: none;
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 4px 15px rgba(0, 255, 255, 0.2);
  }

  .btn-download:hover {
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(0, 255, 255, 0.3);
  }

  /* Maintenance Section */
  .advanced-actions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .action-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1.5rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--glass-border-low);
  }

  .action-item:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .action-label {
    display: block;
    font-weight: 600;
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .action-sub {
    display: block;
    font-size: 0.75rem;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .btn-secondary-action {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .btn-secondary-action:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .btn-danger-outline {
    background: transparent;
    border: 1px solid rgba(255, 50, 50, 0.4);
    color: #ff5555;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    white-space: nowrap;
  }

  .btn-danger-outline:hover {
    background: rgba(255, 50, 50, 0.1);
    border-color: #ff3333;
  }

  /* Featured Card */
  .support-card.featured {
    position: relative;
    background: linear-gradient(135deg, rgba(138, 43, 226, 0.15) 0%, rgba(0, 255, 255, 0.08) 100%),
                radial-gradient(circle at 100% 0%, rgba(0, 255, 255, 0.1) 0%, transparent 40%);
    border: 1px solid var(--glass-border-mid);
    border-radius: 20px;
    padding: 2.5rem;
    overflow: hidden;
    min-height: 220px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    backdrop-filter: blur(20px);
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  }

  .support-card.featured::before {
    content: "";
    position: absolute;
    inset: 0;
    background: radial-gradient(circle at 2px 2px, rgba(255, 255, 255, 0.05) 1px, transparent 0);
    background-size: 24px 24px;
    opacity: 0.5;
    z-index: 1;
  }

  .card-background-glow {
    position: absolute;
    inset: 0;
    overflow: hidden;
    border-radius: 20px;
    pointer-events: none;
    z-index: 0;
  }

  .card-glow {
    position: absolute;
    top: -20%;
    right: -10%;
    width: 400px;
    height: 400px;
    background: radial-gradient(circle, var(--secondary) 0%, transparent 70%);
    filter: blur(80px);
    opacity: 0.15;
    z-index: 0;
  }

  .featured h3 {
    font-size: 1.6rem;
    margin: 0;
    color: var(--text-main);
    font-weight: 800;
    letter-spacing: -0.02em;
    position: relative;
    z-index: 2;
  }

  .text-stack {
    position: relative;
    z-index: 2;
  }

  .text-stack p {
    max-width: 440px;
    opacity: 0.8;
    line-height: 1.6;
    font-size: 1rem;
    margin-top: 0.75rem;
    color: var(--text-muted);
  }

  .featured-actions {
    display: flex;
    justify-content: flex-start;
    margin-top: 2rem;
    position: relative;
    z-index: 2;
  }

  .report-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 0.9rem 1.8rem;
    background: var(--secondary);
    color: #000;
    border: none;
    border-radius: 12px;
    font-weight: 700;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .report-btn:hover {
    transform: translateY(-2px);
    background: #fff;
    box-shadow: 0 10px 25px rgba(0, 255, 255, 0.2);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--secondary);
    margin-bottom: 0.5rem;
  }

  .card-header h3 {
    font-size: 0.95rem;
    margin: 0;
    color: var(--text-main);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  .action-row {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .btn-text {
    background: none;
    border: none;
    color: var(--secondary);
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    padding: 0;
    text-decoration: underline;
    opacity: 0.8;
  }

  .btn-text:hover {
    opacity: 1;
  }

  /* System Info Card */
  .system-stats {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin: 0.5rem 0;
  }

  .stat {
    display: flex;
    justify-content: space-between;
    font-size: 0.85rem;
  }

  .stat .label { color: var(--text-muted); }
  .stat .value { color: var(--text-main); font-family: monospace; }

  .badge {
    font-size: 0.6rem;
    background: var(--secondary);
    color: #000;
    padding: 1px 4px;
    border-radius: 3px;
    font-weight: 800;
  }

  .copy-info-btn {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-muted);
    border-radius: 8px;
    padding: 8px;
    font-size: 0.75rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    transition: all 0.2s;
  }

  .copy-info-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-main);
  }

  .legal {
    display: flex;
    gap: 8px;
    font-size: 0.7rem;
    color: var(--text-muted);
    margin-top: 0.5rem;
  }

  .legal button.legal-btn {
    color: var(--text-muted);
    text-decoration: none;
  }

  .legal button.legal-btn:hover {
    text-decoration: underline;
  }

  .dot { opacity: 0.5; }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @media (max-width: 900px) {
    .support-grid-columns { grid-template-columns: 1fr; }
  }
</style>
