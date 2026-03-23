<script lang="ts">
  let displayName = $state('Sunny'); // Mock OS username
  let totalWatchTime = '142h 35m';
  let topGenres = ['Sci-Fi', 'Cyberpunk', 'Action'];
</script>

<div class="settings-section">
  <h2>My Profile</h2>

  <div class="profile-card">
    <div class="avatar-container">
      <div class="avatar squircle">
        <img src="/flux.png" alt="Profile Avatar" />
      </div>
      <button class="btn-outline">Change Avatar</button>
    </div>

    <div class="info-container">
      <div class="form-group">
        <label for="displayName">Display Name</label>
        <input type="text" id="displayName" bind:value={displayName} placeholder="Enter your name..." />
        <span class="help-text">Used for personalized greetings on the Home Screen.</span>
      </div>
    </div>
  </div>

  <div class="stats-section">
    <h3>Local Statistics</h3>
    <div class="stats-grid">
      <div class="stat-card glass">
        <span class="stat-label">Total Watch Time</span>
        <span class="stat-value">{totalWatchTime}</span>
      </div>
      <div class="stat-card glass">
        <span class="stat-label">Top Genres</span>
        <span class="stat-value genres">
          {#each topGenres as genre}
            <span class="genre-tag">{genre}</span>
          {/each}
        </span>
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
    margin-bottom: 1rem;
    border-bottom: 1px solid var(--border-light);
    padding-bottom: 1rem;
  }

  h3 {
    color: var(--text-main);
    font-size: 1.1rem;
    margin-bottom: 1rem;
  }

  .profile-card {
    display: flex;
    gap: 2rem;
    align-items: flex-start;
    background: var(--bg-surface);
    padding: 2rem;
    border-radius: 12px;
    border: 1px solid var(--border-light);
  }

  .avatar-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  /* Using the squircle mask, making it 128x128 */
  .avatar {
    width: 128px;
    height: 128px;
    background: var(--bg-base);
    clip-path: path('M 0,64 C 0,0 0,0 64,0 C 128,0 128,0 128,64 C 128,128 128,128 64,128 C 0,128 0,128 0,64 Z');
    display: flex;
    justify-content: center;
    align-items: center;
    overflow: hidden;
  }

  .avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .info-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
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

  input[type="text"] {
    background: var(--bg-base);
    border: 1px solid var(--border-light);
    color: var(--text-main);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    font-family: var(--font-body);
    font-size: 1rem;
    outline: none;
    transition: border-color 0.2s ease;
  }

  input[type="text"]:focus {
    border-color: var(--primary);
  }

  .help-text {
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .btn-outline {
    background: transparent;
    color: var(--text-main);
    border: 1px solid var(--border-light);
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: var(--font-body);
    font-size: 0.9rem;
    transition: all 0.2s ease;
  }

  .btn-outline:hover {
    border-color: var(--primary);
    color: var(--primary);
  }

  .stats-section {
    margin-top: 1rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .stat-card {
    padding: 1.5rem;
    border-radius: 12px;
    border: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .stat-label {
    font-size: 0.9rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-main);
    font-family: var(--font-heading);
  }

  .genres {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    font-family: var(--font-body);
  }

  .genre-tag {
    background: rgba(138, 43, 226, 0.15); /* Primary with opacity */
    color: var(--primary);
    padding: 0.25rem 0.75rem;
    border-radius: 100px;
    font-size: 0.85rem;
    font-weight: 500;
  }
</style>
