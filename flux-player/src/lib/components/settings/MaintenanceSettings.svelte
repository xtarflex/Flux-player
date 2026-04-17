<script lang="ts">
  import Icon from "../ui/Icon.svelte";
  import { invoke } from '@tauri-apps/api/core';
  import { confirm } from '@tauri-apps/plugin-dialog';
  import { checkForUpdates } from "../../utils/version";
  import { hasUpdateAvailable } from "../../stores/ui";
  import { openUrl } from '@tauri-apps/plugin-opener';

  let checking = $state(false);
  let resetting = $state(false);
  let uninstalling = $state(false);

  async function handleCheckUpdates() {
    checking = true;
    await checkForUpdates();
    checking = false;
    
    if (!$hasUpdateAvailable) {
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'You are on the latest beta build', icon: 'settings' }
      }));
    }
  }

  async function downloadLatest() {
    await openUrl('https://github.com/xtarflex/flux-player/releases');
  }

  async function handleFactoryReset() {
    const isConfirmed = await confirm(
      'This will delete your entire library database and poster cache. Your media files will NOT be touched. Continue?',
      { title: 'Factory Reset Flux', kind: 'warning' }
    );

    if (isConfirmed) {
      resetting = true;
      try {
        await invoke('factory_reset');
        window.dispatchEvent(new CustomEvent('flux-toast', {
          detail: { label: 'Library database wiped. Please restart Flux.', icon: 'error' }
        }));
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
          detail: { label: 'Could not find uninstaller. Please use Windows Settings.', icon: 'error' }
        }));
      } finally {
        uninstalling = false;
      }
    }
  }
</script>

<div class="maintenance-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="settings" size={24} class="header-icon" />
      <div>
        <h2>Maintenance & Support</h2>
        <p>Beta version management and troubleshooting utilities.</p>
      </div>
    </div>
  </header>

  <div class="settings-grid">
    <!-- Version & Updates -->
    <section class="maintenance-card">
      <div class="card-header">
        <Icon name="playback" size={20} />
        <h3>Software Updates</h3>
      </div>
      <p class="card-desc">Check for the latest beta builds and cinematic improvements.</p>
      
      {#if $hasUpdateAvailable}
        <div class="update-alert">
          <Icon name="star" size={16} />
          <span>A new version is available on GitHub!</span>
        </div>
      {/if}

      <div class="actions">
        <button class="btn-check" onclick={handleCheckUpdates} disabled={checking}>
          {checking ? 'Checking...' : 'Check now'}
        </button>
        {#if $hasUpdateAvailable}
          <button class="btn-download" onclick={downloadLatest}>
            Download latest
          </button>
        {/if}
      </div>
    </section>

    <!-- Danger Zone: Reset -->
    <section class="maintenance-card danger">
      <div class="card-header">
        <Icon name="error" size={20} />
        <h3>Data Reset</h3>
      </div>
      <p class="card-desc">Wipe all local metadata, posters, and indexing data. Media files stay safe.</p>
      <div class="actions">
        <button class="btn-danger" onclick={handleFactoryReset} disabled={resetting}>
          {resetting ? 'Resetting...' : 'Factory Reset'}
        </button>
      </div>
    </section>

    <!-- Danger Zone: Uninstall -->
    <section class="maintenance-card danger">
      <div class="card-header">
        <Icon name="delete" size={20} />
        <h3>Uninstall</h3>
      </div>
      <p class="card-desc">Completely remove Flux from your system.</p>
      <div class="actions">
        <button class="btn-danger-outline" onclick={handleUninstall} disabled={uninstalling}>
          {uninstalling ? 'Closing...' : 'Uninstall Flux'}
        </button>
      </div>
    </section>
  </div>
</div>

<style>
  .maintenance-settings {
    display: flex;
    flex-direction: column;
    gap: 2rem;
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
  }

  p {
    color: var(--text-muted);
    margin: 0.5rem 0 0;
    font-size: 0.9rem;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }

  .maintenance-card {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    transition: all 0.3s ease;
  }

  .maintenance-card:hover {
    border-color: var(--glass-border-mid);
    background: var(--glass-bg-mid);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text-main);
  }

  .card-header h3 {
    font-size: 1.1rem;
    margin: 0;
    font-weight: 700;
  }

  .card-desc {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .update-alert {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    background: rgba(0, 255, 255, 0.1);
    border: 1px solid rgba(0, 255, 255, 0.3);
    border-radius: 10px;
    color: var(--secondary);
    font-size: 0.85rem;
    font-weight: 500;
  }

  .actions {
    display: flex;
    gap: 10px;
    margin-top: auto;
    padding-top: 1rem;
  }

  button {
    padding: 0.7rem 1.2rem;
    border-radius: 10px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-check {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
  }

  .btn-check:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: var(--text-main);
  }

  .btn-download {
    background: var(--secondary);
    color: #000;
    border: none;
    box-shadow: 0 4px 15px rgba(0, 255, 255, 0.3);
  }

  .btn-download:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(0, 255, 255, 0.4);
  }

  .maintenance-card.danger {
    border-color: rgba(255, 50, 50, 0.1);
  }

  .maintenance-card.danger:hover {
    border-color: rgba(255, 50, 50, 0.3);
    background: rgba(255, 50, 50, 0.05);
  }

  .btn-danger {
    background: rgba(255, 50, 50, 0.15);
    color: #ff5555;
    border: 1px solid rgba(255, 50, 50, 0.3);
    width: 100%;
  }

  .btn-danger:hover {
    background: #ff3333;
    color: white;
  }

  .btn-danger-outline {
    background: transparent;
    color: #ff5555;
    border: 1px solid rgba(255, 50, 50, 0.5);
    width: 100%;
  }

  .btn-danger-outline:hover {
    background: rgba(255, 50, 50, 0.1);
    border-color: #ff3333;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
