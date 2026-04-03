<script lang="ts">
  import { onMount, untrack } from 'svelte';
  import { settings, updateSetting, type AutoQueueMode, type TransitionBehavior } from '$lib/stores/settings';
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

  const threadingOptions = ['Auto', '1 Core', '2 Cores', '4 Cores', 'Max Cores'];
  const queueOptions = ['Never', 'Smart', 'Always'] as const;
  const transitionOptions = ['Return to Library', 'Stay in Now Playing'] as const;

  // ── Sync Effects ───────────────────────────────────────────────────────────
  $effect(() => { updateSetting('hwAcceleration', hwAcceleration); });
  $effect(() => { updateSetting('ffmpegThreading', ffmpegThreading); });
  $effect(() => { updateSetting('watchedThreshold', watchedThreshold); });
  $effect(() => { updateSetting('autoQueueMode', autoQueue); });
  $effect(() => { updateSetting('allowMixedQueue', allowMixedQueue); });
  $effect(() => { updateSetting('transitionBehavior', transitionBehavior); });
  $effect(() => { updateSetting('subtitleLanguage', subtitleLanguage); });
  $effect(() => { updateSetting('subtitleFuzzy', subtitleFuzzy); });
  $effect(() => { updateSetting('subtitleAutoFetch', subtitleAutoFetch); });
  $effect(() => { updateSetting('osUsername', osUsername); });
</script>

