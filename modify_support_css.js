const fs = require('fs');

const path = '/app/flux-player/src/lib/components/settings/SupportSettings.svelte';
let content = fs.readFileSync(path, 'utf8');

// We need to alter .support-card.featured to not cut off everything horizontally.
// Usually .settings-card has overflow: hidden. .support-card.featured might inherit it.
// To achieve protrusion on the right but clip on the bottom, we need:
// .support-card.featured { overflow: visible; }
// And the inner styling to handle the background/border that IS clipped, OR
// use clip-path to clip only top, left, and bottom, but not right.

// Let's modify the CSS

const styleAdditions = `
  /* Bug Illustration Styling */
  .bug-illustration-wrapper {
    position: absolute;
    bottom: 0;
    right: -40px; /* Protrude to the right */
    width: 240px;
    height: 240px;
    z-index: 1;
    pointer-events: none;
    /* Clip path to cut off the bottom strictly at the card boundary, but allow the right to extend */
    /* top, right, bottom, left */
    clip-path: polygon(-50% -50%, 200% -50%, 200% 100%, -50% 100%);
  }

  .bug-illustration-inner {
    width: 100%;
    height: 100%;
    position: relative;
    /* We push the SVG slightly down so it gets visually cut off by the bottom clip */
    bottom: -20px;
    transition: transform 0.3s ease;
  }

  @media (max-width: 900px) {
    .bug-illustration-wrapper {
      display: none;
    }
  }
`;

// Also check the existing .support-card.featured to ensure it doesn't have overflow: hidden
content = content.replace(
  /(\.support-card\.featured \{[\s\S]*?)overflow:\s*hidden;/g,
  '$1/* overflow: hidden removed to allow right protrusion */'
);

// We need to make sure the background and border of .support-card.featured still looks right without overflow hidden.
// Since border-radius won't clip children anymore, we need a separate background layer, but it already has .card-background-glow! Let's just use clip-path on the card or background. Wait, if we use overflow: visible on the card, the gradient background and border will still look fine.

if (!content.includes('.bug-illustration-wrapper')) {
    // Inject styles before the final closing </style> tag
    content = content.replace('</style>', styleAdditions + '\n</style>');
}

fs.writeFileSync(path, content);
console.log('Modified CSS.');
