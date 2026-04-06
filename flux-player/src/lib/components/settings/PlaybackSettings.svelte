<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import { settings, updateSetting, type AutoQueueMode, type TransitionBehavior } from '$lib/stores/settings';
  import Icon from "../ui/Icon.svelte";
  import Dropdown from '../ui/Dropdown.svelte';
  import { get } from 'svelte/store';

  // ── Local State (Runes) synced with Global Store ───────────────────────────
  const current = get(settings);

  let hwAcceleration = $state(current.hwAcceleration);
  let ffmpegThreading = $state(current.ffmpegThreading);
  let watchedThreshold = $state(current.watchedThreshold); // percent

  let subtitleLanguage = $state(current.subtitleLanguage);
  let subtitleFuzzy = $state(current.subtitleFuzzy);
  let subtitleAutoFetch = $state(current.subtitleAutoFetch);
  let osUsername = $state(current.osUsername);

  let autoQueue = $state<AutoQueueMode>(current.autoQueueMode);
  let allowMixedQueue = $state(current.allowMixedQueue);
  let transitionBehavior = $state<TransitionBehavior>(current.transitionBehavior);
  let videoAudioTransition = $state(current.videoAudioTransition);

  const threadingOptions = ['Auto', '1 Core', '2 Cores', '4 Cores', 'Max Cores'];
  const queueOptions = ['Never', 'Smart', 'Always'] as const;

  // ── Sync Effects ───────────────────────────────────────────────────────────
  $effect(() => { updateSetting('hwAcceleration', hwAcceleration); });
  $effect(() => { updateSetting('ffmpegThreading', ffmpegThreading); });
  $effect(() => { updateSetting('watchedThreshold', watchedThreshold); });
  $effect(() => { updateSetting('autoQueueMode', autoQueue); });
  $effect(() => { updateSetting('allowMixedQueue', allowMixedQueue); });
  $effect(() => { updateSetting('transitionBehavior', transitionBehavior); });
  $effect(() => { updateSetting('videoAudioTransition', videoAudioTransition); });
  $effect(() => { updateSetting('subtitleLanguage', subtitleLanguage); });
  $effect(() => { updateSetting('subtitleFuzzy', subtitleFuzzy); });
  $effect(() => { updateSetting('subtitleAutoFetch', subtitleAutoFetch); });
  $effect(() => { updateSetting('osUsername', osUsername); });
</script>

