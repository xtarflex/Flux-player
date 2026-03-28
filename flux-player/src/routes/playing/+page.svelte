<script lang="ts">
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
</script>

<div class="playing-container">
  <EmptyState 
    variant="player"
    title="Ready for Input" 
    description="The Flux Core is in standby. Select a media file from your library or drop one here to begin the experience."
    actionLabel="Open Library"
    onAction={() => window.dispatchEvent(new CustomEvent('flux-search-focus'))}
  />
</div>

<style>
  .playing-container {
    height: 100%;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: radial-gradient(circle at center, rgba(138, 43, 226, 0.05) 0%, transparent 80%);
  }
</style>
