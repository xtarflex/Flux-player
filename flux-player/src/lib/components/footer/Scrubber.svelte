<script lang="ts">
  let { progress = $bindable(0) } = $props<{ progress?: number }>();
</script>

<div class="scrubber-container">
  <div class="scrubber-track">
    <div class="scrubber-fill" style="width: {progress * 100}%"></div>
    <div class="scrubber-thumb" style="left: {progress * 100}%"></div>
  </div>
</div>

<style>
  .scrubber-container {
    position: relative;
    width: 100%;
    height: 6px;
    cursor: pointer;
    transition: height 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    z-index: 10;
    margin-bottom: 8px;
  }

  .scrubber-container:hover {
    height: 8px;
  }

  .scrubber-track {
    position: relative;
    width: 100%;
    height: 100%;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
  }

  .scrubber-fill {
    height: 100%;
    background: linear-gradient(90deg, #8a2be2, #00ffff);
    transition: height 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 4px;
  }

  .scrubber-thumb {
    position: absolute;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 12px;
    height: 12px;
    background: white;
    border-radius: 50%;
    opacity: 0;
    transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1), transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .scrubber-container:hover .scrubber-thumb {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1.1);
  }
</style>
