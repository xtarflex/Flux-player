<script lang="ts">
  let tmdbKey = $state('');
  let streamingQuality = $state('Best Available');
  let isCheatSheetOpen = $state(false);

  const qualities = ['720p', '1080p', '4K', 'Best Available'];

  function toggleCheatSheet() {
    isCheatSheetOpen = !isCheatSheetOpen;
  }
</script>

<div class="settings-section">
  <h2>Streaming & Network</h2>

  <div class="card">
    <h3>External API Keys (TMDB)</h3>
    <p class="description">
      Flux uses the TMDB API to fetch posters and metadata.
      By default, it uses a shared key pool (limited to 150 calls).
      Add your own Read Access Token (v4) for unlimited personal use.
    </p>

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
          <button class="btn-primary">Save Token</button>
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
  </div>

  <div class="card">
    <h3>Streaming Quality</h3>
    <div class="setting-row">
      <div class="setting-info">
        <label for="streamingQuality">Maximum Quality Cap</label>
        <span class="description">Limit resolution to save bandwidth when streaming remote content.</span>
      </div>
      <select id="streamingQuality" bind:value={streamingQuality}>
        {#each qualities as quality}
          <option value={quality}>{quality}</option>
        {/each}
      </select>
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

  h4 {
    color: var(--secondary);
    font-size: 0.95rem;
    margin-bottom: 0.5rem;
    letter-spacing: 0.05em;
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

  .description {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
    line-height: 1.5;
  }

  .api-key-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-weight: 500;
    color: var(--text-main);
  }

  .input-with-button {
    display: flex;
    gap: 1rem;
  }

  input[type="password"] {
    flex: 1;
    background: var(--bg-base);
    color: var(--text-main);
    border: 1px solid var(--border-light);
    padding: 0.75rem 1rem;
    border-radius: 6px;
    font-family: monospace;
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.2s ease;
  }

  input[type="password"]:focus {
    border-color: var(--primary);
  }

  .btn-primary {
    background: var(--primary);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font-body);
    font-weight: 600;
    transition: background 0.2s ease;
    white-space: nowrap;
  }

  .btn-primary:hover {
    background: #9b4dff;
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
  }

  .btn-text:hover {
    color: #fff;
  }

  /* Cheat Sheet */
  .cheat-sheet {
    padding: 1.5rem;
    border-radius: 8px;
    border: 1px solid rgba(0, 255, 255, 0.2);
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1rem;
  }

  .cheat-sheet p {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 0;
  }

  .copy-field {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .copy-field .label {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .copy-field .value {
    background: rgba(0, 0, 0, 0.5);
    padding: 0.5rem 0.75rem;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.85rem;
    color: #e0e0e0;
    border: 1px solid rgba(255, 255, 255, 0.05);
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  select {
    background: var(--bg-base);
    color: var(--text-main);
    border: 1px solid var(--border-light);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-family: var(--font-body);
    font-size: 0.95rem;
    outline: none;
    cursor: pointer;
    min-width: 150px;
  }

  select:focus {
    border-color: var(--primary);
  }
</style>
