## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-18 - [Optimize SQLite Bulk Operations]
**Learning:** Calling `tx.execute(...)` inside a loop for database operations like inserts or deletes prepares the SQL statement on every iteration. This leads to redundant parsing and optimization overhead, acting as a bottleneck during bulk library scans.
**Action:** When performing bulk SQLite operations inside transactions, use `tx.prepare(...)` to prepare the statement once outside the loop. Use block scoping to ensure the `stmt` is dropped before calling `tx.commit()`, which satisfies Rust's borrowing rules without explicit `drop(stmt)` calls.
