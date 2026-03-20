<script lang="ts">
  let isCollapsed = $state(false);
  
  const navItems = [
    { id: 'discovery', label: 'Discovery', icon: 'M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z M9 12l2 2 4-4' },
    { id: 'library', label: 'Library', icon: 'M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z' },
    { id: 'playlists', label: 'Playlists', icon: 'M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01' },
    { id: 'settings', label: 'Settings', icon: 'M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z' }
  ];

  let activeTab = $state('discovery');
</script>

<aside class="sidebar" class:collapsed={isCollapsed}>
  <div class="sidebar-header">
    <div class="header-main">
      <img src="/flux.png" alt="Flux Logo" class="brand-logo" />
      <div class="brand-info">
        <span class="brand-name">FLUX</span>
        <span class="version">V0.1.0</span>
      </div>
    </div>
    <button class="menu-toggle" onclick={() => isCollapsed = !isCollapsed} aria-label="Toggle Sidebar">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </button>
  </div>

  <div class="nav-section">
    {#each navItems as item}
      <button 
        class="nav-item" 
        class:active={activeTab === item.id}
        onclick={() => activeTab = item.id}
        title={isCollapsed ? item.label : ''}
      >
        <svg class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d={item.icon} stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        {#if !isCollapsed}
          <span class="nav-label">{item.label}</span>
        {/if}
      </button>
    {/each}
  </div>

  <div class="sidebar-spacer"></div>

  <div class="sidebar-footer">
    <button class="add-folder-btn" class:collapsed={isCollapsed} aria-label="Add Media Folder">
      <svg class="add-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v11z" />
        <path d="M12 11v6m-3-3h6" stroke-width="2" />
      </svg>
      {#if !isCollapsed}
        <span>Add Folder</span>
      {/if}
    </button>
    
    <div class="tmdb-credit" class:collapsed={isCollapsed}>
      <img src="/tmdb.svg" alt="TMDB" class="tmdb-logo" />
      {#if !isCollapsed}
        <p class="credit-text">
          This product uses the TMDB API but is not endorsed or certified by TMDB.
        </p>
      {/if}
    </div>
  </div>
</aside>

<style>
  .sidebar {
    grid-area: sidebar;
    background: var(--bg-base);
    border-right: 1px solid rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    width: var(--sidebar-width);
    padding: 24px 16px;
    z-index: 100;
  }

  .sidebar.collapsed {
    width: var(--sidebar-collapsed-width);
    padding: 24px 12px;
  }

  /* Header Styles */
  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 40px;
    padding: 0 4px;
  }

  .header-main {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .brand-logo {
    width: 56px;
    height: 56px;
    object-fit: contain;
  }

  .brand-info {
    display: flex;
    flex-direction: column;
  }

  .brand-name {
    font-family: var(--font-heading);
    font-size: 1.2rem;
    font-weight: 700;
    letter-spacing: 0.15em;
    color: var(--text-main);
    line-height: 1;
    text-transform: uppercase;
  }

  .version {
    font-size: 0.7rem;
    color: var(--text-muted);
    font-weight: 500;
    margin-top: 4px;
  }

  .menu-toggle {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: var(--text-muted);
    padding: 8px;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .menu-toggle:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.2);
    color: var(--text-main);
    transform: scale(1.05);
  }

  .menu-toggle svg {
    width: 18px;
    height: 18px;
  }

  /* Navigation Styles */
  .nav-section {
    display: flex;
    flex-direction: column;
    gap: 16px; /* Slightly more space like screenshot */
  }

  .nav-item {
    background: none;
    border: none;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    width: 100%;
    text-align: left;
    position: relative;
    overflow: hidden;
    font-family: var(--font-body);
    font-weight: 500;
  }

  .nav-item:hover {
    color: var(--text-main);
    background: rgba(255, 255, 255, 0.03);
  }

  .nav-item.active {
    background: rgba(138, 43, 226, 0.15); /* Reverted to original beautiful look */
    color: var(--secondary);
    font-weight: 600;
    border-radius: 4px 12px 12px 4px;
  }

  .nav-item.active::before {
    content: '';
    position: absolute;
    left: 0;
    top: 20%;
    height: 60%;
    width: 2px;
    background: var(--primary);
    border-radius: 0 4px 4px 0;
  }

  .nav-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
    transition: color 0.3s;
  }

  .nav-item.active .nav-icon {
    color: var(--secondary);
  }

  .nav-label {
    font-size: 1.05rem;
  }

  /* Spacer */
  .sidebar-spacer {
    flex-grow: 1;
  }

  /* Footer Styles */
  .sidebar-footer {
    display: flex;
    flex-direction: column;
    gap: 24px;
    margin-top: 24px;
  }

  .add-folder-btn {
    background: transparent;
    border: 1.5px dashed var(--secondary); /* Cyan dashed border */
    color: var(--secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 16px;
    border-radius: 14px;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-weight: 700;
    font-family: var(--font-body);
    font-size: 0.9rem;
    letter-spacing: 0.02em;
  }

  .add-folder-btn:hover {
    background: rgba(0, 255, 255, 0.05);
    border-color: var(--secondary);
  }

  .add-folder-btn.collapsed {
    padding: 14px 0;
    border-style: solid;
  }

  .add-icon {
    width: 18px;
    height: 18px;
  }

  .tmdb-credit {
    display: flex;
    flex-direction: row; /* Side-by-side */
    align-items: center; /* Center vertically */
    gap: 12px;
    opacity: 0.6;
    transition: opacity 0.2s;
  }

  .tmdb-credit:hover {
    opacity: 1;
  }

  .tmdb-logo {
    height: 36px; /* Larger logo */
    width: auto;
    object-fit: contain;
    flex-shrink: 0;
  }

  .tmdb-credit.collapsed .tmdb-logo {
    align-self: center;
    height: 16px;
  }

  .credit-text {
    font-size: 0.65rem;
    line-height: 1.3;
    color: var(--text-muted);
    margin: 0;
    text-align: left;
  }

  /* Transitions for Collapsed State */
  .sidebar.collapsed .brand-info,
  .sidebar.collapsed .brand-name,
  .sidebar.collapsed .version,
  .sidebar.collapsed .nav-label {
    display: none;
  }

  .sidebar.collapsed .sidebar-header {
    justify-content: center;
  }

  .sidebar.collapsed .header-main {
    display: none;
  }

  /* Responsive Auto-Collapse */
  @media (max-width: 1000px) {
    .sidebar:not(.collapsed) {
      width: var(--sidebar-collapsed-width);
      padding: 24px 12px;
    }
    
    .sidebar:not(.collapsed) .brand-info,
    .sidebar:not(.collapsed) .brand-name,
    .sidebar:not(.collapsed) .version,
    .sidebar:not(.collapsed) .nav-label,
    .sidebar:not(.collapsed) .header-main,
    .sidebar:not(.collapsed) span {
      display: none;
    }

    .sidebar:not(.collapsed) .sidebar-header {
      justify-content: center;
    }
    
    .sidebar:not(.collapsed) .nav-item {
      padding: 10px;
      justify-content: center;
    }

    .sidebar:not(.collapsed) .nav-icon {
      margin: 0;
    }
  }
</style>