<div class="playback-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="playback" size={24} class="header-icon" />
      <div>
        <h2>Playback & Performance</h2>
        <p>Optimize the rendering engine and automate your discovery queue logic.</p>
      </div>
    </div>
  </header>

  <div class="settings-grid">
    <!-- Performance / Core Engine -->
    <section class="settings-card full-width">
      <div class="card-header">
        <div class="header-with-icon">
          <Icon name="performance" size={18} class="sub-icon" />
          <div>
            <h3>Core Engine</h3>
            <p class="subtitle">Hardware decoding and threading optimizations.</p>
          </div>
        </div>
      </div>

      <div class="setting-list">
        <div class="setting-row">
          <div class="setting-info">
            <label for="hwAcceleration">Hardware Acceleration (GPU)</label>
            <span class="description">Offload video decoding to your graphics card for smoother playback.</span>
          </div>
          <div class="switch-wrapper">
            <label class="switch">
              <input type="checkbox" id="hwAcceleration" bind:checked={hwAcceleration} />
              <span class="slider round"></span>
            </label>
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">FFmpeg Threading</span>
            <span class="description">Control CPU core allocation for the rendering pipeline.</span>
          </div>
          <div class="control-wrapper">
            <Dropdown options={threadingOptions} bind:value={ffmpegThreading} />
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <label for="watchedThreshold">"Watched" Threshold ({watchedThreshold}%)</label>
            <span class="description">Percentage of runtime required to mark a video as watched.</span>
          </div>
          <div class="slider-wrapper">
            <input type="range" id="watchedThreshold" min="70" max="100" step="1" bind:value={watchedThreshold} />
            <div class="slider-marks">
              <span>70%</span>
              <span>100%</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Queue Behavior -->
    <section class="settings-card">
      <div class="card-header">
        <div>
          <h3>Playlist & Queue</h3>
          <p class="subtitle">Automation rules for media sessions.</p>
        </div>
      </div>

      <div class="setting-list">
        <div class="setting-row vertical">
          <div class="setting-info">
            <span class="setting-label">Auto-Queue Mode</span>
            <span class="description">How Flux builds your next-up list.</span>
          </div>
          <div class="radio-grid">
            {#each queueOptions as opt}
              <label class="radio-card" class:active={autoQueue === opt}>
                <input type="radio" name="autoQueue" value={opt} bind:group={autoQueue} />
                <span class="radio-label">{opt}</span>
              </label>
            {/each}
          </div>
          <p class="help-text">
            {#if autoQueue === 'Never'}
              Single-item playback only. No auto-next.
            {:else if autoQueue === 'Smart'}
              Automatically queue the next episode in a series.
            {:else if autoQueue === 'Always'}
              Queue all items from your current filtered view.
            {/if}
          </p>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <label for="allowMixedQueue">Mixed Media</label>
            <span class="description">Allow audio and video in the same queue.</span>
          </div>
          <div class="switch-wrapper">
            <label class="switch">
              <input type="checkbox" id="allowMixedQueue" bind:checked={allowMixedQueue} />
              <span class="slider round"></span>
            </label>
          </div>
        </div>
      </div>
    </section>

    <!-- Seamless Flow -->
    <section class="settings-card">
      <div class="card-header">
        <div>
          <h3>Seamless Experience</h3>
          <p class="subtitle">Control how Flux handles media transitions in your queue.</p>
        </div>
      </div>

      <div class="setting-list">
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">Video-to-Audio Transition</span>
            <span class="description">Route to the library automatically when a video ends and the next item in the queue is audio-only.</span>
          </div>
          <div class="switch-wrapper">
            <label class="switch">
              <input type="checkbox" bind:checked={videoAudioTransition} />
              <span class="slider round"></span>
            </label>
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">Post-Playback Behavior</span>
            <span class="description">Where to navigate after the queue finishes.</span>
          </div>
          <div class="control-wrapper">
            <Dropdown options={['Return to Library', 'Stay in Now Playing']} bind:value={transitionBehavior} />
          </div>
        </div>
      </div>
    </section>

    <!-- Subtitles -->
    <section class="settings-card full-width">
      <div class="card-header">
        <div>
          <h3>Subtitles</h3>
          <p class="subtitle">Local matching and OpenSubtitles sync.</p>
        </div>
      </div>

      <div class="setting-list">
        <div class="setting-row">
          <div class="setting-info">
            <span class="setting-label">Preferred Language</span>
          </div>
          <div class="control-wrapper">
            <Dropdown options={['System Default', 'English', 'Spanish', 'French', 'Japanese']} bind:value={subtitleLanguage} />
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-info">
            <label for="subtitleFuzzy">Fuzzy Matching</label>
            <span class="description">Match local .srt files with similar names.</span>
          </div>
          <div class="switch-wrapper">
            <label class="switch">
              <input type="checkbox" id="subtitleFuzzy" bind:checked={subtitleFuzzy} />
              <span class="slider round"></span>
            </label>
          </div>
        </div>

        <div class="setting-row account-row">
          <div class="setting-info">
            <label for="osUsername">OpenSubtitles</label>
          </div>
          <div class="input-group">
            <input type="text" id="osUsername" bind:value={osUsername} placeholder="Username" />
            <button class="btn-secondary">Login</button>
          </div>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
  .playback-settings {
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

  .header-with-icon {
    display: flex;
    gap: 1rem;
    align-items: flex-start;
  }

  :global(.sub-icon) {
    color: var(--secondary);
    margin-top: 2px;
  }

  .setting-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--glass-border-low);
    padding: 1rem;
    border-radius: 12px;
  }

  .setting-row.vertical {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .setting-info label, .setting-info .setting-label {
    font-weight: 600;
    color: var(--text-main);
    font-size: 0.95rem;
  }

  .description {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  /* Control Wrappers */
  .switch-wrapper, .control-wrapper {
    flex-shrink: 0;
  }

  .slider-wrapper {
    flex: 0 0 200px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  /* Radio Grid */
  .radio-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
    width: 100%;
  }

  .radio-card {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.6rem;
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .radio-card:hover {
    background: var(--glass-bg-mid);
    border-color: var(--secondary);
  }

  .radio-card.active {
    background: rgba(0, 255, 255, 0.1);
    border-color: var(--secondary);
    color: var(--secondary);
  }

  .radio-card input {
    display: none;
  }

  .help-text {
    font-size: 0.8rem;
    color: var(--secondary);
    opacity: 0.8;
    margin: 0;
    font-weight: 500;
  }

  /* Input Group */
  .input-group {
    display: flex;
    gap: 0.5rem;
  }

  input[type="text"] {
    background: var(--bg-base);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.9rem;
    width: 150px;
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

  /* Switch */
  .switch {
    position: relative;
    display: inline-block;
    width: 44px;
    height: 22px;
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
    background-color: var(--glass-bg-high);
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 22px;
    border: 1px solid var(--glass-border-mid);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 16px;
    width: 16px;
    left: 2px;
    top: 2px;
    background-color: var(--text-muted);
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: rgba(0, 255, 255, 0.15);
    border-color: var(--secondary);
  }

  input:checked + .slider:before {
    transform: translateX(22px);
    background-color: var(--secondary);
    box-shadow: 0 0 8px rgba(0, 255, 255, 0.5);
  }

  /* Range */
  input[type="range"] {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 4px;
    background: var(--glass-bg-high);
    border-radius: 2px;
  }

  input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    background: var(--secondary);
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.4);
  }

  .slider-marks {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: 600;
    margin-top: 0.25rem;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
