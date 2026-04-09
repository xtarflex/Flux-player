<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
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

  async function copyToClipboard(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: `${label} Copied`, icon: 'copy' }
      }));
    } catch (err) {
      console.error("Failed to copy:", err);
    }
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
        <div class="title-with-badge">
          <h3>TMDB API Metadata</h3>
          <span class="badge">Recommended</span>
        </div>
        <p class="subtitle">
          Flux uses <a href="https://www.themoviedb.org/" target="_blank" rel="noopener">The Movie Database</a> to download high-quality posters and metadata. 
          Linking your own personal account unlocks <strong>unlimited, high-priority fetches</strong> for your entire collection.
        </p>
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
        <div class="cheat-sheet glass" transition:fade>
          <div class="sheet-title">
            <Icon name="sparkle" size={18} />
            <h4>Registration Cheat Sheet</h4>
          </div>
          <p>Click any field below to copy it into the TMDB registration form:</p>

          <div class="copy-grid">
            <button class="copy-field interactive" onclick={() => copyToClipboard('Flux Player Local', 'App Name')}>
              <span class="label">App Name</span>
              <code class="value">Flux Player Local <Icon name="copy" size={12} /></code>
            </button>

            <button class="copy-field interactive" onclick={() => copyToClipboard('https://github.com/flux-player', 'App URL')}>
              <span class="label">App URL</span>
              <code class="value">https://github.com/flux-player <Icon name="copy" size={12} /></code>
            </button>

            <button class="copy-field interactive span-all" onclick={() => copyToClipboard('A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection.', 'Summary')}>
              <span class="label">Application Summary</span>
              <code class="value">A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection. <Icon name="copy" size={12} /></code>
            </button>
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

  .title-with-badge {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .badge {
    background: rgba(0, 255, 255, 0.1);
    color: var(--secondary);
    font-size: 0.65rem;
    padding: 2px 8px;
    border-radius: 40px;
    border: 1px solid rgba(0, 255, 255, 0.2);
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .cheat-sheet {
    padding: 2rem;
    border-radius: 20px;
    border: 1px solid var(--glass-border-mid);
    background: rgba(255, 255, 255, 0.02);
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
    margin-top: 0.5rem;
    animation: slideDown 0.3s ease-out;
  }

  .sheet-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    color: var(--secondary);
  }

  .sheet-title h4 {
    margin: 0;
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .copy-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.25rem;
  }

  .copy-field {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    background: none;
    border: none;
    padding: 0;
    text-align: left;
    width: 100%;
  }

  .copy-field.interactive {
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .copy-field.interactive:hover {
    transform: translateY(-2px);
  }

  .copy-field.span-all {
    grid-column: span 2;
  }

  .copy-field .label {
    font-size: 0.7rem;
    font-weight: 800;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .copy-field .value {
    background: rgba(0, 0, 0, 0.4);
    padding: 1rem;
    border-radius: 12px;
    font-family: monospace;
    font-size: 0.85rem;
    color: var(--text-main);
    border: 1px solid var(--glass-border-low);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    transition: all 0.2s ease;
  }

  .copy-field.interactive:hover .value {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
  }

  .subtitle a {
    color: var(--secondary);
    text-decoration: underline;
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
