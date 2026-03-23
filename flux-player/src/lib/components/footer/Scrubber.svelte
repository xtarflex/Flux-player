<script lang="ts">
  let { progress = $bindable(0) } = $props<{ progress?: number }>();
  let isDragging = $state(false);

  function calculateProgress(e: PointerEvent, element: HTMLElement) {
    const rect = element.getBoundingClientRect();
    let newProgress = (e.clientX - rect.left) / rect.width;
    newProgress = Math.max(0, Math.min(1, newProgress));
    progress = newProgress;
  }

  function onPointerDown(e: PointerEvent) {
    isDragging = true;
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    calculateProgress(e, e.currentTarget as HTMLElement);
  }

  function onPointerMove(e: PointerEvent) {
    if (!isDragging) return;
    calculateProgress(e, e.currentTarget as HTMLElement);
  }

  function onPointerUp(e: PointerEvent) {
    isDragging = false;
    (e.currentTarget as HTMLElement).releasePointerCapture(e.pointerId);
  }
</script>

<div 
  class="scrubber-container"
  onpointerdown={onPointerDown}
  onpointermove={onPointerMove}
  onpointerup={onPointerUp}
  onpointercancel={onPointerUp}
  role="slider"
  aria-valuemin="0"
  aria-valuemax="100"
  aria-valuenow={Math.round(progress * 100)}
  tabindex="0"
>
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
