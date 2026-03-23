<script lang="ts">
  let baseFolders = $state([
    { path: 'C:\\Users\\Default\\Videos', type: 'Videos' },
    { path: 'C:\\Users\\Default\\Music', type: 'Music' }
  ]);

  let autoIndexing = $state(true);
  let scanFrequency = $state(30); // minutes
  let experimentalSearch = $state(false);

  function removeFolder(index: number) {
    baseFolders = baseFolders.filter((_, i) => i !== index);
  }

  function addFolder() {
    // In a real implementation, this would open a directory picker via Tauri
    baseFolders = [...baseFolders, { path: 'C:\\New\\Media\\Folder', type: 'Mixed' }];
  }
</script>

<div class="settings-section">
  <h2>Storage & Library</h2>

  <div class="card">
    <div class="card-header">
      <h3>Base Folders Management</h3>
      <div class="actions">
        <button class="btn-primary" onclick={addFolder}>+ Add Folder</button>
        <button class="btn-outline">Refresh Now</button>
      </div>
    </div>
    <p class="description">Select the local directories where your media is stored.</p>

    <div class="folder-list">
      {#each baseFolders as folder, i}
        <div class="folder-item">
          <div class="folder-icon">📁</div>
          <div class="folder-details">
            <span class="folder-path">{folder.path}</span>
            <span class="folder-type">{folder.type}</span>
          </div>
          <button class="btn-icon" aria-label="Remove folder" onclick={() => removeFolder(i)}>×</button>
        </div>
      {/each}

      {#if baseFolders.length === 0}
        <div class="empty-state">
          No folders added. Your library is empty.
        </div>
      {/if}
    </div>
  </div>

  <div class="card">
    <h3>Auto-Indexing</h3>
    <div class="setting-row">
      <div class="setting-info">
        <label for="autoIndexing">Enable Auto-Indexing</label>
        <span class="description">Automatically scan your base folders for new content.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="autoIndexing" bind:checked={autoIndexing} />
        <span class="slider round"></span>
      </label>
    </div>

    {#if autoIndexing}
      <div class="setting-row sub-setting">
        <div class="setting-info">
          <label for="scanFrequency">Scan Frequency</label>
          <span class="description">Every {scanFrequency} minutes</span>
        </div>
        <div class="slider-container">
          <input type="range" id="scanFrequency" min="5" max="120" step="5" bind:value={scanFrequency} />
          <div class="slider-marks">
            <span>5m</span>
            <span>60m</span>
            <span>120m</span>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <div class="card">
    <h3>Windows Indexing Integration</h3>
    <div class="setting-row">
      <div class="setting-info">
        <label for="experimentalSearch">Experimental Native Search</label>
        <span class="description">Use the Windows Search Index instead of a recursive file scan. May improve performance on very large drives.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="experimentalSearch" bind:checked={experimentalSearch} />
        <span class="slider round"></span>
      </label>
    </div>
  </div>
</div>

<style>
  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 800px;
  }

  h2 {
    color: var(--primary);
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-light);
    padding-bottom: 1rem;
  }

  h3 {
    color: var(--text-main);
    font-size: 1.1rem;
    margin: 0;
  }

  .card {
    background: var(--bg-surface);
    padding: 2rem;
    border-radius: 12px;
    border: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .actions {
    display: flex;
    gap: 1rem;
  }

  .description {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
  }

  .btn-primary {
    background: var(--primary);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font-body);
    font-weight: 500;
    transition: background 0.2s ease;
  }

  .btn-primary:hover {
    background: #9b4dff;
  }

  .btn-outline {
    background: transparent;
    color: var(--text-main);
    border: 1px solid var(--border-light);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font-body);
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .btn-outline:hover {
    border-color: var(--primary);
    color: var(--primary);
  }

  .folder-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .folder-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    background: var(--bg-base);
    padding: 1rem;
    border-radius: 8px;
    border: 1px solid var(--border-light);
  }

  .folder-icon {
    font-size: 1.5rem;
  }

  .folder-details {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .folder-path {
    font-family: monospace;
    color: var(--text-main);
    font-size: 0.95rem;
  }

  .folder-type {
    font-size: 0.8rem;
    color: var(--primary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-family: var(--font-heading);
    margin-top: 0.25rem;
  }

  .btn-icon {
    background: transparent;
    color: var(--text-muted);
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0 0.5rem;
    line-height: 1;
    transition: color 0.2s ease;
  }

  .btn-icon:hover {
    color: #ff4444;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--text-muted);
    font-style: italic;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .sub-setting {
    padding-top: 1rem;
    border-top: 1px dashed var(--border-light);
    margin-top: -0.5rem;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  label {
    font-weight: 500;
    color: var(--text-main);
  }

  /* Slider Container */
  .slider-container {
    flex: 1;
    max-width: 300px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  input[type="range"] {
    -webkit-appearance: none;
    width: 100%;
    background: transparent;
  }

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    height: 16px;
    width: 16px;
    border-radius: 50%;
    background: var(--primary);
    cursor: pointer;
    margin-top: -6px; /* You need to specify a margin in Chrome, but in Firefox and IE it is automatic */
  }

  input[type="range"]::-webkit-slider-runnable-track {
    width: 100%;
    height: 4px;
    cursor: pointer;
    background: var(--border-light);
    border-radius: 2px;
  }

  input[type="range"]:focus::-webkit-slider-runnable-track {
    background: var(--primary);
  }

  .slider-marks {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  /* The switch - the box around the slider */
  .switch {
    position: relative;
    display: inline-block;
    width: 50px;
    height: 24px;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--border-light);
    -webkit-transition: .4s;
    transition: .4s;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    -webkit-transition: .4s;
    transition: .4s;
  }

  input:checked + .slider {
    background-color: var(--primary);
  }

  input:focus + .slider {
    box-shadow: 0 0 1px var(--primary);
  }

  input:checked + .slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
  }

  .slider.round {
    border-radius: 24px;
  }

  .slider.round:before {
    border-radius: 50%;
  }
</style>
