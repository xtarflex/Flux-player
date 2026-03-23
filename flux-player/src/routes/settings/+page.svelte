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
  }

  .settings-sidebar {
    width: 280px;
    padding: 2rem;
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .settings-title {
    font-family: 'Syncopate', sans-serif;
    font-weight: bold;
    letter-spacing: 0.05em;
    font-size: 1.5rem;
    color: var(--primary);
    margin: 0;
  }

  .settings-nav {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .nav-item {
    background: transparent;
    border: none;
    color: var(--text-muted);
    text-align: left;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    font-family: 'Outfit', system-ui, -apple-system, sans-serif;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .nav-item:hover {
    color: var(--text-main);
    background: var(--bg-surface);
  }

  .nav-item.active {
    color: var(--text-main);
    background: var(--bg-surface);
    font-weight: 600;
    border-left: 3px solid var(--primary);
  }

  .settings-content {
    flex: 1;
    padding: 3rem;
    overflow-y: auto;
  }
</style>
