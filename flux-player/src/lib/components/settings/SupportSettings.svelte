<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import Icon from "../ui/Icon.svelte";

  let osInfo = $state('Detecting...');
  let appVersion = import.meta.env.VITE_APP_VERSION; 

  onMount(() => {
    // Using standard web APIs to avoid extra Tauri plugin dependencies
    const platform = navigator.platform.toLowerCase();
    if (platform.includes('win')) osInfo = 'Windows';
    else if (platform.includes('mac')) osInfo = 'macOS';
    else if (platform.includes('linux')) osInfo = 'Linux';
    else osInfo = platform || 'Unknown OS';
  });

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
    const info = `Flux Version: ${appVersion}\nOS: ${osInfo}\nLocale: ${navigator.language}`;
    try {
      await navigator.clipboard.writeText(info);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'System Info Copied', icon: 'check' }
      }));
    } catch (err) {
      console.error("Failed to copy:", err);
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

  <div class="settings-grid">
    <section class="support-card featured">
      <div class="card-glow"></div>
      <div class="card-content">
        <div class="text-stack">
          <h3>Found a bug?</h3>
          <p>The Flux Beta relies on user feedback. Reporting issues helps us build a more stable experience for everyone.</p>
        </div>
        <button class="btn-primary report-btn" onclick={reportBug}>
          <Icon name="bug" size={18} />
          Report Issue on GitHub
        </button>
      </div>
    </section>

    <div class="info-grid">
      <section class="settings-card">
        <div class="card-header">
          <Icon name="discovery" size={20} />
          <h3>Community</h3>
        </div>
        <p>Join our Discord or visit the GitHub repository to stay updated on the latest features and release notes.</p>
        <div class="action-row">
          <button class="btn-text" onclick={() => openUrl('https://github.com/xtarflex/Flux-player')}>Visit GitHub</button>
          <button class="btn-text" onclick={() => openUrl('https://discord.gg/fluxplayer')}>Join Discord</button>
        </div>
      </section>

      <section class="settings-card info-card">
        <div class="card-header">
          <Icon name="performance" size={20} />
          <h3>System Context</h3>
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
          Copy Debug Info
        </button>
      </section>
    </div>

    <section class="settings-card about-card">
      <div class="card-header">
        <h3>About Flux</h3>
      </div>
      <p>Flux is an experimental, high-performance media player focused on high-fidelity aesthetics and seamless library management. Built with Svelte, Rust, and love.</p>
      <div class="legal">
        <span>© 2026 Flux Team</span>
        <span class="dot">•</span>
        <button class="btn-text legal-btn" onclick={() => openUrl('https://github.com/xtarflex/Flux-player/blob/main/LICENSE')}>License</button>
      </div>
    </section>
  </div>
</div>

<style>
  .support-settings {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    animation: fadeIn 0.4s ease-out;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding-bottom: 2rem;
    border-bottom: 1px solid var(--glass-border-low);
  }

  .header-content {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  :global(.header-icon) {
    color: var(--secondary);
  }

  h2 {
    font-family: var(--font-heading);
    font-size: 1.5rem;
    margin: 0;
    letter-spacing: 0.05em;
  }

  p {
    color: var(--text-muted);
    margin: 0.5rem 0 0;
    font-size: 0.9rem;
    line-height: 1.5;
  }

  .settings-grid {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 800px;
  }

  /* Featured Card */
  .support-card.featured {
    position: relative;
    background: linear-gradient(135deg, rgba(138, 43, 226, 0.1) 0%, rgba(0, 255, 255, 0.05) 100%);
    border: 1px solid var(--glass-border-mid);
    border-radius: 20px;
    padding: 2rem;
    overflow: hidden;
  }

  .card-glow {
    position: absolute;
    top: -50%;
    right: -20%;
    width: 300px;
    height: 300px;
    background: var(--secondary);
    filter: blur(100px);
    opacity: 0.15;
    pointer-events: none;
  }

  .card-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 2rem;
    position: relative;
    z-index: 1;
  }

  .featured h3 {
    font-size: 1.4rem;
    margin: 0;
    color: var(--text-main);
  }

  .report-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 1rem 1.5rem;
    background: var(--secondary);
    color: #000;
    border: none;
    border-radius: 12px;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    white-space: nowrap;
  }

  .report-btn:hover {
    transform: scale(1.05) translateY(-2px);
    box-shadow: 0 8px 25px rgba(0, 255, 255, 0.3);
  }

  /* Secondary Cards */
  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
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
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--secondary);
  }

  .card-header h3 {
    font-size: 1rem;
    margin: 0;
    color: var(--text-main);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .action-row {
    display: flex;
    gap: 1.5rem;
    margin-top: auto;
  }

  .btn-text {
    background: none;
    border: none;
    color: var(--secondary);
    font-weight: 600;
    font-size: 0.9rem;
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

  /* About Card */
  .about-card {
    text-align: center;
    padding: 2.5rem;
  }

  .legal {
    display: flex;
    justify-content: center;
    gap: 10px;
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 1rem;
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

  @media (max-width: 600px) {
    .info-grid { grid-template-columns: 1fr; }
    .card-content { flex-direction: column; text-align: center; }
  }
</style>
