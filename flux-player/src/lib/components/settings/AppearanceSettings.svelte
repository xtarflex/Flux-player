
<script lang="ts">
  import AnimatedPlayPause from '$lib/components/ui/animated-icons/AnimatedPlayPause.svelte';

  let uiTheme = $state('Cyber Dark');
  let isDemoPlaying = $state(false);

  const themes = [
    { name: 'Cyber Dark', description: 'Deep blacks with Cyan accents.', icon: '🌙' },
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
</div>

<style>
  .settings-section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 800px;
  }

  h2 {
    color: var(--secondary);
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid var(--glass-border-low);
    padding-bottom: 1rem;
  }

  h3 {
    color: var(--text-main);
    font-size: 1.1rem;
    margin: 0;
  }

  .card {
    background: var(--glass-bg-mid);
    padding: 2rem;
    border-radius: 12px;
    border: 1px solid var(--glass-border-low);
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
    background: var(--glass-bg-low);
    border: 2px solid var(--glass-border-mid);
    cursor: pointer;
    position: relative;
    transition: all 0.2s ease;
  }

  .theme-card:hover {
    border-color: var(--secondary-muted);
  }

  .theme-card.active {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
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
    background: var(--secondary); /* Standardized to Cyan */
    color: var(--bg-base);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.8rem;
    font-weight: bold;
  }
</style>
