const fs = require('fs');

const svgContent = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 500" width="100%" height="100%">
  <!-- Definitions for animations and gradients -->
  <defs>
    <linearGradient id="primaryGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#00ffff" />
      <stop offset="100%" stop-color="#0088ff" />
    </linearGradient>
    <filter id="glow">
      <feGaussianBlur stdDeviation="3" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
  </defs>

  <!-- Group for the whole illustration to apply overall animation -->
  <g id="illustration-group" class="illustration-group">

    <!-- Top left cloud -->
    <path class="cloud cloud-top" d="M 120 100 Q 120 80 140 80 Q 150 60 170 60 Q 190 60 200 80 Q 220 80 220 100 Z" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>

    <!-- Top left sparkle -->
    <path class="sparkle sparkle-1" d="M 100 150 L 110 150 M 105 145 L 105 155 M 105 150 L 105 150" stroke="#00ffff" stroke-width="3" stroke-linecap="round" fill="none"/>
    <g class="sparkle sparkle-1-group">
        <line x1="105" y1="135" x2="105" y2="165" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <line x1="90" y1="150" x2="120" y2="150" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
    </g>

    <!-- Gear -->
    <g class="gear" transform="translate(100, 240)">
      <circle cx="0" cy="0" r="25" fill="none" stroke="#00ffff" stroke-width="4"/>
      <circle cx="0" cy="0" r="10" fill="none" stroke="#00ffff" stroke-width="4"/>
      <path d="M -8 -30 L 8 -30 L 10 -25 L -10 -25 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M -8 30 L 8 30 L 10 25 L -10 25 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M -30 -8 L -30 8 L -25 10 L -25 -10 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M 30 -8 L 30 8 L 25 10 L 25 -10 Z" fill="none" stroke="#00ffff" stroke-width="3"/>

      <path d="M -22 -22 L -12 -32 L -8 -28 L -18 -18 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M 22 22 L 12 32 L 8 28 L 18 18 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M 22 -22 L 32 -12 L 28 -8 L 18 -18 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
      <path d="M -22 22 L -32 12 L -28 8 L -18 18 Z" fill="none" stroke="#00ffff" stroke-width="3"/>
    </g>

    <!-- Bottom right cloud -->
    <path class="cloud cloud-bottom" d="M 280 430 Q 280 390 320 390 Q 340 360 380 360 Q 420 360 440 390 Q 480 390 480 430 Z" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>

    <!-- Bottom cross -->
    <g class="sparkle sparkle-2-group">
        <line x1="220" y1="380" x2="240" y2="400" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <line x1="240" y1="380" x2="220" y2="400" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
    </g>

    <!-- Right sparkle -->
    <g class="sparkle sparkle-3-group">
        <line x1="430" y1="330" x2="430" y2="360" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <line x1="415" y1="345" x2="445" y2="345" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
    </g>

    <!-- Magnifying Glass -->
    <g class="magnifying-glass">
      <!-- Handle -->
      <g class="handle">
        <path d="M 190 310 L 80 420 L 110 450 L 220 340 Z" fill="#00ffff" stroke="#00ffff" stroke-width="4" stroke-linejoin="round"/>
        <path d="M 120 380 L 150 410" stroke="#ffffff" stroke-width="20" stroke-linecap="square"/>
        <path d="M 160 340 L 190 370" stroke="#ffffff" stroke-width="20" stroke-linecap="square"/>
        <path d="M 190 310 L 220 340" stroke="#0055ff" stroke-width="15" stroke-linecap="square"/>
      </g>

      <!-- Glass Rim -->
      <circle cx="280" cy="200" r="110" fill="none" stroke="#00ffff" stroke-width="12"/>
      <circle cx="280" cy="200" r="95" fill="none" stroke="#00aaff" stroke-width="4"/>

      <!-- Glass glare/reflection -->
      <path d="M 185 200 A 95 95 0 0 0 280 295" fill="none" stroke="#00ffff" stroke-width="15" opacity="0.4" stroke-linecap="round"/>
      <path d="M 365 200 A 85 85 0 0 0 280 115" fill="none" stroke="#00ffff" stroke-width="8" opacity="0.4" stroke-linecap="round"/>
      <path d="M 370 200 A 90 90 0 0 0 280 110" fill="none" stroke="#00ffff" stroke-width="4" opacity="0.4" stroke-linecap="round"/>

      <!-- Detail lines on rim -->
      <path d="M 385 180 L 385 220" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
      <path d="M 378 240 L 372 260" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>

      <!-- Bug inside -->
      <g class="bug">
        <!-- Antennae -->
        <path d="M 270 145 Q 265 130 250 135" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <path d="M 290 145 Q 295 130 310 135" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>

        <!-- Legs Left -->
        <path d="M 245 170 Q 230 160 220 180" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <path d="M 240 200 Q 220 200 215 220" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <path d="M 245 230 Q 225 240 225 260" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>

        <!-- Legs Right -->
        <path d="M 315 170 Q 330 160 340 180" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <path d="M 320 200 Q 340 200 345 220" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>
        <path d="M 315 230 Q 335 240 335 260" fill="none" stroke="#00ffff" stroke-width="4" stroke-linecap="round"/>

        <!-- Body base -->
        <ellipse cx="280" cy="205" rx="40" ry="55" fill="none" stroke="#00ffff" stroke-width="5"/>

        <!-- Wings separator -->
        <path d="M 280 150 L 280 260" fill="none" stroke="#00ffff" stroke-width="4"/>

        <!-- Head -->
        <circle cx="280" cy="145" r="12" fill="none" stroke="#00ffff" stroke-width="4"/>
        <!-- Eyes -->
        <circle cx="275" cy="142" r="2" fill="#00ffff"/>
        <circle cx="285" cy="142" r="2" fill="#00ffff"/>

        <!-- Dots on wings -->
        <!-- Left wing dots -->
        <circle cx="260" cy="180" r="4" fill="#00ffff"/>
        <circle cx="265" cy="210" r="4" fill="#00ffff"/>
        <circle cx="255" cy="235" r="4" fill="#00ffff"/>
        <!-- Right wing dots -->
        <circle cx="300" cy="180" r="4" fill="#00ffff"/>
        <circle cx="295" cy="210" r="4" fill="#00ffff"/>
        <circle cx="305" cy="235" r="4" fill="#00ffff"/>
      </g>
    </g>

    <!-- Exclamation Alert -->
    <g class="alert-icon" transform="translate(380, 80)">
      <circle cx="40" cy="40" r="30" fill="none" stroke="#00ffff" stroke-width="4"/>
      <path d="M 40 25 L 40 45" fill="none" stroke="#00ffff" stroke-width="6" stroke-linecap="round"/>
      <circle cx="40" cy="55" r="3" fill="#00ffff"/>
    </g>

  </g>
