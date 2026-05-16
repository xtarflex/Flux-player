## 2026-05-07 - [Debounce Search]
**Learning:** blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-15 - [Database bulk operation optimization]
**Learning:** For performance-critical bulk database operations in the Rust backend, moving SQL statement preparation (via `conn.prepare` or `tx.prepare`) outside of loops avoids redundant parsing and optimization overhead. When using transactions (`tx.prepare`), the statement must be explicitly dropped (e.g., `drop(stmt);`) before committing to release the mutable borrow on the transaction.
**Action:** Always extract statement preparation from loops during bulk database inserts, updates, or deletes.
