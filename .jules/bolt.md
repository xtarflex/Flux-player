## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-24 - [Avoid statement preparation in loops]
**Learning:** In performance-critical bulk database operations using `rusqlite`, preparing SQL statements (via `conn.prepare` or `tx.prepare`) inside loops causes redundant parsing and query optimization overhead. Additionally, when using a transaction (`tx.prepare`), the prepared statement holds a mutable borrow on the transaction, so it must be explicitly dropped (e.g., using a block scope or `drop(stmt)`) before the transaction can be committed.
**Action:** Always extract statement preparation outside of loops for bulk database operations, use `stmt.execute(...)` inside the loop, and wrap the statement in its own scope or drop it explicitly before calling `tx.commit()`.