</svg>`;

const componentContent = `<script lang="ts">
  // Bug Illustration Component
  // Animated SVG that responds to hover state on parent cards
</script>

<div class="bug-illustration-container">
  ${svgContent}
</div>

<style>
  .bug-illustration-container {
    display: block;
    width: 100%;
    height: 100%;
    position: relative;
  }

  svg {
    width: 100%;
    height: 100%;
    display: block;
    filter: drop-shadow(0 0 10px rgba(0, 255, 255, 0.2));
  }

  /* Default idle animations */
  .cloud {
    animation: float 6s ease-in-out infinite;
  }

  .cloud-top {
    animation-delay: 0s;
  }

  .cloud-bottom {
    animation-delay: -3s;
  }

  .sparkle-1-group { animation: pulse 4s infinite; }
  .sparkle-2-group { animation: pulse 4s infinite 1s; }
  .sparkle-3-group { animation: pulse 4s infinite 2s; }

  .gear {
    transform-origin: 100px 240px;
    animation: spinIdle 20s linear infinite;
  }

  .bug {
    transform-origin: 280px 200px;
    animation: bugWiggleIdle 5s ease-in-out infinite;
  }

  .alert-icon {
    transform-origin: 420px 120px;
  }

  /* Hover Choreography (Triggered by parent group) */
  :global(.support-card:hover) .magnifying-glass {
    animation: glassSearch 1.5s ease-in-out;
  }

  :global(.support-card:hover) .bug {
    animation: bugPanic 0.5s ease-in-out infinite;
  }

  :global(.support-card:hover) .gear {
    animation: spinActive 2s linear infinite;
  }

  :global(.support-card:hover) .alert-icon {
    animation: alertBounce 1s cubic-bezier(0.175, 0.885, 0.32, 1.275) infinite;
  }

  :global(.support-card:hover) svg {
    filter: drop-shadow(0 0 20px rgba(0, 255, 255, 0.5));
  }

  /* Keyframes */
  @keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
  }

  @keyframes pulse {
    0%, 100% { transform: scale(1); opacity: 0.8; }
    50% { transform: scale(1.2); opacity: 1; }
  }

  @keyframes spinIdle {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  @keyframes spinActive {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  @keyframes bugWiggleIdle {
    0%, 100% { transform: rotate(0deg); }
    50% { transform: rotate(5deg); }
  }

  @keyframes bugPanic {
    0% { transform: translate(0, 0) rotate(0deg); }
    25% { transform: translate(-2px, 2px) rotate(-5deg); }
    50% { transform: translate(2px, -2px) rotate(5deg); }
    75% { transform: translate(-2px, -2px) rotate(-5deg); }
    100% { transform: translate(2px, 2px) rotate(5deg); }
  }

  @keyframes glassSearch {
    0% { transform: translate(0, 0) scale(1); }
    25% { transform: translate(-10px, -10px) scale(1.05); }
    50% { transform: translate(15px, 5px) scale(1.02); }
    75% { transform: translate(-5px, 15px) scale(1.05); }
    100% { transform: translate(0, 0) scale(1); }
  }

  @keyframes alertBounce {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.2) rotate(10deg); }
  }
</style>
`;

fs.writeFileSync('/app/flux-player/src/lib/components/ui/BugIllustration.svelte', componentContent);
console.log('Updated BugIllustration.svelte');
