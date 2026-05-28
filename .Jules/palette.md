## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-11 - Focus Visibility for Slider Elements
**Learning:** Found custom slider UI pattern (Volume Bars) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values, relying instead on global window shortcuts.
**Action:** When inspecting custom interactive elements, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the slider's expected value changes.

## 2024-05-18 - Icon-Only Button ARIA Labels
**Learning:** Icon-only buttons using nested SVGs lack accessible names by default, making them unreadable to screen readers. For dynamic buttons (like play/pause), the accessible name must accurately reflect the *current* state or action available.
**Action:** Always verify that buttons containing only icons (like in `MiniPlayerOverlay.svelte`) have an `aria-label` attribute, and use reactive expressions (e.g., `aria-label={isPlaying ? 'Pause' : 'Play'}`) for stateful actions.
