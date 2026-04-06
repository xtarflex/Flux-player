<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { onboarding, nextTourStep, completeTourSection, skipTour } from '$lib/stores/onboarding';
  import Icon from '$lib/components/ui/Icon.svelte';
  import { fade, scale } from 'svelte/transition';
  import { backOut } from 'svelte/easing';

  // Define the tour steps for different sections
  type TourStep = { id: string; title: string; description: string; position: 'top' | 'bottom' | 'left' | 'right'; requireClick?: boolean };
  
  const TOURS: Record<string, TourStep[]> = {
    global: [
      { id: 'onboard-sidebar-toggle', title: 'Navigation', description: 'Access your Library, Playlists, and Settings. Collapse it to save space.', position: 'right' },
      { id: 'onboard-connectivity', title: 'Connectivity Status', description: 'Flux works completely offline, but will sync metadata when online.', position: 'bottom' },
      { id: 'onboard-dynamic-island', title: 'The Island', description: 'This area adapts to show Now Playing metadata and system status.', position: 'bottom' },
    ],
    library: [
      { id: 'onboard-filters', title: 'Smart Filtering', description: 'Instantly filter out movies or music.', position: 'bottom' },
      { id: 'onboard-zoom', title: 'Grid Zoom', description: 'Adjust poster sizes with precision.', position: 'bottom' },
      { id: 'onboard-views', title: 'View Options', description: 'Switch between Grid, List, and Detail views.', position: 'bottom' },
      { id: 'onboard-detail-trigger', title: 'Detail Panel', description: 'Click here to open the inspector panel.', position: 'bottom', requireClick: true },
    ],
    player: [
      { id: 'onboard-player-controls', title: 'Playback Engine', description: 'Scrub through media smoothly.', position: 'top' },
      { id: 'onboard-pip-toggle', title: 'Picture-in-Picture', description: 'Pop the video out for multitasking.', position: 'top' }
    ]
  };

  let targetRect: DOMRect | null = $state(null);
  let activeStep = $derived.by(() => {
    if (!$onboarding.isActive || !$onboarding.currentSection) return null;
    const steps = TOURS[$onboarding.currentSection];
    if (!steps) return null;
    return steps[$onboarding.currentStep] || null;
  });

  // Calculate Spotlight CSS
  let clipPath = $derived.by(() => {
    if (!targetRect) return 'circle(0% at 50% 50%)'; // Hidden initially
    const { left, top, width, height } = targetRect;
    // Add some padding around the element
    const padding = 12;
    const x = left - padding;
    const y = top - padding;
    const w = width + padding * 2;
    const h = height + padding * 2;
    
    // Create an inverted rounded rectangle mask
    return `polygon(
      0% 0%, 0% 100%, 100% 100%, 100% 0%, 0% 0%,
      ${x}px ${y}px, ${x+w}px ${y}px, ${x+w}px ${y+h}px, ${x}px ${y+h}px, ${x}px ${y}px
    )`;
  });

  let tooltipStyle = $derived.by(() => {
    if (!targetRect || !activeStep) return { top: '50%', left: '50%' };
    const { left, top, width, height } = targetRect;
    
    // Position 20px away from the spotlight padding
    const offset = 20 + 12; 
    
    if (activeStep.position === 'right') {
      return { left: `${left + width + offset}px`, top: `${top + height/2}px`, transform: 'translateY(-50%)' };
    } else if (activeStep.position === 'left') {
      return { left: `${left - offset}px`, top: `${top + height/2}px`, transform: 'translate(-100%, -50%)' };
    } else if (activeStep.position === 'bottom') {
      return { left: `${left + width/2}px`, top: `${top + height + offset}px`, transform: 'translateX(-50%)' };
    } else { // top
      return { left: `${left + width/2}px`, top: `${top - offset}px`, transform: 'translate(-50%, -100%)' };
    }
  });

  async function updateTargetRect() {
    if (!activeStep) {
      targetRect = null;
      return;
    }
    
    // Allow DOM to update first (especially important if navigating routes)
    await tick();
    
    const el = document.getElementById(activeStep.id);
    if (el) {
      targetRect = el.getBoundingClientRect();
      
      // If the step requires a click, add an event listener to the target element
      if (activeStep.requireClick && !el.dataset.onboardHooked) {
        el.dataset.onboardHooked = 'true';
        el.addEventListener('click', handleRequiredClick, { once: true });
      }
    } else {
      console.warn(`[Onboarding] Target element not found: #${activeStep.id}`);
      // If we can't find the element, just show the tooltip in the center
      targetRect = { left: window.innerWidth/2, top: window.innerHeight/2, width: 0, height: 0, bottom: window.innerHeight/2, right: window.innerWidth/2, x: window.innerWidth/2, y: window.innerHeight/2, toJSON: () => {} };
    }
  }

  function handleRequiredClick() {
    // Wait a brief moment for the click action (e.g. state change) to register
    setTimeout(handleNext, 100);
  }

  // Reactive statement to update rect when the active step changes
  $effect(() => {
    updateTargetRect();
  });

  onMount(() => {
    // Handle resizing window during tour
    window.addEventListener('resize', updateTargetRect);
    return () => {
      window.removeEventListener('resize', updateTargetRect);
      // Clean up any stray event listeners
      document.querySelectorAll('[data-onboard-hooked]').forEach(el => {
        el.removeEventListener('click', handleRequiredClick);
        delete (el as HTMLElement).dataset.onboardHooked;
      });
    };
  });

  function handleNext() {
    if (!$onboarding.currentSection) return;
    const steps = TOURS[$onboarding.currentSection];
    if ($onboarding.currentStep < steps.length - 1) {
      nextTourStep();
    } else {
      completeTourSection();
    }
  }

