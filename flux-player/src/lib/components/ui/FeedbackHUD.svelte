<script lang="ts">
  /**
   * FeedbackHUD.svelte
   * A premium, command-palette-style overlay for rapid bug reporting and feature requests.
   * Triggered globally via Ctrl+Shift+F. Matches the Dynamic Island aesthetic.
   * 
   * Features:
   * - Type-ahead feedback submission
   * - Category toggle (Bug / Feature)
   * - Manual screenshot capture with preview
   * - Auto-attached diagnostic data
   */
  import { fade, fly } from 'svelte/transition';
  import { tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import Icon from './Icon.svelte';

  interface DiagnosticReport {
    app_version: string;
    os: string;
    computer_name: string;
    library_total: number;
    library_enriched: number;
    library_pending: number;
    offline_mode: boolean;
    auto_indexing: boolean;
    has_custom_tmdb_key: boolean;
    theme: string;
  }

  let { show = $bindable(false) } = $props();

  let feedbackType = $state<'bug' | 'feature'>('bug');
  let feedbackText = $state('');
  let isCapturing = $state(false);
  let isSending = $state(false);
  let screenshotPath = $state<string | null>(null);
  let screenshotPreview = $state<string | null>(null);
  let diagnostics = $state<DiagnosticReport | null>(null);
  let inputRef = $state<HTMLTextAreaElement | null>(null);

  /**
   * Fetches diagnostic report from the Rust backend when HUD opens.
   */
  $effect(() => {
    if (show) {
      invoke<DiagnosticReport>('get_diagnostic_report').then(report => {
        diagnostics = report;
      }).catch(err => console.error('[FeedbackHUD] Failed to fetch diagnostics:', err));

      // Auto-focus the input after the opening animation
      setTimeout(() => inputRef?.focus(), 200);
    } else {
      // Reset state when closing
      feedbackText = '';
      screenshotPath = null;
      screenshotPreview = null;
      feedbackType = 'bug';
    }
  });

  /**
   * Captures a screenshot of the window via the cross-platform Rust backend.
   * Hides the HUD temporarily to ensure a clean, unblurred capture.
   */
  async function captureScreenshot() {
    isCapturing = true;

    // 1. Hide the HUD panel immediately
    const hudPanel = document.querySelector('.hud-panel') as HTMLElement;
    const backdrop = document.querySelector('.hud-backdrop') as HTMLElement;
    if (hudPanel) hudPanel.style.opacity = '0';
    if (backdrop) backdrop.style.backdropFilter = 'none'; // Remove blur

    // 2. Flash effect
    const flash = document.createElement('div');
    flash.style.cssText = 'position:fixed;inset:0;background:white;opacity:0;z-index:99999;pointer-events:none;transition:opacity 0.1s ease;';
    document.body.appendChild(flash);
    
    // 3. Small delay to let the OS/GPU refresh without the HUD
    await new Promise(r => setTimeout(r, 150));

    requestAnimationFrame(() => { flash.style.opacity = '0.3'; });
    setTimeout(() => { flash.style.opacity = '0'; setTimeout(() => flash.remove(), 200); }, 100);

    try {
      const result = await invoke<{ path: string; base64: string }>('capture_screenshot');
      screenshotPath = result.path;
      screenshotPreview = result.base64;

      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Clean Screenshot Captured', icon: 'camera' }
      }));
    } catch (err) {
      console.error('[FeedbackHUD] Screenshot failed:', err);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Screenshot Failed', icon: 'close' }
      }));
    } finally {
      // 4. Restore the HUD
      if (hudPanel) hudPanel.style.opacity = '1';
      if (backdrop) backdrop.style.backdropFilter = 'blur(12px)';
      isCapturing = false;
    }
  }

  /**
   * Discards the currently captured screenshot.
   */
  function discardScreenshot() {
    screenshotPath = null;
    screenshotPreview = null;
    window.dispatchEvent(new CustomEvent('flux-toast', {
      detail: { label: 'Screenshot Removed', icon: 'close' }
    }));
  }

  /**
   * Submits the feedback report both to Formspree (Direct) 
   * and opens a GitHub issue (Backup).
   */
  async function submitFeedback() {
    if (!feedbackText.trim() || isSending) return;

    isSending = true;

    // 1. Send to Formspree (Direct)
    try {
      await invoke('send_feedback_report', {
        feedback: feedbackText,
        report: diagnostics,
        screenshotPath: screenshotPath
      });
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Report Sent Directly', icon: 'check' }
      }));
    } catch (err) {
      console.error('[FeedbackHUD] Direct Send failed:', err);
      window.dispatchEvent(new CustomEvent('flux-toast', {
        detail: { label: 'Direct Send Failed', icon: 'close' }
      }));
    }

    // 2. Open GitHub Issue (Backup)
    const isBug = feedbackType === 'bug';
    const titlePrefix = isBug ? '[BUG]' : '[FEATURE]';
    const title = encodeURIComponent(`${titlePrefix} ${feedbackText.slice(0, 80)}`);

    let body = '';
    if (isBug) {
      body += `## Bug Description\n${feedbackText}\n\n`;
      body += `## Steps to Reproduce\n1. \n2. \n3. \n\n`;
    } else {
      body += `## Feature Request\n${feedbackText}\n\n`;
      body += `## Use Case\n_Describe when and why this feature would be useful._\n\n`;
    }

    if (screenshotPath) {
      body += `## Screenshot\n_Screenshot saved locally at:_\n\`${screenshotPath}\`\n\n`;
      body += `> **Note**: Please attach the screenshot file from the path above to this issue.\n\n`;
    }

    if (diagnostics) {
      body += `## Diagnostic Context\n`;
      body += `| Key | Value |\n|---|---|\n`;
      body += `| App Version | ${diagnostics.app_version} |\n`;
      body += `| OS | ${diagnostics.os} |\n`;
      body += `| Device | ${diagnostics.computer_name} |\n`;
      body += `| Library | ${diagnostics.library_total} items (${diagnostics.library_enriched} enriched, ${diagnostics.library_pending} pending) |\n`;
      body += `| Offline Mode | ${diagnostics.offline_mode ? 'Yes' : 'No'} |\n`;
      body += `| Auto Indexing | ${diagnostics.auto_indexing ? 'Yes' : 'No'} |\n`;
      body += `| Custom TMDB Key | ${diagnostics.has_custom_tmdb_key ? 'Yes' : 'No'} |\n`;
      body += `| Theme | ${diagnostics.theme} |\n`;
    }

    const encodedBody = encodeURIComponent(body);
    const labels = isBug ? '&labels=bug' : '&labels=enhancement';
    openUrl(`https://github.com/xtarflex/Flux-player/issues/new?title=${title}&body=${encodedBody}${labels}`);

    isSending = false;
    show = false;
  }

  /**
   * Handles keyboard shortcuts within the HUD.
   * @param e - Keyboard event
   */
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      show = false;
    }
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      submitFeedback();
    }
  }
