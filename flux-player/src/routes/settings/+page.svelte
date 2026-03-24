<script lang="ts">
  import { onMount } from 'svelte';
  import ProfileSettings from '$lib/components/settings/ProfileSettings.svelte';
  import StorageSettings from '$lib/components/settings/StorageSettings.svelte';
  import PlaybackSettings from '$lib/components/settings/PlaybackSettings.svelte';
  import AppearanceSettings from '$lib/components/settings/AppearanceSettings.svelte';
  import StreamingSettings from '$lib/components/settings/StreamingSettings.svelte';

  const tabs = [
    { id: 'profile', label: 'My Profile', component: ProfileSettings },
    { id: 'storage', label: 'Storage & Library', component: StorageSettings },
    { id: 'playback', label: 'Playback & Performance', component: PlaybackSettings },
    { id: 'appearance', label: 'Appearance & UI', component: AppearanceSettings },
    { id: 'streaming', label: 'Streaming & Network', component: StreamingSettings }
  ];

  let activeTab = $state(tabs[0].id);

  function getActiveComponent() {
    return tabs.find(t => t.id === activeTab)?.component;
  }
</script>

<div class="settings-hub">
  <div class="settings-sidebar">
    <h1 class="settings-title">SETTINGS</h1>
    <nav class="settings-nav">
      {#each tabs as tab}
        <button
          class="nav-item {activeTab === tab.id ? 'active' : ''}"
          onclick={() => activeTab = tab.id}
        >
          {tab.label}
        </button>
      {/each}
    </nav>
  </div>

  <div class="settings-content">
    {#if activeTab === 'profile'}
      <ProfileSettings />
    {:else if activeTab === 'storage'}
      <StorageSettings />
    {:else if activeTab === 'playback'}
      <PlaybackSettings />
    {:else if activeTab === 'appearance'}
      <AppearanceSettings />
    {:else if activeTab === 'streaming'}
      <StreamingSettings />
    {/if}
  </div>
</div>

<style>
  .settings-hub {
    display: flex;
    height: 100%;
    color: var(--text-main);
    background: transparent;
  }

  .settings-sidebar {
    width: 280px;
    padding: 2.5rem 2rem;
    border-right: 1px solid var(--glass-border-low);
    display: flex;
    flex-direction: column;
    gap: 2.5rem;
    background: rgba(0, 0, 0, 0.2);
  }

  .settings-title {
    font-family: var(--font-heading);
    font-weight: 800;
    letter-spacing: 0.15em;
    font-size: 1.25rem;
    color: var(--secondary);
    margin: 0;
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .nav-item {
    background: transparent;
    border: none;
    color: var(--text-muted);
    text-align: left;
    padding: 0.85rem 1.25rem;
    border-radius: 10px;
    font-family: var(--font-body);
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: 1px solid transparent;
  }

  .nav-item:hover {
    color: var(--text-main);
    background: var(--glass-bg-low);
    border-color: var(--glass-border-low);
  }

  .nav-item.active {
    color: var(--secondary);
    background: var(--glass-bg-mid);
    font-weight: 700;
    border: 1px solid var(--glass-border-mid);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  }

  .settings-content {
    flex: 1;
    padding: 3rem;
    overflow-y: auto;
    background: var(--bg-base);
  }
</style>
