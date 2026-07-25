## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-11 - Focus Visibility for Slider Elements
**Learning:** Found custom slider UI pattern (Volume Bars) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values, relying instead on global window shortcuts.
**Action:** When inspecting custom interactive elements, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the slider's expected value changes.

## 2026-05-18 - Focus Visibility and Keyboard Support for Scrubber Elements
**Learning:** Found custom slider UI pattern (Media Progress Scrubber) without visible focus outlines `focus-visible` despite having `role="slider"` and being keyboard focusable. Also missing internal keyboard handlers for incrementing/decrementing values. This makes it impossible for keyboard users to interact with or see focus on the scrubber.
**Action:** When inspecting custom interactive elements like scrubbers, always verify that a visually distinct `:focus-visible` state exists and that localized `onkeydown` events map directional arrows to the scrubber's expected progress changes. Additionally, ensure `aria-label` is present to provide context.