<div class="settings-section">
  <h2>Playback & Performance</h2>

  <div class="card">
    <h3>Core Engine</h3>

    <div class="setting-row">
      <div class="setting-info">
        <label for="hwAcceleration">Hardware Acceleration (GPU)</label>
        <span class="description">Offload video decoding to your graphics card for smoother playback.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="hwAcceleration" bind:checked={hwAcceleration} />
        <span class="slider round"></span>
      </label>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <span class="description">Control CPU core allocation for the rendering pipeline.</span>
      </div>
      <Dropdown options={threadingOptions} bind:value={ffmpegThreading} label="FFmpeg Threading" />
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="watchedThreshold">"Watched" Threshold ({watchedThreshold}%)</label>
        <span class="description">Percentage of runtime required to mark a video as watched.</span>
      </div>
      <div class="slider-container">
        <input type="range" id="watchedThreshold" min="70" max="100" step="1" bind:value={watchedThreshold} />
        <div class="slider-marks">
          <span>70%</span>
          <span>100%</span>
        </div>
      </div>
    </div>
  </div>

  <div class="card">
    <h3>Playlist & Queue Behavior</h3>

    <div class="setting-row radio-group-row">
      <div class="setting-info">
        <span class="setting-label">Auto-Queue Library Items</span>
        <span class="description">Control whether playing media automatically creates a queue.</span>
      </div>
      <div class="radio-group" role="radiogroup" aria-label="Auto-Queue Library Items">
        {#each queueOptions as opt}
          <label class="radio-label">
            <input type="radio" name="autoQueue" value={opt} bind:group={autoQueue} />
            <span class="radio-custom"></span>
            {opt}
          </label>
        {/each}
        {#if autoQueue === 'Never'}
          <span class="help-text">Single-item playback only.</span>
        {:else if autoQueue === 'Smart'}
          <span class="help-text">Queue episodes in series folders.</span>
        {:else if autoQueue === 'Always'}
          <span class="help-text">Queue from current filtered view.</span>
        {/if}
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="allowMixedQueue">Mixed Media Queue</label>
        <span class="description">Allow audio and video tracks to be mixed in the same session queue.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="allowMixedQueue" bind:checked={allowMixedQueue} />
        <span class="slider round"></span>
      </label>
    </div>

    <div class="setting-row radio-group-row">
      <div class="setting-info">
        <span class="setting-label">Video-to-Audio Transition</span>
        <span class="description">Behavior when transitioning from video to audio in mixed playlists.</span>
      </div>
      <div class="radio-group" role="radiogroup" aria-label="Video-to-Audio Transition">
        {#each transitionOptions as opt}
          <label class="radio-label">
            <input type="radio" name="transitionBehavior" value={opt} bind:group={transitionBehavior} />
            <span class="radio-custom"></span>
            {opt}
          </label>
        {/each}
        {#if transitionBehavior === 'Stay in Now Playing'}
          <span class="help-text">Show audio visualizer in full Now Playing view.</span>
        {/if}
      </div>
    </div>
  </div>

  <div class="card">
    <h3>Subtitles</h3>

    <div class="setting-row">
      <div class="setting-info">
        <span class="description">Default language for selected subtitles.</span>
      </div>
      <Dropdown options={['System Default', 'English', 'Spanish', 'French', 'Japanese']} bind:value={subtitleLanguage} label="Preferred Language" />
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="subtitleFuzzy">Fuzzy Matching (Local)</label>
        <span class="description">Match subtitle files with similar names in the same directory.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="subtitleFuzzy" bind:checked={subtitleFuzzy} />
        <span class="slider round"></span>
      </label>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <label for="subtitleAutoFetch">Auto-Fetch Subtitles (Online)</label>
        <span class="description">Download missing subtitles automatically.</span>
      </div>
      <label class="switch">
        <input type="checkbox" id="subtitleAutoFetch" bind:checked={subtitleAutoFetch} />
        <span class="slider round"></span>
      </label>
    </div>

    <div class="setting-row account-row">
      <div class="setting-info">
        <label for="osUsername">OpenSubtitles Account</label>
        <span class="description">Optional login for higher daily download limits.</span>
      </div>
      <div class="input-group">
        <input type="text" id="osUsername" bind:value={osUsername} placeholder="Username" />
        <button class="btn-outline">Login</button>
      </div>
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

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--glass-border-low);
    padding-bottom: 1.5rem;
  }

  .setting-row:last-child {
    border-bottom: none;
    padding-bottom: 0;
  }

  .radio-group-row {
    align-items: flex-start;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
    padding-right: 2rem;
  }

  label, .setting-label {
    font-weight: 500;
    color: var(--text-main);
  }

  .description {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 0;
  }

  /* Input elements */
  .input-group {
    display: flex;
    gap: 0.5rem;
  }

  input[type="text"] {
    background: var(--glass-bg-low);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.65rem 1rem;
    border-radius: 8px;
    font-family: var(--font-body);
    font-size: 0.95rem;
    outline: none;
    transition: all 0.2s ease;
  }

  input[type="text"]:focus {
    border-color: var(--secondary);
    background: var(--glass-bg-mid);
  }

  .btn-outline {
    background: transparent;
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 0.65rem 1.25rem;
    border-radius: 8px;
    cursor: pointer;
    font-family: var(--font-body);
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .btn-outline:hover {
    border-color: var(--secondary);
    color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
  }

  /* Radio Group Styles */
  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    min-width: 250px;
  }

  .radio-label {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    cursor: pointer;
    font-size: 0.95rem;
    color: var(--text-muted);
    transition: color 0.2s ease;
  }

  .radio-label:hover {
    color: var(--text-main);
  }

  .radio-label input[type="radio"] {
    display: none;
  }

  .radio-custom {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2px solid var(--glass-border-high);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .radio-label input[type="radio"]:checked + .radio-custom {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.1);
  }

  .radio-label input[type="radio"]:checked + .radio-custom::after {
    content: '';
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--secondary);
    box-shadow: 0 0 8px var(--secondary);
  }

  .help-text {
    font-size: 0.8rem;
    color: var(--secondary);
    opacity: 0.8;
    margin-left: 2rem;
    margin-top: -0.25rem;
    font-weight: 500;
  }

  /* Slider Container */
  .slider-container {
    flex: 0 0 200px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  input[type="range"] {
    appearance: none;
    width: 100%;
    background: transparent;
  }

  input[type="range"]::-webkit-slider-thumb {
    appearance: none;
    height: 18px;
    width: 18px;
    border-radius: 50%;
    background: var(--secondary);
    cursor: pointer;
    margin-top: -7px;
    border: 2px solid var(--bg-base);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.3);
  }

  input[type="range"]::-webkit-slider-runnable-track {
    width: 100%;
    height: 4px;
    cursor: pointer;
    background: var(--glass-border-mid);
    border-radius: 4px;
  }

  .slider-marks {
    display: flex;
    justify-content: space-between;
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: 600;
  }

  /* Switch */
  .switch {
    position: relative;
    display: inline-block;
    width: 48px;
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
    background-color: var(--glass-border-high);
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 24px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: var(--secondary);
  }

  input:checked + .slider:before {
    transform: translateX(24px);
  }
</style>
