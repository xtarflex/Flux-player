<script lang="ts">
  import Icon from "../ui/Icon.svelte";
  import { open } from '@tauri-apps/plugin-dialog';

  let storageLocations = $state([
    { path: "D:/Media/Movies", type: "video" },
    { path: "D:/Media/TV Shows", type: "video" },
    { path: "C:/Users/Flux/Music", type: "audio" }
  ]);

  let cacheSize = "1.2 GB";
  let dbSize = "45 MB";

  async function addFolder() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Media Folder"
      });

      if (selected) {
        // Just push to UI state for now
        storageLocations.push({ path: selected as string, type: "video" }); // Default to video for now
      }
    } catch (err) {
      console.error("Failed to open dialog", err);
      // Fallback for browser testing
      window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Cannot open dialog in browser', icon: 'error' } }));
    }
  }

  function removeFolder(index: number) {
    storageLocations.splice(index, 1);
  }

  function openFolderDialog() {
    addFolder();
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
        <button class="btn-primary" onclick={openFolderDialog}>
          <Icon name="plus" size={16} /> Add Folder
        </button>
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
          <h3>SQLite Database</h3>
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
</style>
