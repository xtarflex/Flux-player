## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-05-27 - [Optimize SQLite Batch Queries in Rust]
**Learning:** When performing batch `INSERT` or `DELETE` operations using `rusqlite` in loops, calling `tx.execute(...)` inside the loop redundantly parses and optimizes the SQL statement on every iteration. Moving `tx.prepare(...)` outside the loop and using `stmt.execute(...)` inside provides a substantial performance boost. However, due to Rusts borrow checker, the prepared statement holds a mutable borrow on the transaction (`tx`). This prevents `tx.commit()` from being called until the statement is dropped.
**Action:** Always hoist statement preparation out of loops for batch DB operations. Wrap the statement creation and execution loop in a `{ ... }` block to explicitly drop the prepared statement and release the mutable borrow before calling `tx.commit()`.
