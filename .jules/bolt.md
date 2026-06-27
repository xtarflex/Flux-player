## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2023-10-27 - [SQLite Bulk Optimization]
**Learning:** In Rust's `rusqlite` crate, calling `execute()` (or `tx.execute()`) implicitly prepares the SQL statement. When performing bulk database operations inside a loop, this causes redundant parsing overhead on every iteration.
**Action:** Move SQL statement preparation outside of the loop using `prepare()`, and call `stmt.execute()` inside. For transactions, ensure the prepared statement is explicitly dropped or block-scoped before committing to release the mutable borrow.
