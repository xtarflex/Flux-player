<script lang="ts">
  let { src = "", alt = "User profile", size = "medium" } = $props();
</script>

<div class="avatar-container squircle" class:small={size === 'small'}>
  <img src={src || "/flux2d.png"} {alt} class="avatar-img" />
</div>

<style>
  .avatar-container {
    width: 32px;
    height: 32px;
    background: var(--glass-bg-low);
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    border: 1px solid var(--glass-border-low);
    transition: all 0.2s ease;
  }

  .avatar-container.small {
    width: 24px;
    height: 24px;
  }

  /* The .squircle class is defined in app.css using a path clip-mask */
  
  .avatar-container:hover {
    transform: scale(1.05);
    border-color: var(--secondary);
    background: var(--glass-bg-mid);
  }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: contain;
    padding: 2px;
  }
</style>
