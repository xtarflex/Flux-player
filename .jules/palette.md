
## 2026-05-21 - Focus Visibility for Custom Toggles
**Learning:** Found that the custom `Toggle.svelte` component hid its native `<input type="checkbox">`, making its standard browser focus outline invisible. Screen reader users and keyboard navigators lacked visual feedback when tabbing to the setting switches.
**Action:** When creating custom interactive components (like switches or sliders) that use visually hidden inputs, ensure keyboard navigation accessibility by applying `:focus-visible` styling (e.g., `outline: 2px solid var(--secondary);`) to their visible sibling elements.
