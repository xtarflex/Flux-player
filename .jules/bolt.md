## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2023-10-27 - [Optimize Bulk Database Operations]
**Learning:** In SQLite/Tauri projects, executing SQL statements iteratively (`tx.execute(...)` inside a loop) causes a new compilation/parsing of the statement on every iteration. This acts as a significant hidden bottleneck during bulk operations like `save_media_items` or `clean_stale_media`.
**Action:** Always pre-compile statements outside of loops using `tx.prepare("...")` and then use `stmt.execute(...)` inside the loop. Ensure the statement scope (`{}`) correctly releases it before calling `tx.commit()`.