</script>

{#if show}
  <!-- Backdrop -->
  <div 
    class="hud-backdrop" 
    transition:fade={{ duration: 200 }}
    onclick={() => show = false}
    onkeydown={handleKeydown}
    role="button"
    tabindex="-1"
    aria-label="Close feedback"
  >

    <!-- HUD Panel -->
    <div 
      class="hud-panel"
      transition:fly={{ y: -40, duration: 350 }}
      onclick={(e) => e.stopPropagation()}
      onkeydown={handleKeydown}
      role="dialog"
      tabindex="-1"
      aria-label="Feedback HUD"
    >
      <!-- Header -->
      <div class="hud-header">
        <div class="hud-brand">
          <Icon name="feedback" size={18} />
          <span>Flux Feedback</span>
        </div>
        <div class="hud-shortcut">
          <button class="close-btn" onclick={() => show = false} aria-label="Close HUD">
            <Icon name="close" size={16} />
          </button>
        </div>
      </div>

      <!-- Type Toggle -->
      <div class="type-toggle">
        <button 
          class="toggle-btn" 
          class:active={feedbackType === 'bug'}
          onclick={() => feedbackType = 'bug'}
        >
          <Icon name="bug" size={14} />
          Bug Report
        </button>
        <button 
          class="toggle-btn" 
          class:active={feedbackType === 'feature'}
          onclick={() => feedbackType = 'feature'}
        >
          <Icon name="sparkle" size={14} />
          Feature Request
        </button>
      </div>

      <!-- Input -->
      <div class="input-area">
        <textarea 
          bind:this={inputRef}
          bind:value={feedbackText}
          placeholder={feedbackType === 'bug' 
            ? 'Describe the issue you encountered...' 
            : 'Describe the feature you\'d like to see...'}
          rows={3}
          onkeydown={handleKeydown}
        ></textarea>
      </div>

      <!-- Screenshot Preview Area -->
      {#if screenshotPreview}
        <div class="screenshot-area" transition:fade={{ duration: 200 }}>
          <img src={screenshotPreview} alt="Screenshot preview" />
          <div class="screenshot-overlay">
            <button class="discard-btn" onclick={discardScreenshot}>
              <Icon name="close" size={12} />
              Discard Screenshot
            </button>
          </div>
        </div>
      {/if}

      <!-- Diagnostic Badges -->
      {#if diagnostics}
        <div class="diag-badges" transition:fade={{ duration: 300 }}>
          <span class="diag-badge">{diagnostics.os}</span>
          <span class="diag-badge">{diagnostics.app_version}</span>
          <span class="diag-badge">{diagnostics.library_total} items</span>
          <span class="diag-badge">{diagnostics.theme}</span>
        </div>
      {/if}

      <!-- Actions -->
      <div class="hud-actions">
        <button 
          class="action-btn capture-btn"
          onclick={captureScreenshot}
          disabled={isCapturing || isSending}
        >
          <Icon name="camera" size={16} />
          {isCapturing ? 'Capturing...' : screenshotPath ? 'Retake' : 'Capture App UI'}
        </button>
        <button 
          class="action-btn submit-btn"
          onclick={submitFeedback}
          disabled={!feedbackText.trim() || isSending || !diagnostics}
        >
          {#if isSending}
            <div class="spinner"></div>
            Sending Report...
          {:else}
            <Icon name="send" size={16} />
            Submit & Open GitHub
            <kbd>↵</kbd>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<svelte:window onkeydown={(e) => {
  if (show && e.key === 'Escape') {
    show = false;
  }
}} />

<style>
  .hud-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    z-index: 9500;
    display: flex;
    justify-content: center;
    padding-top: 80px;
  }


  .hud-panel {
    width: 580px;
    max-height: 80vh;
    background: rgba(14, 14, 18, 0.95);
    border: 1px solid rgba(0, 255, 255, 0.15);
    border-radius: 24px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 
      0 0 60px rgba(0, 255, 255, 0.08),
      0 25px 50px rgba(0, 0, 0, 0.5);
    align-self: flex-start;
    transition: opacity 0.1s ease;
  }

  /* Header */
  .hud-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .hud-brand {
    display: flex;
    align-items: center;
    gap: 10px;
    color: var(--secondary);
    font-family: var(--font-heading);
    font-size: 0.9rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .hud-shortcut {
    display: flex;
    align-items: center;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    color: var(--text-main);
  }

  /* Type Toggle */
  .type-toggle {
    display: flex;
    gap: 8px;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 12px;
    padding: 4px;
  }

  .toggle-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 10px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 10px;
    color: var(--text-muted);
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.25s cubic-bezier(0.23, 1, 0.32, 1);
  }

  .toggle-btn:hover {
    color: var(--text-main);
    background: rgba(255, 255, 255, 0.04);
  }

  .toggle-btn.active {
    background: rgba(0, 255, 255, 0.08);
    border-color: rgba(0, 255, 255, 0.25);
    color: var(--secondary);
  }

  /* Input */
  .input-area textarea {
    width: 100%;
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--glass-border-mid);
    border-radius: 14px;
    padding: 16px;
    color: var(--text-main);
    font-family: var(--font-body);
    font-size: 0.95rem;
    line-height: 1.5;
    resize: none;
    outline: none;
    transition: border-color 0.2s ease;
  }

  .input-area textarea::placeholder {
    color: var(--text-muted);
    opacity: 0.6;
  }

  .input-area textarea:focus {
    border-color: var(--secondary);
    box-shadow: 0 0 0 3px rgba(0, 255, 255, 0.08);
  }

  /* Screenshot Preview Area */
  .screenshot-area {
    position: relative;
    width: 100%;
    max-height: 240px;
    border-radius: 16px;
    overflow: hidden;
    border: 1px solid var(--glass-border-mid);
    background: #000;
  }

  .screenshot-area img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    display: block;
  }

  .screenshot-overlay {
    position: absolute;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    opacity: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: opacity 0.2s ease;
  }

  .screenshot-area:hover .screenshot-overlay {
    opacity: 1;
  }

  .discard-btn {
    background: #ff4040;
    color: white;
    border: none;
    border-radius: 10px;
    padding: 8px 16px;
    font-size: 0.8rem;
    font-weight: 700;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    box-shadow: 0 4px 12px rgba(255, 64, 64, 0.3);
    transform: scale(0.9);
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .discard-btn:hover {
    transform: scale(1);
    background: #ff5555;
  }

  /* Diagnostic Badges */
  .diag-badges {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .diag-badge {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid var(--glass-border-low);
    border-radius: 6px;
    padding: 3px 10px;
    font-size: 0.7rem;
    color: var(--text-muted);
    font-family: monospace;
  }

  /* Actions */
  .hud-actions {
    display: flex;
    gap: 10px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 18px;
    border-radius: 12px;
    border: none;
    cursor: pointer;
    font-weight: 700;
    font-size: 0.85rem;
    transition: all 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .capture-btn {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    flex-shrink: 0;
  }

  .capture-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.1);
    transform: translateY(-1px);
  }

  .capture-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .submit-btn {
    flex: 1;
    background: var(--secondary);
    color: #000;
    justify-content: center;
  }

  .submit-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(0, 255, 255, 0.3);
  }

  .submit-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .submit-btn kbd {
    background: rgba(0, 0, 0, 0.15);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 0.65rem;
    font-family: monospace;
    margin-left: 8px;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(0, 0, 0, 0.2);
    border-top-color: #000;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-right: 8px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
