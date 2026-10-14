## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-05-08 - [SQLite Bulk Operation Optimization]
**Learning:** Preparing SQL statements *inside* loops (e.g., `conn.prepare(...).execute(...)` or `tx.prepare(...)`) during bulk operations like media syncing or cleanup creates significant overhead in `rusqlite` by repeatedly parsing and optimizing the query.
**Action:** Always move `conn.prepare` or `tx.prepare` outside of loops for bulk database operations in the Rust backend to improve execution speed for large datasets. Drop the statement explicitly using `drop(stmt)` if a mutable borrow of the transaction (`tx`) is required immediately afterwards (like `tx.commit()`).
