## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.
