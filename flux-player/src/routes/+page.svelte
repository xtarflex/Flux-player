<script lang="ts">
  import Library from '$lib/components/Library.svelte';
</script>

<div class="page-wrapper">
  <Library />
</div>

<style>
  .page-wrapper {
    height: 100%;
    width: 100%;
    overflow: hidden; /* Let Library handle its own scrolling */
  }
</style>
