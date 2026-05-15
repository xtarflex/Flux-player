## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-15 - Optimize Bulk Database Operations
**Learning:** In the Rust backend utilizing `rusqlite`, preparing SQL statements inside loops during bulk operations causes redundant parsing and optimization overhead.
**Action:** Always prepare statements (`conn.prepare` or `tx.prepare`) outside of loops for batch insertions or deletions. When using a transaction, ensure the statement is explicitly dropped (e.g., `drop(stmt);` or via block scoping) before calling `tx.commit()` to release the mutable borrow.
