## 2024-05-27 - Add focus states to library detail panel buttons
**Learning:** Found that some buttons in `LibraryDetailPanel.svelte` (`.close-btn`, `.btn-play`, `.btn-secondary`) were lacking `:focus-visible` styles, which hindered keyboard accessibility. The standard pattern in the app is `outline: none; box-shadow: 0 0 0 2px var(--secondary);`.
**Action:** Always ensure custom interactive elements or specific buttons (like those inside detail panels) explicitly define a `:focus-visible` state using the app's standard `var(--secondary)` focus ring.
