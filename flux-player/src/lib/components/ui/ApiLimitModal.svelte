<script lang="ts">
  import { fade, scale } from 'svelte/transition';
  import { backOut } from 'svelte/easing';
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './Icon.svelte';

  let { onclose } = $props();
  let tmdbKey = $state('');
  let isSaving = $state(false);

  async function saveToken() {
    if (!tmdbKey.trim()) return;
    isSaving = true;
    try {
      await invoke('save_setting', { key: 'tmdb_read_token', value: tmdbKey.trim() });
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Infinite Discovery Unlocked!', icon: 'sparkle' }
      }));
      onclose();
    } catch (err) {
      console.error("Failed to save TMDB token:", err);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Failed to verify token', icon: 'error' }
      }));
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') onclose();
    if (e.key === 'Enter' && tmdbKey) saveToken();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="modal-overlay glass-dark" transition:fade={{ duration: 200 }}>
  <div class="modal-card glass" transition:scale={{ duration: 400, easing: backOut, start: 0.9 }}>
    <header class="modal-header">
      <div class="header-content">
        <Icon name="streaming" size={32} class="header-icon" />
        <div class="title-stack">
          <h2>Unlock Infinite Discovery</h2>
          <p>You've hit the 150-call community limit for Flux's shared API keys. To keep Flux free and high-performance for everyone, users who scan large libraries need to link their own personal TMDB account.</p>
        </div>
      </div>
      <button class="close-btn" onclick={onclose}>✕</button>
    </header>

    <div class="modal-body">
      <div class="steps-container">
        <div class="step-item">
          <div class="step-number">1</div>
          <div class="step-text">
            <strong>Create a Free Account</strong>
            <span>Sign up at <a href="https://www.themoviedb.org/signup" target="_blank" rel="noopener">themoviedb.org/signup</a>. It's 100% free for personal use.</span>
          </div>
        </div>
        <div class="step-item">
          <div class="step-number">2</div>
          <div class="step-text">
            <strong>Register your "App"</strong>
            <span>Go to Settings > API and create a new "Developer" API key. Use the cheat sheet below to get approved instantly.</span>
          </div>
        </div>
      </div>

      <div class="cheat-sheet">
        <div class="sheet-info">
          <h4>Registration Cheat Sheet</h4>
          <p>Copy these values into your <a href="https://www.themoviedb.org/settings/api" target="_blank" rel="noopener">TMDB App registration form:</a></p>
        </div>

        <div class="copy-grid">
          <div class="copy-field">
            <span class="label">App Name</span>
            <code class="value">Flux Player Local</code>
          </div>
          <div class="copy-field">
            <span class="label">App URL</span>
            <code class="value">https://github.com/flux-player</code>
          </div>
          <div class="copy-field long">
            <span class="label">Application Summary</span>
            <code class="value">A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection.</code>
          </div>
        </div>
      </div>

      <div class="form-group">
        <label for="token-input">Paste Your API Read Access Token (v4) Here</label>
        <div class="input-row">
          <input
            id="token-input"
            type="password"
            bind:value={tmdbKey}
            placeholder="eyJhbGciOiJIUzI1NiJ9..."
          />
          <button class="btn-primary" onclick={saveToken} disabled={!tmdbKey || isSaving}>
            {isSaving ? 'Verifying...' : 'Unlock Now'}
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    z-index: 100000;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(10, 10, 12, 0.85);
    backdrop-filter: blur(12px);
  }

  .modal-card {
    width: 650px;
    max-width: 90vw;
    background: rgba(14, 14, 16, 0.95);
    border: 1px solid var(--glass-border-mid);
    border-radius: 32px;
    padding: 40px;
    box-shadow: 0 40px 100px rgba(0, 0, 0, 0.8);
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .header-content {
    display: flex;
    gap: 24px;
    align-items: center;
  }

  :global(.header-icon) {
    color: var(--secondary);
    filter: drop-shadow(0 0 10px rgba(138, 43, 226, 0.4));
  }

  .title-stack h2 {
    font-family: var(--font-heading);
    font-size: 1.5rem;
    margin: 0;
    background: linear-gradient(135deg, #fff, var(--secondary));
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .title-stack p {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 8px 0 0;
    line-height: 1.5;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 20px;
    cursor: pointer;
    padding: 8px;
    transition: color 0.2s;
  }

  .close-btn:hover {
    color: var(--text-main);
  }

  .modal-body {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .steps-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
    background: rgba(255, 255, 255, 0.02);
    border-radius: 16px;
    padding: 20px;
  }

  .step-item {
    display: flex;
    gap: 16px;
    align-items: flex-start;
  }

  .step-number {
    width: 24px;
    height: 24px;
    background: var(--secondary);
    color: #000;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: 800;
    flex-shrink: 0;
  }

  .step-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .step-text strong {
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .step-text span {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .cheat-sheet {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--glass-border-low);
    border-radius: 16px;
    padding: 20px;
  }

  .sheet-info h4 {
    color: var(--secondary);
    margin: 0;
    font-size: 0.95rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .sheet-info p {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 4px 0 16px;
  }

  .step-text a, .sheet-info a {
    color: var(--primary);
    text-decoration: underline;
  }

  .sheet-info a {
    text-decoration: underline;
  }

  .copy-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .copy-field.long {
    grid-column: span 2;
  }

  .label {
    display: block;
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: 700;
    margin-bottom: 6px;
    text-transform: uppercase;
  }

  .value {
    display: block;
    background: rgba(0, 0, 0, 0.4);
    padding: 10px 12px;
    border-radius: 8px;
    font-family: monospace;
    font-size: 0.8rem;
    color: var(--text-main);
    border: 1px solid var(--glass-border-low);
    user-select: all;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  label {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-weight: 600;
  }

  .input-row {
    display: flex;
    gap: 12px;
  }

  input {
    flex: 1;
    background: var(--bg-base);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 12px 16px;
    border-radius: 12px;
    font-family: monospace;
    outline: none;
    transition: all 0.2s;
  }

  input:focus {
    border-color: var(--secondary);
    background: rgba(138, 43, 226, 0.05);
  }

  .btn-primary {
    background: var(--secondary);
    color: #000;
    border: none;
    padding: 0 24px;
    border-radius: 12px;
    font-weight: 700;
    font-family: var(--font-body);
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    filter: brightness(1.1);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
