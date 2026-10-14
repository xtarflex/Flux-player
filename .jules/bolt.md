## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-11 - [SQLite Transaction Prepare Loop Optimization]
**Learning:** For performance-critical bulk database operations in the Rust backend, moving SQL statement preparation (via `conn.prepare` or `tx.prepare`) outside of loops avoids redundant parsing and optimization overhead. When using transactions (`tx.prepare`), a manual `drop(stmt)` is strictly required before committing (`tx.commit()`) to release the mutable borrow and satisfy Rust's borrow checker.
**Action:** Always pre-compile loop statements. Keep `drop(stmt)` in mind when working with SQLite transactions via `rusqlite` to avoid `borrow mut` compiler errors before commits.
