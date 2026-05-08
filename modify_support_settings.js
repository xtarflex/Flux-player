const fs = require('fs');

const path = '/app/flux-player/src/lib/components/settings/SupportSettings.svelte';
let content = fs.readFileSync(path, 'utf8');

// Import BugIllustration
if (!content.includes('import BugIllustration')) {
  content = content.replace(
    /import Icon from "\.\.\/ui\/Icon\.svelte";/,
    'import Icon from "../ui/Icon.svelte";\n  import BugIllustration from "../ui/BugIllustration.svelte";'
  );
}

// Add BugIllustration component inside .support-card.featured
// First check where the 'featured-actions' div is, we'll place it after the text stack but let's make a wrapper for it.

const replacementTarget = `<div class="featured-actions">
            <button class="btn-primary report-btn" onclick={reportBug}>
              <Icon name="github" size={18} />
              Report Issue on GitHub
            </button>
          </div>
        </section>`;

const newContent = `<div class="featured-actions">
            <button class="btn-primary report-btn" onclick={reportBug}>
              <Icon name="github" size={18} />
              Report Issue on GitHub
            </button>
          </div>
          <div class="bug-illustration-wrapper">
             <div class="bug-illustration-inner">
               <BugIllustration />
             </div>
          </div>
        </section>`;

content = content.replace(replacementTarget, newContent);

fs.writeFileSync(path, content);
console.log('Modified SupportSettings.svelte layout.');
