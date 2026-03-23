<script lang="ts">
  import '../app.css';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import DynamicIsland from '$lib/components/DynamicIsland.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import PlaybackFooter from '$lib/components/PlaybackFooter.svelte';
  let { children } = $props();
</script>

<div class="app-container">
  <Titlebar />
  <Sidebar />
  <DynamicIsland />
  <main class="main-content">
    {@render children()}
  </main>
  <PlaybackFooter />
</div>

<style>
  .main-content {
    grid-area: main;
    overflow-y: auto;
    padding: 20px;
  }
</style>
