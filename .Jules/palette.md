## 2026-05-06 - ARIA Label Additions
**Learning:** Found several Svelte components using interactive elements like `<input type="checkbox">` toggles, search inputs, and inline editing fields without `aria-label`s. Visual layout tricks (like empty wrappers) made them look accessible but hid them from screen readers.
**Action:** Always ensure inputs not directly associated with descriptive `<label>` text use an explicit `aria-label` attribute, especially in custom UI patterns like toggles or inline editing panels.

## 2026-05-08 - Dropdown ARIA Attributes and SSR Hydration
**Learning:** Adding ARIA properties to custom UI elements like `Dropdown.svelte` requires care with ID generation for `aria-controls`. Using `Math.random()` during component initialization leads to hydration mismatches in SvelteKit SSR environments. Additionally, Svelte 5 dynamically converts boolean props like `aria-expanded={isOpen}` to string values `true` or `false` in the DOM automatically without needing ternary workarounds.
**Action:** When generating unique IDs for `aria-controls` or similar associations, use an SSR-safe pattern like Svelte's `useId` (if available) or a module-level counter, rather than `Math.random()`.
