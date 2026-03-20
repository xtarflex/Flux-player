<script lang="ts">
  import '../app.css';
  import Titlebar from '$lib/components/Titlebar.svelte';
  import DynamicIsland from '$lib/components/DynamicIsland.svelte';
  let { children } = $props();
</script>

<div class="app-container">
  <Titlebar />
  <DynamicIsland />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<style>
  .main-content {
    grid-area: main;
    overflow-y: auto;
    padding: 20px;
  }
</style>
