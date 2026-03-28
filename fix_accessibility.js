const fs = require('fs');

const path = 'flux-player/src/lib/components/ui/EmptyState.svelte';
let svelteCode = fs.readFileSync(path, 'utf8');

const regex = /@media\s+\(prefers-reduced-motion:\s+reduce\)\s+\{([\s\S]*?)\}/;

const accessibilityBlock = `@media (prefers-reduced-motion: reduce) {
    .conn-line, .satellite-node, .satellite-play,
    .core-spin, .core-pulse, .radar-sweep,
    .radar-dot, .blip, .slate-base, .slate-tab,
    .data-node, .data-line, .aurora, .empty-state,
    .draw-rect, .draw-circle, .draw-line, .matrix-dot,
    .ui-dot, .play-triangle, .slide-in-right,
    .slide-in-left, .fade-in-up, .fade-in, .float-up-down {
      animation: none !important;
      transition: none !important;
      stroke-dashoffset: 0 !important;
      opacity: 1 !important;
      transform: none !important;
    }
  }`;

svelteCode = svelteCode.replace(regex, accessibilityBlock);
fs.writeFileSync(path, svelteCode);
