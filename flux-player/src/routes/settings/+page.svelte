<script lang="ts">
  import { settings, updateSetting } from '$lib/stores/settings';
  import Icon from '$lib/components/ui/Icon.svelte';
  import ProfileSettings from '$lib/components/settings/ProfileSettings.svelte';
  import AppearanceSettings from '$lib/components/settings/AppearanceSettings.svelte';
  import PlaybackSettings from '$lib/components/settings/PlaybackSettings.svelte';
  import StorageSettings from '$lib/components/settings/StorageSettings.svelte';
  import StreamingSettings from '$lib/components/settings/StreamingSettings.svelte';
  import ShortcutSettings from '$lib/components/settings/ShortcutSettings.svelte';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let activeCategory = $state('appearance');

  const categories = [
    { id: 'profile', label: 'Profile', icon: 'profile', description: 'Account & preferences' },
    { id: 'appearance', label: 'Appearance', icon: 'appearance', description: 'UI & Theme' },
    { id: 'playback', label: 'Performance', icon: 'playback', description: 'Engine & Queue' },
    { id: 'storage', label: 'Storage', icon: 'storage', description: 'Library folders' },
    { id: 'streaming', label: 'Streaming', icon: 'streaming', description: 'TMDB & APIs' },
    { id: 'shortcuts', label: 'Shortcuts', icon: 'keyboard', description: 'Global keys' }
  ];

  function setCategory(id: string) {
    activeCategory = id;
    window.scrollTo({ top: 0, behavior: 'smooth' });
  }

  function handleKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement;
    const isEditing = target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable;
    if (isEditing) return;

    if (!e.ctrlKey && !e.altKey && !e.metaKey) {
      if (e.key >= '1' && e.key <= '6') {
        const idx = parseInt(e.key) - 1;
        if (categories[idx]) setCategory(categories[idx].id);
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<svelte:head>
  <title>Settings | Flux</title>
</svelte:head>

<div class="settings-hub">
  <!-- Detailed Settings View -->
  <main class="settings-viewport">
    <!-- Sticky Content Header -->
    <header class="hub-header">
      <div class="header-main">
        <h1>Settings Hub</h1>
        <p>Fine-tune your Flux environment</p>
      </div>
    </header>

    <div class="content-wrapper">
      {#if activeCategory === 'profile'}
        <ProfileSettings />
      {:else if activeCategory === 'appearance'}
        <AppearanceSettings />
      {:else if activeCategory === 'playback'}
        <PlaybackSettings />
      {:else if activeCategory === 'storage'}
        <StorageSettings />
      {:else if activeCategory === 'streaming'}
        <StreamingSettings />
      {:else if activeCategory === 'shortcuts'}
        <ShortcutSettings />
      {/if}
    </div>
  </main>

  <!-- Bottom Navigation (Footer Nav) -->
  <nav class="settings-footer-nav">
    <div class="nav-container">
      <div class="nav-logo" onclick={() => goto('/')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && goto('/')}>
        <img src="/flux.png" alt="Flux" class="mini-logo" />
        <div class="logo-meta">
          <span class="brand">FLUX</span>
          <span class="version-badge">BETA V0.2.0</span>
        </div>
      </div>

      <div class="nav-items-right">
        {#each categories as cat}
          <button 
            class="footer-nav-item" 
            class:active={activeCategory === cat.id}
            onclick={() => setCategory(cat.id)}
          >
            <div class="icon-orb">
              <Icon name={cat.icon} size={20} />
            </div>
            <div class="label-stack">
              <span class="label-text">{cat.label}</span>
              <span class="desc-text">{cat.description}</span>
            </div>
          </button>
        {/each}
      </div>
    </div>
  </nav>
</div>

<style>
  .settings-hub {
    display: flex;
    flex-direction: column;
    height: 100%; /* Fill the grid area */
    width: 100%;
    background: var(--bg-base);
    overflow: hidden;
    position: relative;
  }

  /* ── Header ────────────────── */
  .hub-header {
    position: sticky;
    top: 0;
    padding: 32px 48px 24px;
    background: rgba(10, 10, 12, 0.4);
    backdrop-filter: blur(20px) saturate(180%);
    -webkit-backdrop-filter: blur(20px) saturate(180%);
    border-bottom: 1px solid var(--glass-border-low);
    margin: 0 -48px 32px;
    z-index: 50;
  }

  .header-main h1 {
    font-family: var(--font-heading);
    font-size: 1.8rem;
    margin: 0;
    color: var(--text-main);
    background: linear-gradient(135deg, #fff 0%, var(--secondary) 100%);
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .header-main p {
    color: var(--text-muted);
    font-size: 0.9rem;
    margin: 8px 0 0;
    letter-spacing: 0.02em;
  }

  /* ── Content Area ──────────── */
  .settings-viewport {
    flex-grow: 1;
    overflow-y: auto;
    padding: 0 48px 160px; /* Increased bottom padding for nav clearance */
    scrollbar-width: thin;
    scrollbar-color: var(--glass-border-high) transparent;
  }

  .content-wrapper {
    max-width: 900px;
    margin: 0 auto;
    animation: slideIn 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }

  @keyframes slideIn {
    from { opacity: 0; transform: translateY(30px); }
    to { opacity: 1; transform: translateY(0); }
  }

  /* ── Bottom Navigation ──────── */
  .settings-footer-nav {
    position: absolute;
    bottom: 24px;
    left: 24px;
    right: 24px;
    height: 80px;
    background: rgba(20, 20, 25, 0.85); 
    backdrop-filter: blur(30px) saturate(180%);
    -webkit-backdrop-filter: blur(30px) saturate(180%);
    border: 1px solid rgba(0, 255, 255, 0.2); 
    border-radius: 24px;
    z-index: 1000;
    display: flex;
    align-items: center;
    padding: 0 12px;
    animation: slideUpNav 0.6s cubic-bezier(0.23, 1, 0.32, 1);
  }

  @keyframes slideUpNav {
    from { transform: translateY(100px); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
  }

  .nav-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between; /* Space between logo and nav items */
    padding: 0 12px;
  }

  /* ── Logo Section ───────────── */
  .nav-logo {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 16px;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid transparent;
  }

  .nav-logo:hover {
    background: rgba(255, 255, 255, 0.06);
    border-color: rgba(0, 255, 255, 0.2);
    transform: translateX(4px);
  }

  .mini-logo {
    width: 32px;
    height: 32px;
    object-fit: contain;
  }

  .logo-meta {
    display: flex;
    flex-direction: column;
    line-height: 1.1;
  }

  .brand {
    font-family: var(--font-heading);
    font-size: 0.9rem;
    font-weight: 800;
    letter-spacing: 0.1em;
    color: var(--text-main);
  }

  .version-badge {
    font-family: var(--font-heading);
    font-size: 0.55rem;
    font-weight: 700;
    color: var(--secondary);
    opacity: 0.8;
  }

  .nav-items-right {
    display: flex;
    gap: 8px;
    height: 100%;
    align-items: center;
    overflow-x: auto;
    scrollbar-width: none;
  }

  .nav-items-right::-webkit-scrollbar { display: none; }

  .footer-nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 16px;
    border-radius: 16px;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.23, 1, 0.32, 1);
    min-width: auto;
  }

  .footer-nav-item:hover {
    background: var(--glass-bg-low);
    color: var(--text-main);
  }

  .footer-nav-item.active {
    background: rgba(0, 255, 255, 0.08); 
    border-color: rgba(0, 255, 255, 0.3);
    color: var(--secondary); 
  }

  .icon-orb {
    width: 38px;
    height: 38px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--glass-bg-low);
    border-radius: 12px;
    transition: all 0.3s ease;
  }

  .footer-nav-item.active .icon-orb {
    background: var(--secondary);
    color: #000;
    transform: scale(1.05);
  }

  .label-stack {
    display: flex;
    flex-direction: column;
    text-align: left;
  }

  .label-text {
    font-family: var(--font-heading);
    font-size: 0.8rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .desc-text {
    font-size: 0.65rem;
    opacity: 0.6;
    margin-top: 2px;
    display: none; /* Hide by default for compact look */
  }

  /* ── Desktop Bloom (Only show descriptions on very wide screens) ── */
  @media (min-width: 1500px) {
    .desc-text { display: block; }
    .footer-nav-item { 
      min-width: 160px; 
      padding: 10px 20px;
      gap: 14px;
    }
  }

  /* ── Responsive ──────────────── */
  @media (max-width: 1150px) {
    .nav-logo { gap: 8px; padding: 8px 12px; }
    .brand { font-size: 0.8rem; }
  }

  @media (max-width: 950px) {
    .nav-items-right { 
      gap: 4px;
      overflow-x: auto;
      scrollbar-width: none; 
    }
    .nav-items-right::-webkit-scrollbar { display: none; }
    .footer-nav-item { padding: 8px 12px; }
  }

  @media (max-width: 850px) {
    .settings-viewport { padding: 0 24px 120px; }
    .hub-header { padding: 32px 24px 16px; }

    .settings-footer-nav {
      left: 12px;
      right: 12px;
      bottom: 12px;
      height: 70px;
      border-radius: 20px;
    }

    .nav-container { 
      justify-content: center; 
      gap: 16px;
    } 

    .nav-logo { display: none; } /* Hide logo on mobile to save space */
    .nav-items-right { gap: 4px; overflow-x: auto; scrollbar-width: none; }
    .nav-items-right::-webkit-scrollbar { display: none; }

    .footer-nav-item { 
      flex-direction: column;
      gap: 6px;
      padding: 8px;
      min-width: 65px;
      text-align: center;
    }

    .icon-orb { width: 32px; height: 32px; }
    .label-stack { text-align: center; }
    .label-text { font-size: 0.6rem; }
  }
</style>
