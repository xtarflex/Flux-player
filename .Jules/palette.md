## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-11 - Focus Visibility for Slider Elements
**Learning:** Found custom slider UI pattern (Volume Bars) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values, relying instead on global window shortcuts.
**Action:** When inspecting custom interactive elements, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the slider's expected value changes.

## 2026-05-13 - Toggle component Focus Visibility
**Learning:** Found custom switch component (`Toggle.svelte`) using a visually hidden `<input type="checkbox">` without visible focus outlines `focus-visible` on the surrounding slider visual element. Keyboard users navigating via tab could not see the focused element.
**Action:** When implementing custom interactive components using visually hidden inputs (like switches or custom checkboxes), always ensure `:focus-visible` styling (typically `outline: 2px solid var(--secondary); outline-offset: 2px;`) is applied to their visible sibling elements (e.g., `input:focus-visible + .slider`) to guarantee keyboard accessibility.
