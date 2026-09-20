## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-03-24 - [Fix IPC Process Leak in PlayerEngine]
**Learning:** Checking Tauri window state (e.g. `getCurrentWindow().isMinimized()`) from within high-frequency event handlers like Video.js `timeupdate` floods the IPC channel, leading to severe process blocking and UI rendering freezing.
**Action:** Always move IPC calls outside of high-frequency DOM/media event handlers, preferring debounced/throttled executions or cached states updated by low-frequency events.
