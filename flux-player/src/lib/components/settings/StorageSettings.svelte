<script lang="ts">
  import { onMount } from 'svelte';
  import Icon from "../ui/Icon.svelte";
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { loadLibraryFromDb, isScanning } from '$lib/stores/media';

  type FolderEntry = { path: string; type: 'video' | 'audio' };

  let storageLocations = $state<FolderEntry[]>([]);
  let autoIndexing = $state(true);
  let scanFrequency = $state(30); // minutes
  let isLoading = $state(true);
  let cacheSize = $state("—");
  let dbSize = $state("—");
  let scanProgress = $state({ current: 0, total: 0 });
  let healProgress = $state({ current: 0, total: 0 });
  let isHealing = $state(false);

  onMount(() => {
    let unlistenScan: () => void;
    let unlistenHeal: () => void;

    async function init() {
      try {
        // 1. Load Persistence
        const savedFolders = await invoke<string | null>('get_setting', { key: 'library_folders' });
        const savedAuto = await invoke<string | null>('get_setting', { key: 'auto_indexing' });
        const savedFreq = await invoke<string | null>('get_setting', { key: 'scan_frequency' });

        if (savedFolders) {
          storageLocations = JSON.parse(savedFolders);
        } else {
          const defaults = await invoke<FolderEntry[]>('get_default_media_folders');
          storageLocations = defaults;
          await syncFolders(defaults);
        }

        if (savedAuto) autoIndexing = savedAuto === 'true';
        if (savedFreq) scanFrequency = parseInt(savedFreq);

      } catch (e) {
        console.warn('Flux Storage: Persistence load failed:', e);
      } finally {
        isLoading = false;
      }

      // 2. Listen for Progress Updates
      unlistenScan = await listen('flux-scan-progress', (event: any) => {
        const [current, total] = event.payload;
        scanProgress = { current, total };
      });

      unlistenHeal = await listen('flux-heal-progress', (event: any) => {
        const [current, total] = event.payload;
        healProgress = { current, total };
      });
    }

    init();

    return () => {
      if (unlistenScan) unlistenScan();
      if (unlistenHeal) unlistenHeal();
    };
  });

  async function syncFolders(folders: FolderEntry[]) {
    try {
      await invoke('save_setting', { key: 'library_folders', value: JSON.stringify(folders) });
    } catch (e) {
      console.error('Failed to sync folders:', e);
    }
  }

  async function updateAutoIndexing(val: boolean) {
    autoIndexing = val;
    await invoke('save_setting', { key: 'auto_indexing', value: val.toString() });
  }

  async function updateFrequency(val: number) {
    scanFrequency = val;
    await invoke('save_setting', { key: 'scan_frequency', value: val.toString() });
  }

  async function addFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Media Folder"
      });

      if (selected && typeof selected === 'string') {
        const newFolders: FolderEntry[] = [...storageLocations, { path: selected, type: "video" }];
        storageLocations = newFolders;
        await syncFolders(newFolders);
        
        // Kick off the scan and refresh the library
        isScanning.set(true);
        // Global scanning state handles feedback

        try {
          await invoke('start_library_scan', { dir: selected });
          await loadLibraryFromDb();
          window.dispatchEvent(new CustomEvent('flux-toast', { 
            detail: { label: 'Library updated', icon: 'library' } 
          }));
        } catch (scanErr) {
          console.error('Scan failed:', scanErr);
          window.dispatchEvent(new CustomEvent('flux-toast', { 
            detail: { label: 'Scan failed', icon: 'error' } 
          }));
        } finally {
          isScanning.set(false);
        }
      }
    } catch (err) {
      console.error("Failed to open dialog", err);
      window.dispatchEvent(new CustomEvent('flux-toast', { 
        detail: { label: 'Cannot open dialog in browser', icon: 'error' } 
      }));
    }
  }

  async function removeFolder(index: number) {
    const newFolders = storageLocations.filter((_, i) => i !== index);
    storageLocations = newFolders;
    await syncFolders(newFolders);
  }

  function openFolderDialog() {
    addFolder();
  }

  async function refreshAll() {
    if ($isScanning) return;
    
    isScanning.set(true);
    isHealing = true;
    scanProgress = { current: 0, total: 0 };
    healProgress = { current: 0, total: 0 };

    try {
      // 1. Heal the backlog (Offline-to-Online sync)
      await invoke('heal_library');
      isHealing = false;
      
      // 2. Scan all configured folders for new media
      for (const folder of storageLocations) {
        await invoke('start_library_scan', { dir: folder.path });
      }
      
      // 3. Refresh the UI
      await loadLibraryFromDb();
      
      window.dispatchEvent(new CustomEvent('flux-toast', { 
        detail: { label: 'Library Refresh Complete', icon: 'library' } 
      }));
    } catch (e) {
      console.error('Manual refresh failed:', e);
      window.dispatchEvent(new CustomEvent('flux-toast', { 
        detail: { label: 'Refresh failed', icon: 'error' } 
      }));
    } finally {
      isScanning.set(false);
    }
  }

  function formatFrequency(mins: number) {
    if (mins < 60) return `Every ${mins} minutes`;
    const h = Math.floor(mins / 60);
    const m = mins % 60;
    if (m === 0) return `Every ${h} hour${h > 1 ? 's' : ''}`;
    return `Every ${h}h ${m}m`;
  }
