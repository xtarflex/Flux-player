<script lang="ts">
  import ProfileAvatar from './ProfileAvatar.svelte';
  
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

  <div class="sidebar-footer">
    <button class="collapse-toggle" onclick={() => isCollapsed = !isCollapsed}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        {#if isCollapsed}
          <path d="M13 17l5-5-5-5M6 17l5-5-5-5" />
        {:else}
          <path d="M11 17l-5-5 5-5M18 17l-5-5 5-5" />
        {/if}
      </svg>
    </button>
    
    <div class="account-item">
      <ProfileAvatar />
      {#if !isCollapsed}
        <div class="user-info">
          <span class="user-name">User</span>
          <span class="user-status">Online</span>
        </div>
      {/if}
    </div>
  </div>
</aside>

<style>
  .sidebar {
    grid-area: sidebar;
    background: var(--bg-surface);
    border-right: 1px solid var(--border-light);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    width: var(--sidebar-width);
    padding: 20px 12px;
    z-index: 100;
  }

  .sidebar.collapsed {
    width: var(--sidebar-collapsed-width);
  }

  .nav-section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .nav-item {
    background: none;
    border: none;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    width: 100%;
    text-align: left;
  }

  .nav-item:hover {
    background: rgba(255, 255, 255, 0.03);
    color: var(--text-main);
  }

  .nav-item.active {
    background: rgba(138, 43, 226, 0.15); /* Purple pill effect */
    color: var(--secondary);
    border-left: 2px solid var(--primary); /* Left accent */
    border-radius: 4px 12px 12px 4px;
  }

  .nav-icon {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
  }


  .nav-label {
    font-family: var(--font-body);
    font-size: 0.9rem;
    font-weight: 500;
  }

  .sidebar-footer {
    display: flex;
    flex-direction: column;
    gap: 20px;
    border-top: 1px solid var(--border-light);
    padding-top: 20px;
  }

  .collapse-toggle {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.2s;
  }

  .collapse-toggle:hover {
    color: var(--secondary);
  }

  .collapse-toggle svg {
    width: 20px;
    height: 20px;
  }

  .account-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
  }

  .user-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .user-name {
    font-size: 0.85rem;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .user-status {
    font-size: 0.7rem;
    color: var(--secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
</style>
