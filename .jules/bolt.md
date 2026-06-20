## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-06-20 - Prepare Statements Outside Loops in rusqlite
**Learning:** In Rust's `rusqlite` crate, calling `tx.execute(...)` implicitly parses and prepares the SQL statement every single time it's called. When performing bulk database operations inside a loop (like deleting stale paths or inserting batch media items), this causes significant redundant parsing overhead.
**Action:** Always move SQL statement preparation outside of the loop using `tx.prepare()` (or `conn.prepare()`) when dealing with performance-critical bulk database operations. Make sure to bind variables to the prepared statement using `stmt.execute(rusqlite::params![...])` inside the loop, and drop the statement (via explicit block scoping or `drop(stmt)`) before committing the transaction to release the mutable borrow.