</script>

<div class="storage-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="library" size={24} class="header-icon" />
      <div>
        <h2>Storage & Library</h2>
        <p>Manage media folders, cache size, and database settings.</p>
      </div>
    </div>
  </header>

  <div class="settings-grid">
    <!-- Media Folders -->
    <section class="settings-card full-width">
      <div class="card-header">
        <div>
          <h3>Base Media Folders</h3>
          <p class="subtitle">Folders that Flux Player will automatically scan for media files.</p>
        </div>
        <div class="header-actions">
          <button class="btn-secondary refresh-btn" onclick={refreshAll} disabled={$isScanning}>
            {#if $isScanning}
              <div class="spinner-box">
                <svg class="flux-spinner" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path class="flux-spin-cyan" d="M12 22 A10 10 0 0 1 12 2 A5 5 0 0 1 12 12 A5 5 0 0 0 12 22 Z" fill="var(--secondary)"/>
                  <path class="flux-spin-violet" d="M12 2 A10 10 0 0 1 12 22 A5 5 0 0 1 12 12 A5 5 0 0 0 12 2 Z" fill="var(--primary)"/>
                </svg>
              </div>
              <span class="scanning-text">
                {#if isHealing && healProgress.total > 0}
                  Heal: {healProgress.current}/{healProgress.total}
                {:else if scanProgress.total > 0}
                  Scan: {scanProgress.current}/{scanProgress.total}
                {:else}
                  Working...
                {/if}
              </span>
            {:else}
              <Icon name="refresh" size={16} /> Refresh All
            {/if}
          </button>
          <button class="btn-primary" onclick={openFolderDialog} disabled={$isScanning}>
            <Icon name="plus" size={16} /> Add Folder
          </button>
        </div>
      </div>

      <div class="folder-list">
        {#each storageLocations as folder, index}
          <div class="folder-item">
            <div class="folder-info">
              <Icon name={folder.type === 'video' ? 'movie' : 'music'} size={18} class="folder-icon" />
              <span class="folder-path">{folder.path}</span>
            </div>
            <button class="btn-remove" onclick={() => removeFolder(index)}>
              <Icon name="close" size={16} />
            </button>
          </div>
        {/each}
        {#if storageLocations.length === 0}
          <div class="empty-state">No media folders added. Click "Add Folder" to begin scanning.</div>
        {/if}
      </div>

      <div class="indexing-controls">
        <div class="control-row">
          <div class="label-group">
            <span class="label">Auto-Indexing</span>
            <span class="sub">{autoIndexing ? 'Scanning active' : 'Manual refresh only'}</span>
          </div>
          <button 
            class="toggle-switch" 
            class:active={autoIndexing} 
            onclick={() => updateAutoIndexing(!autoIndexing)}
            aria-label="Toggle Auto-Indexing"
          ></button>
        </div>

        {#if autoIndexing}
          <div class="control-row frequency">
            <div class="label-group">
              <span class="label">Scan Frequency</span>
              <span class="sub">{formatFrequency(scanFrequency)}</span>
            </div>
            <input 
              type="range" 
              min="5" 
              max="1440" 
              step="10" 
              value={scanFrequency} 
              oninput={(e) => updateFrequency(parseInt(e.currentTarget.value))}
            />
          </div>
        {/if}
      </div>
    </section>

    <!-- Cache Management -->
    <section class="settings-card">
      <div class="card-header">
        <div>
          <h3>Image Cache</h3>
          <p class="subtitle">Offline posters, backdrops, and album art.</p>
        </div>
      </div>
      <div class="stat-row">
        <span class="stat-value">{cacheSize}</span>
        <button class="btn-secondary">Clear Cache</button>
      </div>
    </section>

    <!-- Database Management -->
    <section class="settings-card">
      <div class="card-header">
        <div>
          <h3>Flux Database</h3>
          <p class="subtitle">Metadata, playlists, and playback history.</p>
        </div>
      </div>
      <div class="stat-row">
        <span class="stat-value">{dbSize}</span>
        <div class="action-group">
          <button class="btn-secondary">Optimize</button>
          <button class="btn-danger">Reset All</button>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .storage-settings {
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
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
  }

  .full-width {
    grid-column: 1 / -1;
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

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .card-header h3 {
    font-size: 1.1rem;
    margin: 0 0 0.25rem 0;
    color: var(--text-main);
  }

  .subtitle {
    font-size: 0.85rem;
    margin: 0;
  }

  .btn-primary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(0, 255, 255, 0.1);
    color: var(--secondary);
    border: 1px solid rgba(0, 255, 255, 0.3);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-primary:hover {
    background: rgba(0, 255, 255, 0.2);
    box-shadow: 0 0 15px rgba(0, 255, 255, 0.1);
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.05);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-secondary:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .btn-danger {
    background: rgba(255, 50, 50, 0.1);
    color: #ff5555;
    border: 1px solid rgba(255, 50, 50, 0.3);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-danger:hover {
    background: rgba(255, 50, 50, 0.2);
  }

  /* Folder List */
  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .folder-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--glass-border-low);
    padding: 0.75rem 1rem;
    border-radius: 8px;
  }

  .folder-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  :global(.folder-icon) {
    color: var(--text-muted);
  }

  .folder-path {
    font-family: monospace;
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .btn-remove {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.4rem;
    border-radius: 6px;
    transition: all 0.2s ease;
  }

  .btn-remove:hover {
    background: rgba(255, 50, 50, 0.1);
    color: #ff5555;
  }

  .empty-state {
    padding: 2rem;
    text-align: center;
    color: var(--text-muted);
    font-style: italic;
    background: rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    border: 1px dashed var(--glass-border-mid);
  }

  /* Indexing Controls */
  .indexing-controls {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--glass-border-low);
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .control-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label-group {
    display: flex;
    flex-direction: column;
  }

  .label-group .label {
    font-weight: 600;
    font-size: 0.95rem;
    color: var(--text-main);
  }

  .label-group .sub {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .toggle-switch {
    width: 44px;
    height: 22px;
    background: var(--glass-bg-high);
    border: 1px solid var(--glass-border-mid);
    border-radius: 11px;
    position: relative;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .toggle-switch::after {
    content: '';
    position: absolute;
    left: 2px;
    top: 2px;
    width: 16px;
    height: 16px;
    background: var(--text-muted);
    border-radius: 50%;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .toggle-switch.active {
    background: rgba(0, 255, 255, 0.15);
    border-color: var(--secondary);
  }

  .toggle-switch.active::after {
    left: calc(100% - 18px);
    background: var(--secondary);
    box-shadow: 0 0 8px rgba(0, 255, 255, 0.5);
  }

  input[type="range"] {
    -webkit-appearance: none;
    appearance: none;
    width: 200px;
    height: 4px;
    background: var(--glass-bg-high);
    border-radius: 2px;
    outline: none;
  }

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    background: var(--secondary);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.4);
    transition: transform 0.2s ease;
  }

  input[type="range"]::-webkit-slider-thumb:hover {
    transform: scale(1.2);
  }

  /* Stat Rows */
  .stat-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 1rem;
    border-top: 1px solid var(--glass-border-low);
  }

  .stat-value {
    font-size: 1.5rem;
    font-family: var(--font-heading);
    font-weight: 700;
    color: var(--primary);
  }

  .action-group {
    display: flex;
    gap: 0.5rem;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .btn-primary:disabled, .btn-secondary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  .refresh-btn {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-width: 140px;
    justify-content: center;
  }

  .scanning-text {
    font-size: 0.75rem;
    font-weight: 700;
    font-family: monospace;
    opacity: 0.9;
  }

  /* Unified Island Spinner */
  .spinner-box {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  @keyframes flux-spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  @keyframes flux-breathe-cyan {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(-1px, -1px) scale(0.9); opacity: 1; }
  }
  @keyframes flux-breathe-violet {
    0%, 100% { transform: translate(0, 0) scale(1); opacity: 0.9; }
    50% { transform: translate(1px, 1px) scale(0.9); opacity: 1; }
  }

  .flux-spinner {
    width: 100%;
    height: 100%;
    animation: flux-spin 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
  }
  .flux-spin-cyan {
    animation: flux-breathe-cyan 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }
  .flux-spin-violet {
    animation: flux-breathe-violet 1.6s cubic-bezier(0.34, 1.56, 0.64, 1) infinite;
    transform-origin: center;
  }
</style>
