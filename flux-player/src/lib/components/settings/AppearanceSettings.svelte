<script lang="ts">
  import Icon from "../ui/Icon.svelte";
  import { resetOnboarding } from "$lib/stores/onboarding";
  let uiTheme = $state('Cyber Dark');

  const themes = [
    { name: 'Cyber Dark', description: 'Deep blacks with Cyan accents.', icon: '🌙' },
    { name: 'Neon High Contrast', description: 'Brighter edges for maximum legibility.', icon: '⚡' },
    { name: 'Adaptive Tint', description: 'Dynamically shifts colors based on playing media.', icon: '🎨' }
  ];
</script>

<div class="appearance-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="appearance" size={24} class="header-icon" />
      <div>
        <h2>Appearance & UI</h2>
        <p>Personalize the visual identity and interface behavior of Flux.</p>
      </div>
    </div>
  </header>

  <div class="settings-grid">
    <section class="settings-card full-width">
      <div class="card-header">
        <div>
          <h3>UI Theme</h3>
          <p class="subtitle">Select the visual style that defines your Flux experience.</p>
        </div>
      </div>

      <div class="theme-grid">
        {#each themes as theme}
          <label class="theme-card" class:active={uiTheme === theme.name}>
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
    </section>

    <section class="settings-card full-width">
      <div class="card-header">
        <div>
          <h3>Onboarding & Tooltips</h3>
          <p class="subtitle">Manage the introductory tours and educational guides.</p>
        </div>
      </div>
      <div class="setting-row">
        <div class="setting-info">
          <span class="theme-name">Reset Onboarding Tours</span>
          <span class="theme-desc">Replay all the guided tours next time you visit the Library or Player.</span>
        </div>
        <button class="primary-btn" onclick={() => {
          resetOnboarding();
          window.dispatchEvent(new CustomEvent('flux-toast', { detail: { label: 'Tours Reset', icon: 'refresh' } }));
        }}>
          Reset Tours
        </button>
      </div>
    </section>
  </div>
</div>

<style>
  .appearance-settings {
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
    grid-template-columns: 1fr;
    gap: 1.5rem;
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
    margin: 0 0 0.25rem 0;
    color: var(--text-main);
  }

  .subtitle {
    font-size: 0.85rem;
    margin: 0;
    color: var(--text-muted);
  }

  .theme-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
    gap: 1rem;
  }

  .theme-card {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.5rem;
    border-radius: 12px;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
    cursor: pointer;
    position: relative;
    transition: all 0.2s ease;
  }

  .theme-card:hover {
    border-color: var(--secondary);
    background: var(--glass-bg-mid);
  }

  .theme-card.active {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.05);
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
    gap: 0.4rem;
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
    width: 20px;
    height: 20px;
    background: var(--secondary);
    color: var(--bg-base);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: bold;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
    border-radius: 12px;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .primary-btn {
    background: var(--secondary);
    color: var(--bg-base);
    border: none;
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    font-family: var(--font-body);
    transition: transform 0.2s ease, opacity 0.2s ease;
  }

  .primary-btn:hover {
    transform: scale(1.05);
    opacity: 0.9;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
