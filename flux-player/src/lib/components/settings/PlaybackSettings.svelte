<script lang="ts">
  let hwAcceleration = $state(true);
  let ffmpegThreading = $state('Auto');
  let watchedThreshold = $state(90); // percent

  let subtitleLanguage = $state('System Default');
  let subtitleFuzzy = $state(true);
  let subtitleAutoFetch = $state(true);
  let osUsername = $state('');

  let autoQueue = $state('Smart');
  let transitionBehavior = $state('Return to Library');

  const threadingOptions = ['Auto', '1 Core', '2 Cores', '4 Cores', 'Max Cores'];
  const queueOptions = ['Never', 'Smart', 'Always'];
  const transitionOptions = ['Return to Library', 'Stay in Now Playing'];
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
        <label for="ffmpegThreading">FFmpeg Threading</label>
        <span class="description">Control CPU core allocation for the rendering pipeline.</span>
      </div>
      <select id="ffmpegThreading" bind:value={ffmpegThreading}>
        {#each threadingOptions as opt}
          <option value={opt}>{opt}</option>
        {/each}
      </select>
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
        <label for="subtitleLanguage">Preferred Language</label>
        <span class="description">Default language for selected subtitles.</span>
      </div>
      <select id="subtitleLanguage" bind:value={subtitleLanguage}>
        <option value="System Default">System Default</option>
        <option value="English">English</option>
        <option value="Spanish">Spanish</option>
        <option value="French">French</option>
        <option value="Japanese">Japanese</option>
      </select>
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

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
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

  /* Form Elements */
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

  .input-group {
    display: flex;
    gap: 0.5rem;
  }

  input[type="text"] {
    background: var(--bg-base);
    color: var(--text-main);
    border: 1px solid var(--border-light);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-family: var(--font-body);
    font-size: 0.95rem;
    outline: none;
    transition: border-color 0.2s ease;
  }

  input[type="text"]:focus {
    border-color: var(--primary);
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
  }

  .radio-label input[type="radio"] {
    display: none;
  }

  .radio-custom {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid var(--border-light);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .radio-label input[type="radio"]:checked + .radio-custom {
    border-color: var(--primary);
  }

  .radio-label input[type="radio"]:checked + .radio-custom::after {
    content: '';
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--primary);
  }

  .help-text {
    font-size: 0.8rem;
    color: var(--secondary);
    margin-left: 1.75rem;
    margin-top: -0.25rem;
  }

  /* Slider Container */
  .slider-container {
    flex: 0 0 200px;
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
    margin-top: -6px;
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

  /* Switch */
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
