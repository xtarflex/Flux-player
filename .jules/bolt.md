## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-17 - [SQLite Bulk Operation Optimization]
**Learning:** In the Rust backend, SQLite statement preparation (via `conn.prepare` or `tx.prepare`) inside loops incurs redundant parsing and optimization overhead.
**Action:** Move SQL statement preparation outside of loops to avoid this overhead, especially for performance-critical bulk operations. When using transactions (`tx.prepare`), explicitly drop the statement (e.g., via block scoping) before committing to release the mutable borrow.
