<script lang="ts">
  let { title } = $props<{ title: string }>();
</script>

<div class="placeholder-view">
  <h2>{title}</h2>
  <p>Coming Soon</p>
</div>

<style>
  .placeholder-view {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    color: var(--text-muted);
    font-family: var(--font-heading);
  }
  
  h2 {
    font-size: 2rem;
    color: var(--text-main);
    margin-bottom: 8px;
    letter-spacing: 0.1em;
  }
</style>
