## 2024-11-20 - Add Tooltips to Icon-Only Buttons
**Learning:** Icon-only buttons often lack visual context for screen readers or new users, reducing accessibility and discoverability. Adding a tooltip provides on-hover context and, ideally, includes the shortcut for faster interactions.
**Action:** Use `$lib/actions/tooltip` to standardise context hints over raw `title` attributes.
## 2026-09-19 - Custom Inputs Keyboard Accessibility
**Learning:** Using `display: none` on inputs inside custom radio cards or toggle switches completely removes them from the keyboard navigation flow, making them inaccessible.
**Action:** Use `position: absolute; opacity: 0; width: 0; height: 0;` instead of `display: none` for hidden inputs, and apply focus ring styling to their parent wrapper using `:has(input:focus-visible)` or adjacent siblings using `input:focus-visible + .slider`.
