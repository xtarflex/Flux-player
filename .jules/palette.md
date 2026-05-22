## 2026-05-22 - Keyboard Navigation for Hidden Inputs
**Learning:** When using visually hidden inputs wrapped in custom interactive elements (like sliders or toggles), the `:focus-visible` state must be explicitly handled on a visible sibling element to ensure keyboard accessibility.
**Action:** Use `input:focus-visible + .sibling-class` or similar CSS selectors to apply a focus ring (e.g., `outline: 2px solid var(--secondary)`) to the visible part of the component.
