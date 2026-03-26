<script lang="ts">

  import { page } from '$app/stores';
  import Icon from '$lib/components/ui/Icon.svelte';
  import AnimatedDiscovery from '$lib/components/ui/animated-icons/AnimatedDiscovery.svelte';
  import AnimatedLibrary from '$lib/components/ui/animated-icons/AnimatedLibrary.svelte';
  import AnimatedPlaying from '$lib/components/ui/animated-icons/AnimatedPlaying.svelte';
  import AnimatedPlaylists from '$lib/components/ui/animated-icons/AnimatedPlaylists.svelte';
  import AnimatedSettings from '$lib/components/ui/animated-icons/AnimatedSettings.svelte';


  let isCollapsed = $state(false);
  

  const navItems = [
    { id: 'discovery', label: 'Discovery', component: AnimatedDiscovery },
    { id: 'library', label: 'Library', component: AnimatedLibrary },
    { id: 'playing', label: 'Now Playing', component: AnimatedPlaying },
    { id: 'playlists', label: 'Playlists', component: AnimatedPlaylists },
    { id: 'settings', label: 'Settings', component: AnimatedSettings }
  ];

  let hoveredItem = $state<string | null>(null);

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
      <Icon name="menu" size="24" />
    </button>
  </div>

  <div class="nav-section">

    {#each navItems as item}

      <a 
        href="/{item.id}"
        class="nav-item" 
        class:active={$page.url.pathname.startsWith('/' + item.id)}
        title={isCollapsed ? item.label : ''}
        onmouseenter={() => hoveredItem = item.id}
        onmouseleave={() => hoveredItem = null}
      >
        <div class="nav-icon-container">

          <item.component
            active={$page.url.pathname.startsWith('/' + item.id)}
            hovered={hoveredItem === item.id}
          />
        </div>
        {#if !isCollapsed}
          <span class="nav-label">{item.label}</span>
        {/if}
      </a>
    {/each}

  </div>

  <div class="sidebar-spacer"></div>

  <div class="sidebar-footer">
    <button class="add-folder-btn" class:collapsed={isCollapsed} aria-label="Add Media Folder">
      <Icon name="new-playlist" class="add-icon" />
      <span class="btn-text">Add Folder</span>
    </button>
    
    <div class="tmdb-credit" class:collapsed={isCollapsed}>
      <img src="/tmdb.svg" alt="TMDB" class="tmdb-logo" />
      <p class="credit-text">
        This product uses the TMDB API but is not endorsed or certified by TMDB.
      </p>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    grid-area: sidebar;
    background: var(--glass-bg-low);
    backdrop-filter: blur(25px);
    -webkit-backdrop-filter: blur(25px);
    border-right: 1px solid var(--glass-border-low);
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
    width: 64px;
    height: 64px;
    object-fit: contain;
  }

  .brand-info {
    display: flex;
    flex-direction: column;
  }

  .brand-name {
    font-family: var(--font-heading);
    font-size: 1.3rem;
    font-weight: 800;
    letter-spacing: 0.12em;
    color: var(--text-main);
    line-height: 1;
    text-transform: uppercase;
  }

  .version {
    font-size: 0.75rem;
    color: var(--primary);
    font-weight: 500;
    margin-top: 4px;
  }

  .menu-toggle {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
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
    background: var(--glass-bg-mid);
    border-color: var(--glass-border-high);
    color: var(--text-main);
    transform: scale(1.05);
  }

  :global(.menu-toggle svg) {
    width: 24px;
    height: 24px;
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
    text-decoration: none;
  }

  .nav-item:hover {
    color: var(--text-main);
    background: var(--glass-bg-low);
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

  .nav-icon-container {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
    transition: color 0.3s;
  }

  .nav-item.active .nav-icon-container {
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
    padding: 0 16px; /* Use vertical height instead of padding for stability */
    height: 56px; /* Fixed height to prevent vertical jumping */
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    font-weight: 700;
    font-family: var(--font-body);
    font-size: 0.9rem;
    letter-spacing: 0.02em;
    width: 100%;
  }

  .add-folder-btn:hover {
    background: rgba(0, 255, 255, 0.05);
    border-color: var(--secondary);
  }

  .add-folder-btn.collapsed {
    padding: 0;
    width: 56px;
    height: 56px; /* Same height as uncollapsed */
    margin: 0 auto;
    border-style: solid;
    gap: 0;
  }

  .btn-text {
    white-space: nowrap;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    opacity: 1;
    max-width: 200px;
  }

  .add-folder-btn.collapsed .btn-text {
    opacity: 0;
    max-width: 0;
    margin: 0;
  }

  :global(.add-icon) {
    width: 18px;
    height: 18px;
  }

  .tmdb-credit {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 12px;
    height: 56px; /* Match logo height to prevent jumping */
    opacity: 0.6;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
  }

  .tmdb-credit:hover {
    opacity: 1;
  }

  .tmdb-logo {
    height: 56px; /* Larger logo */
    width: auto;
    object-fit: contain;
    flex-shrink: 0;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .tmdb-credit.collapsed .tmdb-logo {
    align-self: center;
    height: 32px;
    margin: 0 auto;
  }

  .credit-text {
    font-size: 0.75rem;
    line-height: 1.4;
    color: var(--text-muted);
    margin: 0;
    text-align: left;
    overflow: hidden;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    opacity: 1;
    max-width: 400px;
  }

  .tmdb-credit.collapsed .credit-text {
    opacity: 0;
    max-width: 0;
    margin: 0;
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

    .sidebar:not(.collapsed) .nav-icon-container {
      margin: 0;
    }
  }
</style>
