<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { settings, updateSetting } from '$lib/stores/settings';
  import Icon from "../ui/Icon.svelte";
  import Dropdown from '../ui/Dropdown.svelte';
  import Toggle from '../ui/Toggle.svelte';
  
  let tmdbKey = $state('');
  let streamingQuality = $state('Best Available');
  let isCheatSheetOpen = $state(false);

  const qualities = ['720p', '1080p', '4K', 'Best Available'];

  onMount(async () => {
    try {
      const savedKey = await invoke<string | null>('get_setting', { key: 'tmdb_read_token' });
      if (savedKey) tmdbKey = savedKey;
    } catch (err) {
      console.error("Failed to load TMDB key:", err);
    }
  });

  async function saveToken() {
    try {
      await invoke('save_setting', { key: 'tmdb_read_token', value: tmdbKey });
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'TMDB Token Saved', icon: 'settings' }
      }));
    } catch (err) {
      console.error("Failed to save TMDB token:", err);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Failed to save token', icon: 'error' }
      }));
    }
  }

  function toggleCheatSheet() {
    isCheatSheetOpen = !isCheatSheetOpen;
  }
</script>

<div class="streaming-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="streaming" size={24} class="header-icon" />
      <div>
        <h2>Streaming & Network</h2>
        <p>Configure external metadata providers and network behavior.</p>
      </div>
    </div>
  </header>

  <div class="settings-grid">
    <section class="settings-card">
      <div class="card-header">
        <h3>Connectivity</h3>
      </div>
      <div class="setting-row">
        <div class="setting-info">
          <span class="description">Enable Intentional Offline Mode to stop all network requests. Useful for saving bandwidth or preventing online data fetching.</span>
        </div>
        <Toggle 
          checked={$settings.offlineMode} 
          onchange={(e: boolean) => updateSetting('offlineMode', e)} 
          label="Offline Mode"
        />
      </div>
    </section>

    <section class="settings-card">
      <div class="card-header">
        <h3>External API Keys (TMDB)</h3>
        <p class="subtitle">Flux uses the TMDB API to fetch posters and metadata. Add your own Read Access Token (v4) for unlimited personal use.</p>
      </div>

      <div class="api-key-container">
        <div class="form-group">
          <label for="tmdbKey">TMDB Read Access Token (v4)</label>
          <div class="input-with-button">
            <input
              type="password"
              id="tmdbKey"
              bind:value={tmdbKey}
              placeholder="eyJhbGciOiJIUzI1NiJ9..."
            />
            <button class="btn-primary" onclick={saveToken}>Save Token</button>
          </div>
        </div>

        <button class="btn-text" onclick={toggleCheatSheet}>Need help registering? View Cheat Sheet</button>
      </div>

      {#if isCheatSheetOpen}
        <div class="cheat-sheet glass">
          <h4>Registration Cheat Sheet</h4>
          <p>Use these exact values when creating your TMDB App to get approved instantly:</p>

          <div class="copy-field">
            <span class="label">App Name:</span>
            <code class="value">Flux Player Local</code>
          </div>

          <div class="copy-field">
            <span class="label">App URL:</span>
            <code class="value">https://github.com/flux-player</code>
          </div>

          <div class="copy-field">
            <span class="label">Summary:</span>
            <code class="value">A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection.</code>
          </div>
        </div>
      {/if}
    </section>

    <section class="settings-card">
      <div class="card-header">
        <h3>Streaming Quality</h3>
      </div>
      <div class="setting-row">
        <div class="setting-info">
          <span class="description">Limit resolution to save bandwidth when streaming remote content.</span>
        </div>
        <Dropdown options={qualities} bind:value={streamingQuality} label="Maximum Quality Cap" />
      </div>
    </section>
  </div>
</div>

<style>
  .streaming-settings {
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
  }

  .settings-grid {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 800px;
  }

  .settings-card {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .card-header h3 {
    font-size: 1.1rem;
    margin: 0;
    color: var(--text-main);
  }

  .subtitle {
    font-size: 0.85rem;
    margin: 0.5rem 0 0 0;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .description {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
    line-height: 1.5;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 2rem;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  .api-key-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
  }

  label {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .input-with-button {
    display: flex;
    gap: 1rem;
  }

  input[type="password"] {
    flex: 1;
    background: var(--bg-base);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    font-family: monospace;
    font-size: 0.95rem;
    outline: none;
    transition: all 0.2s ease;
  }

  input[type="password"]:focus {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
  }

  .btn-primary {
    background: var(--secondary);
    color: var(--bg-base);
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-family: var(--font-body);
    font-weight: 700;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn-primary:hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
  }

  .btn-text {
    background: transparent;
    color: var(--secondary);
    border: none;
    padding: 0;
    font-size: 0.85rem;
    cursor: pointer;
    text-align: left;
    text-decoration: underline;
    align-self: flex-start;
    opacity: 0.8;
    transition: opacity 0.2s ease;
  }

  .btn-text:hover {
    opacity: 1;
  }

  .cheat-sheet {
    padding: 1.5rem;
    border-radius: 12px;
    border: 1px solid var(--glass-border-mid);
    background: rgba(0, 0, 0, 0.2);
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .cheat-sheet h4 {
    color: var(--secondary);
    margin: 0;
    font-size: 1rem;
  }

  .copy-field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .copy-field .label {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
  }

  .copy-field .value {
    background: rgba(0, 0, 0, 0.4);
    padding: 0.75rem;
    border-radius: 6px;
    font-family: monospace;
    font-size: 0.85rem;
    color: var(--text-main);
    border: 1px solid var(--glass-border-low);
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
