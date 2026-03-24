<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  onMount(() => {
    goto('/library', { replaceState: true });
  });
</script>

<style>
  /* Base styles */
</style>
