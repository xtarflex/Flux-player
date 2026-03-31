<script lang="ts">
  import { tooltipState } from '$lib/stores/tooltip';

  let tooltipEl: HTMLElement | undefined = $state();
  
  let visible = $derived($tooltipState.visible);
  let content = $derived($tooltipState.content);
  let shortcut = $derived($tooltipState.shortcut);
  let baseX = $derived($tooltipState.x);
  let baseY = $derived($tooltipState.y);
  let basePlacement = $derived($tooltipState.placement);

  let finalX = $state(0);
  let finalY = $state(0);
  let placement = $state('bottom');

  // Trigger recalculation on visibility/pos changes
  $effect(() => {
    if (visible && tooltipEl) {
      calculatePosition();
    }
  });

  function calculatePosition() {
    if (!tooltipEl) return;
    
    // Reset to base placement
    placement = basePlacement;
    
    const tooltipRect = tooltipEl.getBoundingClientRect();
    const ww = window.innerWidth;
    const wh = window.innerHeight;
    
    const offset = 8; // Space from element
    
    let x = baseX;
    let y = baseY;

    // Edge Detection Adjustments
    if (placement === 'bottom') {
      x -= tooltipRect.width / 2;
      y += offset;
      if (y + tooltipRect.height > wh) {
        // Flipping to top
        placement = 'top';
        y = baseY - tooltipRect.height - offset;
      }
    } else if (placement === 'top') {
      x -= tooltipRect.width / 2;
      y -= (tooltipRect.height + offset);
      if (y < 0) {
        // Flipping to bottom
        placement = 'bottom';
        y = baseY + offset;
      }
    } else if (placement === 'left') {
      x -= (tooltipRect.width + offset);
      y -= tooltipRect.height / 2;
      if (x < 0) {
        placement = 'right';
        x = baseX + offset;
      }
    } else if (placement === 'right') {
      x += offset;
      y -= tooltipRect.height / 2;
      if (x + tooltipRect.width > ww) {
        placement = 'left';
        x = baseX - tooltipRect.width - offset;
      }
    }

    // Keep horizontally in bounds (for top/bottom)
    if (placement === 'top' || placement === 'bottom') {
      if (x < 10) x = 10;
      if (x + tooltipRect.width > ww - 10) x = ww - tooltipRect.width - 10;
    }

    // Keep vertically in bounds (for left/right)
    if (placement === 'left' || placement === 'right') {
      if (y < 10) y = 10;
      if (y + tooltipRect.height > wh - 10) y = wh - tooltipRect.height - 10;
    }

    finalX = x;
    finalY = y;
  }
</script>

{#if visible}
  <div 
    bind:this={tooltipEl}
    class="tooltip-bubble {placement}"
    style:left="{finalX}px"
    style:top="{finalY}px"
    role="tooltip"
  >
    <span class="content">{content}</span>
    {#if shortcut}
      <span class="divider"></span>
      <span class="shortcut">{shortcut}</span>
    {/if}
  </div>
{/if}

<style>
  .tooltip-bubble {
    position: fixed;
    z-index: 10000;
    background: var(--bg-surface);
    color: var(--text-main);
    border: 1px solid var(--glass-border-mid);
    padding: 8px 12px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    pointer-events: none;
    display: flex;
    align-items: center;
    gap: 8px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    
    /* Animation base */
    animation: fadeScale 0.15s ease-out;
    transform-origin: center;
    
    /* Max width logic */
    max-width: 250px;
    width: max-content;
  }

  @keyframes fadeScale {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .content {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .divider {
    width: 1px;
    height: 14px;
    background: var(--glass-border-mid);
  }

  .shortcut {
    color: var(--secondary);
    font-family: inherit;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 700;
    opacity: 0.8;
  }

  /* Speech Bubble Pointers */
  .tooltip-bubble::after {
    content: '';
    position: absolute;
    width: 8px;
    height: 8px;
    background: var(--bg-surface);
    border: 1px solid transparent;
    border-color: var(--glass-border-mid) transparent transparent var(--glass-border-mid);
    transform: rotate(45deg);
  }

  .tooltip-bubble.bottom::after {
    top: -5px;
    left: 50%;
    margin-left: -4px;
    border-color: var(--glass-border-mid) transparent transparent var(--glass-border-mid);
  }

  .tooltip-bubble.top::after {
    bottom: -5px;
    left: 50%;
    margin-left: -4px;
    border-color: transparent var(--glass-border-mid) var(--glass-border-mid) transparent;
  }

  .tooltip-bubble.right::after {
    left: -5px;
    top: 50%;
    margin-top: -4px;
    border-color: transparent transparent var(--glass-border-mid) var(--glass-border-mid);
  }

  .tooltip-bubble.left::after {
    right: -5px;
    top: 50%;
    margin-top: -4px;
    border-color: var(--glass-border-mid) var(--glass-border-mid) transparent transparent;
  }

</style>
