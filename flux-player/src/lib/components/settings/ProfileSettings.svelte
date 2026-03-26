<script lang="ts">
  import Icon from "../ui/Icon.svelte";
  import ProfileAvatar from "../ProfileAvatar.svelte";
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let computerName = $state("Fetching...");
  let displayName = $state("Flux User");

  onMount(async () => {
    try {
      // Safe context check for Playwright/Browser environment
      if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
        const name = await invoke<string>('get_computer_name');
        computerName = name;
        if (displayName === "Flux User") {
          displayName = name; // Auto-set display name to computer name initially
        }
      } else {
        computerName = "LOCAL-TEST-PC";
      }
    } catch (err) {
      console.error("Failed to fetch computer name", err);
      computerName = "UNKNOWN-PC";
    }
  });

  function saveProfile() {
    window.dispatchEvent(new CustomEvent('flux-toast', {
      detail: { label: `Profile saved: ${displayName}`, icon: 'settings' }
    }));
    // Future: Save to SQLite / Settings Config via Rust invoke here
  }
</script>

<div class="profile-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="profile" size={24} class="header-icon" />
      <div>
        <h2>My Profile</h2>
        <p>Manage your identity across the local network.</p>
      </div>
    </div>
  </header>

  <section class="profile-card">
    <div class="avatar-section">
      <div class="avatar-large">
        <!-- Reusing the squircle profile avatar from the topbar -->
        <ProfileAvatar />
        <button class="edit-avatar-btn">
          <Icon name="edit" size={14} />
        </button>
      </div>
      <div class="avatar-info">
        <h3>{displayName}</h3>
        <span class="os-name">OS Name: {computerName}</span>
      </div>
    </div>

    <div class="form-group">
      <label for="display-name">Display Name</label>
      <input
        type="text"
        id="display-name"
        bind:value={displayName}
        placeholder="Enter your display name"
      />
      <span class="hint">This name will be visible if you cast to other devices.</span>
    </div>

    <div class="actions">
      <button class="btn-save" onclick={saveProfile}>Save Changes</button>
    </div>
  </section>
</div>

<style>
  .profile-settings {
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

  .profile-card {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 600px;
  }

  .avatar-section {
    display: flex;
    align-items: center;
    gap: 2rem;
  }

  .avatar-large {
    position: relative;
    width: 80px;
    height: 80px;
    /* Scale up the existing squircle */
    transform: scale(2.5);
    transform-origin: top left;
    margin-right: 40px; /* Compensate for scale */
    margin-bottom: 40px;
  }

  .edit-avatar-btn {
    position: absolute;
    bottom: -4px;
    right: -4px;
    background: var(--bg-surface);
    border: 1px solid var(--glass-border-mid);
    color: var(--secondary);
    width: 24px;
    height: 24px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    box-shadow: 0 4px 10px rgba(0,0,0,0.5);
    /* Scale down visually because parent is scaled up 2.5x */
    transform: scale(0.4);
    transition: all 0.2s ease;
  }

  .edit-avatar-btn:hover {
    border-color: var(--secondary);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.3);
  }

  .avatar-info h3 {
    font-size: 1.5rem;
    margin: 0 0 0.5rem 0;
    color: var(--text-main);
  }

  .os-name {
    font-family: monospace;
    color: var(--text-muted);
    font-size: 0.85rem;
    background: rgba(0,0,0,0.3);
    padding: 0.3rem 0.6rem;
    border-radius: 6px;
    border: 1px solid var(--glass-border-low);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  label {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  input {
    background: var(--bg-base);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 0.8rem 1rem;
    border-radius: 8px;
    font-size: 1rem;
    transition: all 0.2s ease;
  }

  input:focus {
    outline: none;
    border-color: var(--secondary);
    box-shadow: 0 0 0 2px rgba(0, 255, 255, 0.1);
  }

  .hint {
    font-size: 0.8rem;
    color: var(--text-muted);
    font-style: italic;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
    border-top: 1px solid var(--glass-border-low);
    padding-top: 1.5rem;
  }

  .btn-save {
    background: rgba(0, 255, 255, 0.1);
    color: var(--secondary);
    border: 1px solid rgba(0, 255, 255, 0.3);
    padding: 0.8rem 1.5rem;
    border-radius: 8px;
    font-size: 0.95rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-save:hover {
    background: rgba(0, 255, 255, 0.2);
    box-shadow: 0 0 15px rgba(0, 255, 255, 0.1);
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
