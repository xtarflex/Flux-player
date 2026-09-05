## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-11 - Focus Visibility for Slider Elements
**Learning:** Found custom slider UI pattern (Volume Bars) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values, relying instead on global window shortcuts.
**Action:** When inspecting custom interactive elements, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the slider's expected value changes.
## 2026-05-18 - ARIA Labels for Mini Player Buttons
**Learning:** Found icon-only buttons in `MiniPlayerOverlay.svelte` (`Return to Player`, `Close Player`, and `Play/Pause`) relying on visual icons or simple `title` attributes instead of proper screen-reader friendly `aria-label`s.
**Action:** Always ensure custom overlay components, especially compact player controls, map interactive states (like `isPlaying ? 'Pause' : 'Play'`) explicitly to `aria-label` attributes to maintain accessibility parity with full-sized controls.
