const fs = require('fs');

const path = 'flux-player/src/lib/components/ui/EmptyState.svelte';
let svelteCode = fs.readFileSync(path, 'utf8');

const regex = /(<svg\s+class="illus-svg player-core-svg"[\s\S]*?<\/svg>)/;

const newSvg = `      <svg
        class="illus-svg player-core-svg"
        viewBox="0 0 300 240"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true"
      >
        <!-- Layer 1 (Deepest Back) — Dot Matrix Pattern -->
        <g class="layer-back" style:transform="translate({parallaxX * 0.08}px, {parallaxY * 0.08}px)">
          <g class="dot-matrix">
            {#each Array(5) as _, row}
              {#each Array(7) as _, col}
                <circle
                  cx={30 + col * 12 + (row % 2 === 0 ? 0 : 6)}
                  cy={30 + row * 12}
                  r="2"
                  fill="none"
                  stroke="var(--secondary)"
                  stroke-width="1.5"
                  class="matrix-dot"
                  style:animation-delay="{row * 0.1 + col * 0.1}s"
                />
              {/each}
            {/each}
          </g>
        </g>

        <!-- Layer 2 (Mid) — Main Video Player Window -->
        <g class="layer-mid" style:transform="translate({parallaxX * 0.3}px, {parallaxY * 0.3}px)">
          <rect x="50" y="50" width="200" height="140" rx="6" fill="rgba(138, 43, 226, 0.03)" stroke="var(--primary)" stroke-width="2" class="draw-rect player-window" />

          <!-- Top Left Lines -->
          <line x1="65" y1="65" x2="105" y2="65" stroke="var(--primary)" stroke-width="2" stroke-linecap="round" class="draw-line draw-line-1" />
          <line x1="65" y1="72" x2="85" y2="72" stroke="var(--primary)" stroke-width="2" stroke-linecap="round" class="draw-line draw-line-2" />

          <!-- Top Right Dots -->
          <circle cx="235" cy="65" r="1.5" fill="var(--primary)" class="ui-dot delay-1" />
          <circle cx="235" cy="72" r="1.5" fill="var(--primary)" class="ui-dot delay-2" />
          <circle cx="235" cy="79" r="1.5" fill="var(--primary)" class="ui-dot delay-3" />

          <!-- Central Play Button -->
          <g class="play-button-group">
            <circle cx="150" cy="120" r="22" fill="rgba(0, 255, 255, 0.08)" stroke="var(--secondary)" stroke-width="2" class="draw-circle play-circle" />
            <path d="M145 110 L160 120 L145 130 Z" fill="var(--secondary)" class="play-triangle core-pulse" />
          </g>

          <!-- Bottom Progress Bar -->
          <line x1="90" y1="165" x2="235" y2="165" stroke="var(--primary)" stroke-width="3" stroke-linecap="round" class="draw-line prog-bar" opacity="0.4" />
          <line x1="90" y1="165" x2="185" y2="165" stroke="var(--primary)" stroke-width="3" stroke-linecap="round" class="draw-line prog-fill" />
          <circle cx="185" cy="165" r="4" fill="var(--glass-bg)" stroke="var(--primary)" stroke-width="2" class="prog-thumb fade-in-up" />

          <!-- Tooltip above progress thumb -->
          <g class="tooltip fade-in-up delay-1">
            <rect x="173" y="145" width="24" height="12" rx="3" fill="var(--primary)" />
            <path d="M185 160 L182 157 L188 157 Z" fill="var(--primary)" />
            <line x1="177" y1="150" x2="193" y2="150" stroke="#0a0a0c" stroke-width="1.5" stroke-linecap="round" />
            <line x1="177" y1="154" x2="185" y2="154" stroke="#0a0a0c" stroke-width="1.5" stroke-linecap="round" />
          </g>

          <!-- Bottom Right Controls (HD + Fullscreen) -->
          <rect x="215" y="175" width="14" height="8" rx="1.5" fill="var(--primary)" class="fade-in delay-2" />
          <text x="222" y="182" fill="#0a0a0c" font-family="var(--font-heading)" font-size="6" font-weight="bold" text-anchor="middle" class="fade-in delay-2">HD</text>

          <g class="fullscreen-icon fade-in delay-3" stroke="var(--primary)" stroke-width="1.5" stroke-linecap="round">
            <path d="M233 175 L231 175 L231 177" fill="none" />
            <path d="M237 175 L239 175 L239 177" fill="none" />
            <path d="M231 181 L231 183 L233 183" fill="none" />
            <path d="M239 181 L239 183 L237 183" fill="none" />
          </g>
        </g>

        <!-- Layer 3 (Front Right) — Filmstrip -->
        <g class="layer-front float-up-down" style:transform="translate({parallaxX * 0.5}px, {parallaxY * 0.5}px)">
          <g class="filmstrip-group slide-in-right">
            <rect x="235" y="100" width="40" height="38" rx="4" fill="var(--glass-bg)" stroke="var(--secondary)" stroke-width="1.5" />

            <!-- Left Perforations -->
            {#each Array(5) as _, i}
              <rect x="238" y={104 + i * 6} width="3" height="4" rx="0.5" fill="none" stroke="var(--secondary)" stroke-width="1" />
            {/each}

            <!-- Right Perforations -->
            {#each Array(5) as _, i}
              <rect x="269" y={104 + i * 6} width="3" height="4" rx="0.5" fill="none" stroke="var(--secondary)" stroke-width="1" />
            {/each}

            <!-- Image Frame -->
            <rect x="244" y="104" width="22" height="30" rx="2" fill="none" stroke="var(--secondary)" stroke-width="1.5" />

            <!-- Mountains and Sun -->
            <circle cx="253" cy="114" r="2.5" fill="none" stroke="var(--secondary)" stroke-width="1.5" />
            <path d="M245 125 L252 118 L257 122 L262 115 L265 119 L265 133 L245 133 Z" fill="rgba(0, 255, 255, 0.1)" stroke="var(--secondary)" stroke-width="1.5" stroke-linejoin="round" />
          </g>
        </g>

        <!-- Layer 4 (Front Left) — Folder -->
        <g class="layer-front float-up-down delay-anim" style:transform="translate({parallaxX * 0.6}px, {parallaxY * 0.6}px)">
          <g class="folder-group slide-in-left">
            <!-- Back flap -->
            <path d="M10 140 L40 140 C 42 140, 44 142, 45 144 L48 150 L90 150 C 93 150, 95 152, 95 155 L95 200 C 95 203, 93 205, 90 205 L10 205 C 7 205, 5 203, 5 200 L5 145 C 5 142, 7 140, 10 140 Z" fill="rgba(0, 255, 255, 0.05)" stroke="var(--secondary)" stroke-width="1.5" />

            <!-- Tab contents (simulated files) -->
            <rect x="50" y="145" width="30" height="15" rx="2" fill="var(--primary)" />
            <line x1="55" y1="150" x2="70" y2="150" stroke="#0a0a0c" stroke-width="1.5" stroke-linecap="round" />
            <line x1="55" y1="154" x2="65" y2="154" stroke="#0a0a0c" stroke-width="1.5" stroke-linecap="round" />

            <!-- Front flap -->
            <path d="M5 155 C 5 152, 7 150, 10 150 L90 150 C 93 150, 95 152, 95 155 L95 200 C 95 203, 93 205, 90 205 L10 205 C 7 205, 5 203, 5 200 Z" fill="var(--glass-bg)" stroke="var(--secondary)" stroke-width="1.5" />

            <!-- Bottom Left Lines on Folder -->
            <line x1="15" y1="192" x2="35" y2="192" stroke="var(--secondary)" stroke-width="1.5" stroke-linecap="round" />
            <line x1="15" y1="197" x2="25" y2="197" stroke="var(--secondary)" stroke-width="1.5" stroke-linecap="round" />
          </g>
        </g>
      </svg>`;

