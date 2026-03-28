<script lang="ts">
  import { onMount } from 'svelte';
  import type { Snippet } from 'svelte';

  /**
   * EmptyState.svelte
   *
   * A premium, animated empty state component for Flux.
   *
   * @prop variant - Controls which illustration to render:
   *   - 'library'  — "Storage Node Cluster" for an empty media library
   *   - 'search'   — "Scanning Radar" for empty search/filter results
   *   - 'default'  — Generic geometric fallback
   * @prop title         - Heading text
   * @prop description   - Body text
   * @prop actionLabel   - CTA button label (optional)
   * @prop onAction      - CTA button handler (optional)
   * @prop illustration  - Custom Snippet to override the illustration entirely
   */
  let {
    variant = 'default',
    title,
    description,
    actionLabel = '',
    onAction = () => {},
    illustration,
  } = $props<{
    variant?: 'library' | 'search' | 'default';
    title: string;
    description: string;
    actionLabel?: string;
    onAction?: () => void;
    illustration?: Snippet;
  }>();

  // --- Parallax State ---
  let containerEl: HTMLDivElement;
  let parallaxX = $state(0);
  let parallaxY = $state(0);

  // Smoothed values using manual lerp via requestAnimationFrame
  let smoothX = 0;
  let smoothY = 0;
  let targetX = 0;
  let targetY = 0;
  let rafId: number;
  let reducedMotion = false;

  /**
   * Calculates normalized mouse offset relative to the container center.
   * @param e - Mouse event
   */
  function handleMouseMove(e: MouseEvent) {
    if (reducedMotion || !containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    targetX = ((e.clientX - rect.left) / rect.width - 0.5) * 20;
    targetY = ((e.clientY - rect.top) / rect.height - 0.5) * 20;
  }

  function handleMouseLeave() {
    targetX = 0;
    targetY = 0;
  }

  /**
   * Lerp-based smoothing loop for parallax motion.
   */
  function animate() {
    smoothX += (targetX - smoothX) * 0.08;
    smoothY += (targetY - smoothY) * 0.08;
    parallaxX = smoothX;
    parallaxY = smoothY;
    rafId = requestAnimationFrame(animate);
  }

  onMount(() => {
    reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    if (!reducedMotion) {
      rafId = requestAnimationFrame(animate);
    }
    return () => cancelAnimationFrame(rafId);
  });
</script>

<div
  class="empty-state"
  bind:this={containerEl}
  onmousemove={handleMouseMove}
  onmouseleave={handleMouseLeave}
  role="img"
  aria-label={title}
>
  <div class="illustration-container">
    <!-- Atmospheric Aurora Glow -->
    <div class="aurora" class:aurora-violet={variant === 'library'} class:aurora-cyan={variant === 'search'}></div>

    {#if illustration}
      {@render illustration()}
    {:else if variant === 'library'}
      <!-- ==========================================
           VARIANT: LIBRARY — "Storage Node Cluster"
           ========================================== -->
      <svg
        class="illus-svg"
        viewBox="0 0 200 180"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
      >
        <defs>
          <radialGradient id="core-glow" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stop-color="var(--primary)" stop-opacity="0.6" />
            <stop offset="100%" stop-color="var(--primary)" stop-opacity="0" />
          </radialGradient>
          <radialGradient id="node-glow" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stop-color="var(--secondary)" stop-opacity="0.5" />
            <stop offset="100%" stop-color="var(--secondary)" stop-opacity="0" />
          </radialGradient>
          <filter id="blur-sm">
            <feGaussianBlur stdDeviation="2" />
          </filter>
        </defs>

        <!-- Layer 1 (Back) — Connection grid lines, move slowest -->
        <g class="layer-back" style:transform="translate({parallaxX * 0.15}px, {parallaxY * 0.15}px)">
          <line x1="100" y1="90" x2="40" y2="50" stroke="var(--glass-border-high)" stroke-width="1" stroke-dasharray="4 4" class="conn-line" />
          <line x1="100" y1="90" x2="160" y2="50" stroke="var(--glass-border-high)" stroke-width="1" stroke-dasharray="4 4" class="conn-line delay-1" />
          <line x1="100" y1="90" x2="30" y2="130" stroke="var(--glass-border-high)" stroke-width="1" stroke-dasharray="4 4" class="conn-line delay-2" />
          <line x1="100" y1="90" x2="170" y2="130" stroke="var(--glass-border-high)" stroke-width="1" stroke-dasharray="4 4" class="conn-line delay-3" />
          <line x1="100" y1="90" x2="100" y2="30" stroke="var(--glass-border-high)" stroke-width="1" stroke-dasharray="4 4" class="conn-line delay-2" />

          <!-- Ambient data dots -->
          <circle cx="20" cy="25" r="2" fill="var(--primary)" opacity="0.25" />
          <circle cx="175" cy="160" r="3" fill="var(--secondary)" opacity="0.2" />
          <circle cx="185" cy="40" r="1.5" fill="var(--primary)" opacity="0.2" />
          <circle cx="15" cy="145" r="2" fill="var(--secondary)" opacity="0.2" />
        </g>

        <!-- Layer 2 (Mid) — Satellite nodes, move at medium speed -->
        <g class="layer-mid" style:transform="translate({parallaxX * 0.35}px, {parallaxY * 0.35}px)">
          <!-- Node A (top) -->
          <circle cx="100" cy="30" r="12" fill="rgba(138,43,226,0.1)" stroke="var(--primary)" stroke-width="1" class="satellite-node" />
          <path d="M96 26 L106 30 L96 34 Z" fill="var(--primary)" opacity="0.8" class="satellite-play" />

          <!-- Node B (bottom-left) -->
          <rect x="20" y="115" width="22" height="22" rx="4" fill="rgba(0,255,255,0.05)" stroke="var(--secondary)" stroke-width="1" class="satellite-node delay-1" />
          <line x1="24" y1="123" x2="38" y2="123" stroke="var(--secondary)" stroke-width="1.5" stroke-linecap="round" opacity="0.7" />
          <line x1="24" y1="128" x2="32" y2="128" stroke="var(--secondary)" stroke-width="1.5" stroke-linecap="round" opacity="0.5" />

          <!-- Node C (bottom-right) -->
          <rect x="158" y="115" width="22" height="22" rx="4" fill="rgba(138,43,226,0.05)" stroke="var(--primary)" stroke-width="1" class="satellite-node delay-2" />
          <circle cx="169" cy="126" r="5" stroke="var(--primary)" stroke-width="1" opacity="0.7" />
          <line x1="166" y1="126" x2="172" y2="126" stroke="var(--primary)" stroke-width="1" opacity="0.5" />

          <!-- Node D (top-left) -->
          <rect x="25" y="35" width="22" height="22" rx="4" fill="rgba(0,255,255,0.05)" stroke="var(--secondary)" stroke-width="1" class="satellite-node delay-3" />
          <circle cx="36" cy="46" r="6" stroke="var(--secondary)" stroke-width="1" opacity="0.7" />
          <path d="M33 43 L40 46 L33 49 Z" fill="var(--secondary)" opacity="0.6" />

          <!-- Node E (top-right) -->
          <rect x="153" y="35" width="22" height="22" rx="4" fill="rgba(138,43,226,0.05)" stroke="var(--primary)" stroke-width="1" class="satellite-node delay-1" />
          <line x1="157" y1="43" x2="171" y2="43" stroke="var(--primary)" stroke-width="1.5" stroke-linecap="round" opacity="0.7" />
          <line x1="157" y1="48" x2="165" y2="48" stroke="var(--primary)" stroke-width="1.5" stroke-linecap="round" opacity="0.5" />
        </g>

        <!-- Layer 3 (Front) — Core node, moves most -->
        <g class="layer-front" style:transform="translate({parallaxX * 0.6}px, {parallaxY * 0.6}px)">
          <!-- Outer glow ring -->
          <circle cx="100" cy="90" r="32" fill="url(#core-glow)" filter="url(#blur-sm)" class="core-pulse" />
          <!-- Glass disc -->
          <circle cx="100" cy="90" r="28" fill="rgba(138,43,226,0.08)" stroke="var(--primary)" stroke-width="1.5" />
          <!-- Inner ring -->
          <circle cx="100" cy="90" r="20" fill="rgba(138,43,226,0.06)" stroke="var(--primary)" stroke-width="1" stroke-dasharray="3 3" class="core-spin" />
          <!-- Central icon: folder + play -->
          <path d="M88 82 L88 99 L112 99 L112 82 Z" fill="rgba(138,43,226,0.15)" stroke="var(--primary)" stroke-width="1.5" stroke-linejoin="round" />
          <path d="M88 82 L88 79 L95 79 L98 82 Z" fill="rgba(138,43,226,0.15)" stroke="var(--primary)" stroke-width="1.5" stroke-linejoin="round" />
          <path d="M95 87 L105 91 L95 95 Z" fill="var(--secondary)" />
        </g>
      </svg>

    {:else if variant === 'search'}
      <!-- ==========================================
           VARIANT: SEARCH — "Scanning Radar"
           ========================================== -->
      <svg
        class="illus-svg"
        viewBox="0 0 200 200"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
      >
        <defs>
          <radialGradient id="radar-glow" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stop-color="var(--secondary)" stop-opacity="0.4" />
            <stop offset="100%" stop-color="var(--secondary)" stop-opacity="0" />
          </radialGradient>
          <linearGradient id="sweep-grad" x1="100" y1="100" x2="170" y2="100" gradientUnits="userSpaceOnUse">
            <stop offset="0%" stop-color="var(--secondary)" stop-opacity="0.7" />
            <stop offset="100%" stop-color="var(--secondary)" stop-opacity="0" />
          </linearGradient>
        </defs>

        <!-- Layer 1 (Back) — Grid -->
        <g class="layer-back" style:transform="translate({parallaxX * 0.1}px, {parallaxY * 0.1}px)" opacity="0.25">
          {#each [20, 40, 60, 80, 100, 120, 140, 160, 180] as x}
            <line x1={x} y1="0" x2={x} y2="200" stroke="var(--glass-border-high)" stroke-width="0.5" />
          {/each}
          {#each [20, 40, 60, 80, 100, 120, 140, 160, 180] as y}
            <line x1="0" y1={y} x2="200" y2={y} stroke="var(--glass-border-high)" stroke-width="0.5" />
          {/each}
          <!-- Dot particles -->
          <circle cx="55" cy="60" r="2" fill="var(--secondary)" opacity="0.3" class="radar-dot" />
          <circle cx="148" cy="72" r="2.5" fill="var(--primary)" opacity="0.3" class="radar-dot delay-1" />
          <circle cx="165" cy="140" r="2" fill="var(--secondary)" opacity="0.3" class="radar-dot delay-2" />
          <circle cx="40" cy="155" r="1.5" fill="var(--primary)" opacity="0.3" class="radar-dot delay-3" />
        </g>

        <!-- Layer 2 (Mid) — Concentric circles -->
        <g class="layer-mid" style:transform="translate({parallaxX * 0.3}px, {parallaxY * 0.3}px)">
          <circle cx="100" cy="100" r="70" stroke="var(--glass-border-mid)" stroke-width="1" />
          <circle cx="100" cy="100" r="50" stroke="var(--glass-border-mid)" stroke-width="1" />
          <circle cx="100" cy="100" r="30" stroke="var(--glass-border-mid)" stroke-width="1" />
          <!-- Crosshair lines -->
          <line x1="100" y1="25" x2="100" y2="175" stroke="var(--glass-border-low)" stroke-width="0.5" />
          <line x1="25" y1="100" x2="175" y2="100" stroke="var(--glass-border-low)" stroke-width="0.5" />
          <!-- Ambient glow background -->
          <circle cx="100" cy="100" r="70" fill="url(#radar-glow)" />
        </g>

        <!-- Layer 3 (Front) — Sweeping arm and core -->
        <g class="layer-front" style:transform="translate({parallaxX * 0.55}px, {parallaxY * 0.55}px)">
          <!-- Sweep arm -->
          <g class="radar-sweep" style:transform-origin="100px 100px">
            <!-- Sweep wedge -->
            <path d="M100 100 L170 80 A70 70 0 0 1 170 120 Z" fill="url(#sweep-grad)" opacity="0.5" />
            <!-- Leading edge line -->
            <line x1="100" y1="100" x2="170" y2="100" stroke="var(--secondary)" stroke-width="1.5" stroke-linecap="round" />
          </g>

          <!-- Center core -->
          <circle cx="100" cy="100" r="8" fill="rgba(0,255,255,0.1)" stroke="var(--secondary)" stroke-width="1.5" />
          <circle cx="100" cy="100" r="3" fill="var(--secondary)" class="core-pulse" />

          <!-- "Echo" blips — placeholder positions for "found nothing" -->
          <circle cx="142" cy="78" r="3" fill="none" stroke="var(--secondary)" stroke-width="1.5" opacity="0" class="blip delay-1" />
          <circle cx="68" cy="130" r="2.5" fill="none" stroke="var(--secondary)" stroke-width="1.5" opacity="0" class="blip delay-2" />
        </g>
      </svg>

    {:else}
      <!-- ==========================================
           VARIANT: DEFAULT — Simple geometric
           ========================================== -->
      <svg class="illus-svg" viewBox="0 0 120 120" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <g style:transform="translate({parallaxX * 0.4}px, {parallaxY * 0.4}px)">
          <circle cx="20" cy="20" r="4" fill="var(--secondary)" opacity="0.2" />
          <circle cx="100" cy="100" r="6" fill="var(--primary)" opacity="0.2" />
          <path class="slate-base" d="M30 45 L90 45 L90 95 L30 95 Z" stroke="var(--primary)" stroke-width="1.5" fill="rgba(138, 43, 226, 0.05)" />
          <path class="slate-tab" d="M30 45 L30 35 L50 35 L60 45 Z" stroke="var(--primary)" stroke-width="1.5" fill="rgba(138, 43, 226, 0.05)" />
          <circle cx="45" cy="70" r="10" stroke="var(--secondary)" stroke-width="1.5" class="data-node" />
          <path d="M42 66 L52 70 L42 74 Z" fill="var(--secondary)" />
          <path d="M65 65 L80 65" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4" class="data-line" />
          <path d="M65 75 L75 75" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4" class="data-line delay-1" />
        </g>
      </svg>
    {/if}
  </div>

  <h2 class="empty-title">{title}</h2>
  <p class="empty-description">{description}</p>

  {#if actionLabel}
    <button class="empty-action-btn" onclick={onAction}>
      <span>{actionLabel}</span>
    </button>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 48px 24px;
    height: 100%;
    min-height: 400px;
    animation: fadeIn 0.5s ease;
    cursor: default;
  }

  /* ==========================================
     Illustration Container
     ========================================== */
  .illustration-container {
    width: 260px;
    height: 260px;
    margin-bottom: 36px;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .illus-svg {
    width: 90%;
    height: 90%;
    position: relative;
    z-index: 1;
    overflow: visible;
  }

  /* Parallax layers use CSS transform inherited from inline style */
  .layer-back,
  .layer-mid,
  .layer-front {
    transition: transform 0.05s linear;
    will-change: transform;
  }

  /* ==========================================
     Aurora Atmospheric Glow
     ========================================== */
  .aurora {
    position: absolute;
    inset: -20%;
    border-radius: 50%;
    z-index: 0;
    animation: aurora-pulse 5s ease-in-out infinite alternate;
    pointer-events: none;
  }

  .aurora-violet {
    background: radial-gradient(ellipse at center, rgba(138, 43, 226, 0.15) 0%, transparent 70%);
    box-shadow: 0 0 60px 20px rgba(138, 43, 226, 0.08);
  }

  .aurora-cyan {
    background: radial-gradient(ellipse at center, rgba(0, 255, 255, 0.12) 0%, transparent 70%);
    box-shadow: 0 0 60px 20px rgba(0, 255, 255, 0.06);
  }

  .aurora:not(.aurora-violet):not(.aurora-cyan) {
    background: radial-gradient(ellipse at center, rgba(138, 43, 226, 0.1) 0%, transparent 70%);
  }

  /* ==========================================
     Animation: Library — Node Cluster
     ========================================== */
  .conn-line {
    stroke-dasharray: 80;
    stroke-dashoffset: 80;
    animation: draw-line 1.2s ease forwards;
  }
  .conn-line.delay-1 { animation-delay: 0.15s; }
  .conn-line.delay-2 { animation-delay: 0.3s; }
  .conn-line.delay-3 { animation-delay: 0.45s; }

  .satellite-node {
    opacity: 0;
    animation: node-appear 0.5s ease forwards 0.6s;
  }
  .satellite-node.delay-1 { animation-delay: 0.75s; }
  .satellite-node.delay-2 { animation-delay: 0.9s; }
  .satellite-node.delay-3 { animation-delay: 1.05s; }

  .satellite-play {
    opacity: 0;
    animation: node-appear 0.4s ease forwards 1.2s;
  }

  .core-spin {
    animation: spin 20s linear infinite;
    transform-origin: 100px 90px;
  }

  .core-pulse {
    animation: core-breathe 3s ease-in-out infinite alternate;
  }

  /* ==========================================
     Animation: Search — Radar
     ========================================== */
  .radar-sweep {
    animation: radar-rotate 3.5s linear infinite;
    transform-origin: 100px 100px;
  }

  .radar-dot {
    animation: dot-blink 4s ease-in-out infinite;
  }
  .radar-dot.delay-1 { animation-delay: 1s; }
  .radar-dot.delay-2 { animation-delay: 2s; }
  .radar-dot.delay-3 { animation-delay: 3s; }

  .blip {
    animation: blip-appear 3.5s ease-in-out infinite;
  }
  .blip.delay-1 { animation-delay: 0s; }
  .blip.delay-2 { animation-delay: 1.2s; }

  /* ==========================================
     Animation: Default
     ========================================== */
  .slate-base, .slate-tab {
    stroke-dasharray: 240;
    stroke-dashoffset: 240;
    animation: draw-line 2s ease forwards;
  }
  .slate-tab { animation-delay: 0.5s; }
  .data-node { animation: core-breathe 4s ease-in-out infinite alternate; }
  .data-line {
    stroke-dasharray: 20;
    stroke-dashoffset: 20;
    animation: draw-line 1.5s ease forwards 1s;
  }
  .data-line.delay-1 { animation-delay: 1.2s; }

  /* ==========================================
     Typography
     ========================================== */
  .empty-title {
    font-family: var(--font-heading);
    font-size: 1.4rem;
    font-weight: 700;
    color: var(--text-main);
    letter-spacing: 0.05em;
    margin-bottom: 12px;
    margin-top: 0;
  }

  .empty-description {
    font-family: var(--font-body);
    font-size: 0.95rem;
    color: var(--text-muted);
    max-width: 380px;
    line-height: 1.7;
    margin-bottom: 32px;
    margin-top: 0;
  }

  /* ==========================================
     CTA Button
     ========================================== */
  .empty-action-btn {
    background: var(--glass-bg-low);
    border: 1px solid var(--primary);
    color: var(--primary);
    padding: 12px 32px;
    border-radius: 8px;
    font-family: var(--font-heading);
    font-size: 0.85rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
    position: relative;
    overflow: hidden;
  }

  .empty-action-btn::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, var(--primary), var(--secondary));
    opacity: 0;
    transition: opacity 0.3s;
  }

  .empty-action-btn:hover::before {
    opacity: 1;
  }

  .empty-action-btn:hover {
    color: #0a0a0c;
    box-shadow: 0 4px 24px rgba(138, 43, 226, 0.5), 0 0 0 1px rgba(0, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .empty-action-btn span {
    position: relative;
    z-index: 1;
  }

  /* ==========================================
     Keyframes
     ========================================== */
  @keyframes draw-line {
    to { stroke-dashoffset: 0; }
  }

  @keyframes node-appear {
    from { opacity: 0; transform: scale(0.7); }
    to   { opacity: 1; transform: scale(1); }
  }

  @keyframes core-breathe {
    from { opacity: 0.7; transform: scale(1); }
    to   { opacity: 1;   transform: scale(1.08); }
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }

  @keyframes radar-rotate {
    from { transform: rotate(0deg); }
    to   { transform: rotate(360deg); }
  }

  @keyframes dot-blink {
    0%, 60%, 100% { opacity: 0.3; }
    30%            { opacity: 0.9; }
  }

  @keyframes blip-appear {
    0%, 45%, 100% { opacity: 0; r: 3; }
    50%            { opacity: 0.9; }
    55%            { opacity: 0.6; }
    65%            { opacity: 0; }
  }

  @keyframes aurora-pulse {
    from { opacity: 0.6; transform: scale(0.95); }
    to   { opacity: 1;   transform: scale(1.05); }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(12px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  /* ==========================================
     Accessibility: Reduced Motion
     ========================================== */
  @media (prefers-reduced-motion: reduce) {
    .conn-line, .satellite-node, .satellite-play,
    .core-spin, .core-pulse, .radar-sweep,
    .radar-dot, .blip, .slate-base, .slate-tab,
    .data-node, .data-line, .aurora, .empty-state {
      animation: none !important;
      transition: none !important;
      stroke-dashoffset: 0 !important;
      opacity: 1 !important;
    }
  }
</style>
