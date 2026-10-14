## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-22 - [Optimize Bulk Insert/Delete Database Operations]
**Learning:** Moving `tx.prepare` outside of loops in Rust `rusqlite` transactions significantly avoids redundant SQL parsing and optimization overhead, improving database operation performance.
**Action:** Use prepared statements outside loops with block scoping to release mutable borrows before calling `tx.commit()` in future database updates.