svelteCode = svelteCode.replace(regex, newSvg);

// Inject new CSS animations
const cssToAdd = `
  /* ==========================================
     Animation: Player UI Design
     ========================================== */
  .draw-rect {
    stroke-dasharray: 700;
    stroke-dashoffset: 700;
    animation: draw-shape 1.5s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }

  .draw-circle {
    stroke-dasharray: 150;
    stroke-dashoffset: 150;
    animation: draw-shape 1.2s cubic-bezier(0.4, 0, 0.2, 1) forwards 0.5s;
  }

  .draw-line {
    stroke-dasharray: 200;
    stroke-dashoffset: 200;
    animation: draw-shape 1s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }

  .draw-line-1 { animation-delay: 0.6s; }
  .draw-line-2 { animation-delay: 0.8s; }
  .prog-bar { animation-delay: 1.0s; }
  .prog-fill { animation-delay: 1.2s; }

  .matrix-dot {
    opacity: 0;
    transform: scale(0.5);
    animation: pop-in 0.4s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
  }

  .ui-dot {
    opacity: 0;
    animation: fade-in 0.5s ease forwards;
  }

  .play-triangle {
    opacity: 0;
    transform: scale(0.8);
    transform-origin: 150px 120px;
    animation: pop-in 0.5s cubic-bezier(0.34, 1.56, 0.64, 1) forwards 1s;
  }

  .slide-in-right {
    opacity: 0;
    transform: translateX(40px);
    animation: slide-in 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards 1.2s;
  }

  .slide-in-left {
    opacity: 0;
    transform: translateX(-40px);
    animation: slide-in 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards 1.4s;
  }

  .fade-in-up {
    opacity: 0;
    transform: translateY(10px);
    animation: fade-in-up-anim 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards 1.5s;
  }

  .fade-in {
    opacity: 0;
    animation: fade-in-anim 0.6s ease forwards 1.8s;
  }

  .float-up-down {
    animation: float 6s ease-in-out infinite alternate;
  }

  .float-up-down.delay-anim {
    animation-delay: -3s; /* Desync the two floating elements */
  }

  /* Shared Animation delays */
  .delay-1 { animation-delay: 1.7s; }
  .delay-2 { animation-delay: 1.9s; }
  .delay-3 { animation-delay: 2.1s; }

  @keyframes draw-shape {
    to { stroke-dashoffset: 0; }
  }

  @keyframes pop-in {
    to { opacity: 1; transform: scale(1); }
  }

  @keyframes slide-in {
    to { opacity: 1; transform: translateX(0); }
  }

  @keyframes fade-in-up-anim {
    to { opacity: 1; transform: translateY(0); }
  }

  @keyframes fade-in-anim {
    to { opacity: 1; }
  }

  @keyframes float {
    0% { transform: translateY(0px) rotate(0deg); }
    100% { transform: translateY(-8px) rotate(1deg); }
  }
`;

const styleRegex = /(<\/style>)/;

// also remove old player css classes
const oldCssRegex = /\/\* ==========================================\s*Animation: Player — Flux Core\s*========================================== \*\/(.|\n)*?(?=\/\* ==========================================\s*Accessibility)/gm;
svelteCode = svelteCode.replace(oldCssRegex, '');
svelteCode = svelteCode.replace(styleRegex, cssToAdd + '\n$1');

fs.writeFileSync(path, svelteCode);
