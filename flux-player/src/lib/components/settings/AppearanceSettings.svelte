<script lang="ts">
  let uiTheme = $state('Cyber Dark');

  const themes = [
    { name: 'Cyber Dark', description: 'Deep blacks with Electric Violet accents.', icon: '🌙' },
    { name: 'Neon High Contrast', description: 'Brighter edges for maximum legibility.', icon: '⚡' },
    { name: 'Adaptive Tint', description: 'Dynamically shifts colors based on playing media.', icon: '🎨' }
  ];
</script>

<div class="settings-section">
  <h2>Appearance & UI</h2>

  <div class="card">
    <h3>UI Theme</h3>
    <p class="description">Select the visual style that defines your Flux experience.</p>

    <div class="theme-grid">
      {#each themes as theme}
        <label class="theme-card {uiTheme === theme.name ? 'active' : ''}">
          <input type="radio" name="uiTheme" value={theme.name} bind:group={uiTheme} />
          <div class="theme-icon">{theme.icon}</div>
          <div class="theme-details">
            <span class="theme-name">{theme.name}</span>
            <span class="theme-desc">{theme.description}</span>
          </div>
          {#if uiTheme === theme.name}
            <div class="active-indicator">✓</div>
          {/if}
        </label>
      {/each}
    </div>
  </div>

  <div class="card immutable-info">
    <h3>Core Identity Elements</h3>
    <p class="description">Certain elements are locked to maintain the premium Flux aesthetic.</p>

    <ul class="locked-list">
      <li>
        <strong>Typography:</strong> Syncopate (Headings) & Outfit (Body) are core to the brand.
      </li>
      <li>
        <strong>Macro Layout:</strong> The CSS Grid is highly optimized for performance.
      </li>
      <li>
        <strong>Animation Timing:</strong> Physics are tuned for a consistent "zero-gravity" feel.
      </li>
    </ul>
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

  .description {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
    margin-top: -0.5rem;
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .theme-card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 12px;
    background: var(--bg-base);
    border: 2px solid var(--border-light);
    cursor: pointer;
    position: relative;
    transition: all 0.2s ease;
  }

  .theme-card:hover {
    border-color: rgba(138, 43, 226, 0.5); /* Semi-transparent primary */
    transform: translateY(-2px);
  }

  .theme-card.active {
    border-color: var(--primary);
    background: rgba(138, 43, 226, 0.05);
  }

  .theme-card input[type="radio"] {
    display: none;
  }

  .theme-icon {
    font-size: 2rem;
  }

  .theme-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .theme-name {
    font-weight: 600;
    color: var(--text-main);
    font-size: 1.05rem;
  }

  .theme-desc {
    font-size: 0.85rem;
    color: var(--text-muted);
    line-height: 1.4;
  }

  .active-indicator {
    position: absolute;
    top: 1rem;
    right: 1rem;
    width: 24px;
    height: 24px;
    background: var(--primary);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8rem;
    font-weight: bold;
  }

  .immutable-info {
    border-style: dashed;
    background: rgba(255, 255, 255, 0.02);
  }

  .locked-list {
    margin: 0;
    padding-left: 1.5rem;
    color: var(--text-muted);
    font-size: 0.9rem;
    line-height: 1.6;
  }

  .locked-list strong {
    color: var(--text-main);
    font-weight: 500;
  }
</style>
