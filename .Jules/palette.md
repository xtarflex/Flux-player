## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-11 - Focus Visibility for Slider Elements
**Learning:** Found custom slider UI pattern (Volume Bars) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values, relying instead on global window shortcuts.
**Action:** When inspecting custom interactive elements, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the slider's expected value changes.

## 2026-06-27 - Focus Visibility for Hidden Inputs in Custom Toggles
**Learning:** Found custom toggle components (`Toggle.svelte`) where the native `<input type="checkbox">` is visually hidden (opacity 0) and replaced by a styled `.slider` sibling. Because the native input is invisible, keyboard navigation tab focus is completely lost to the user, making it impossible to tell which toggle is active.
**Action:** Always ensure that custom interactive components using visually hidden native inputs apply a `:focus-visible` style to their visible sibling element (e.g., `input:focus-visible + .slider`) to maintain keyboard navigation accessibility.
