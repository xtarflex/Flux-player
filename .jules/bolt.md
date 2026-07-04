## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-07-04 - [Optimize Bulk Database Operations]
**Learning:** In Rust's `rusqlite` crate, calling `tx.execute()` implicitly prepares the SQL statement. When performing bulk database operations inside a loop, this causes redundant parsing overhead on every iteration.
**Action:** Move `tx.prepare()` outside the loop and call `stmt.execute()` inside. Remember to explicitly drop the statement (e.g., via block scoping) before committing the transaction to release the mutable borrow on `tx`.
