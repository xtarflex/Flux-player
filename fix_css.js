const fs = require('fs');
const path = 'flux-player/src/lib/components/ui/EmptyState.svelte';
let svelteCode = fs.readFileSync(path, 'utf8');

svelteCode = svelteCode.replace(/    }\n  }\n  }/g, '    }\n  }');

fs.writeFileSync(path, svelteCode);
