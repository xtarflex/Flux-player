## 2026-05-21 - Prepared Statements in SQLite Transactions
**Learning:** In the Rust backend using `rusqlite`, preparing SQL statements inside loops (via `conn.execute` or `tx.execute`) during bulk database operations (like batch inserts or deletes) causes significant, redundant parsing and optimization overhead.
**Action:** Always move SQL statement preparation (`conn.prepare` or `tx.prepare`) outside of loops for bulk operations. When using transactions, explicitly drop the statement (e.g., using a block scope `{{ }}`) before calling `tx.commit()` to release the mutable borrow.
