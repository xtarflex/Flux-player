## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-07-15 - [Hoist SQL Statement Preparation in Loops]
**Learning:** In Rust's `rusqlite` crate, calling `execute()` inside a loop (e.g., `tx.execute(...)`) implicitly parses and prepares the SQL statement on every iteration, leading to significant overhead for bulk database operations.
**Action:** Always hoist statement preparation (using `tx.prepare(...)`) outside of loops and call `stmt.execute()` inside. Scope the prepared statement within a block to ensure it is dropped before attempting to commit the transaction.