</script>

{#if $onboarding.isActive && activeStep}
  <div class="spotlight-overlay" style="clip-path: {clipPath};" transition:fade={{ duration: 400 }}></div>
  
  <!-- We use a separate div for pointer-events so the underlying element is clickable if needed -->
  <div class="tour-interaction-layer">
    
    {#if targetRect}
      <div 
        class="guide-bubble glass" 
        style="left: {tooltipStyle.left}; top: {tooltipStyle.top}; transform: {tooltipStyle.transform};"
        in:scale={{ duration: 400, easing: backOut, start: 0.9 }}
      >
        <div class="bubble-header">
          <h3>{activeStep.title}</h3>
          <span class="step-counter">{$onboarding.currentStep + 1} / {TOURS[$onboarding.currentSection!].length}</span>
        </div>
        <p>{activeStep.description}</p>
        
        <div class="bubble-actions">
          <button class="btn-skip" onclick={(e) => { e.preventDefault(); skipTour(); }}>Skip Tour</button>
          {#if activeStep.requireClick}
            <span class="required-action-hint">
              <Icon name="check" size={14} /> Click to proceed
            </span>
          {:else}
            <button class="btn-next" onclick={(e) => { e.preventDefault(); handleNext(); }}>
              {$onboarding.currentStep === TOURS[$onboarding.currentSection!].length - 1 ? 'Finish' : 'Next'}
              <Icon name="chevron-right" size={14} />
            </button>
          {/if}
        </div>
      </div>
    {/if}

  </div>
{/if}

<style>
  .spotlight-overlay {
    position: fixed;
    inset: 0;
    z-index: 99998;
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    background: rgba(0, 0, 0, 0.4);
    pointer-events: none;
    transition: clip-path 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
    /* Inverted mask technique requires fill-rule: evenodd if using SVG, but polygon does it naturally by cutting out the middle if we draw the outer box then inner box in reverse. 
       Wait, standard polygon doesn't natively do hole-punching easily without complex wrapping.
       Actually, standard CSS polygon can do hole punching if you draw the outer perimeter, then go IN, draw the exact inner rectangle, and go OUT the exact same point.
       Let's refine the inline clip-path logic to ensure a perfect cutout.
    */
  }

  .tour-interaction-layer {
    position: fixed;
    inset: 0;
    z-index: 99999;
    pointer-events: none; /* Let clicks pass through to the spotlighted element */
  }

  .guide-bubble {
    position: absolute;
    width: 300px;
    padding: 20px;
    border-radius: 16px;
    background: rgba(20, 20, 25, 0.95); /* Opaque enough to read */
    border: 1px solid var(--primary);
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.5), 
                0 0 0 1px rgba(138, 43, 226, 0.4),
                0 0 30px rgba(138, 43, 226, 0.2);
    pointer-events: auto; /* Catch clicks inside the bubble */
    transition: all 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  /* Add a subtle breathing glow to the bubble */
  .guide-bubble::before {
    content: '';
    position: absolute;
    inset: -1px;
    border-radius: 16px;
    padding: 1px;
    background: linear-gradient(135deg, var(--secondary) 0%, transparent 50%, var(--primary) 100%);
    -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    opacity: 0.5;
    animation: breath 3s ease-in-out infinite alternate;
    pointer-events: none;
  }

  @keyframes breath {
    from { opacity: 0.3; }
    to { opacity: 1; }
  }

  .bubble-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }

  h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--secondary);
    font-family: var(--font-heading);
    letter-spacing: 0.05em;
  }

  .step-counter {
    font-size: 0.8rem;
    color: var(--text-muted);
    font-weight: 700;
  }

  p {
    margin: 0 0 20px 0;
    font-size: 0.95rem;
    color: var(--text-main);
    line-height: 1.4;
  }

  .bubble-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .btn-skip {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 0.85rem;
    cursor: pointer;
    font-family: var(--font-body);
  }

  .btn-skip:hover {
    color: var(--text-main);
  }

  .btn-next {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--glass-bg-high);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 6px 14px;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-next:hover {
    background: rgba(0, 255, 255, 0.15);
    border-color: var(--secondary);
    color: var(--secondary);
  }

  .required-action-hint {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 0.8rem;
    color: var(--primary);
    font-weight: 600;
    animation: pulse 2s infinite ease-in-out;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.6; }
    50% { opacity: 1; }
  }
</style>
