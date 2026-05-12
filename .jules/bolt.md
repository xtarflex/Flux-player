## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-05-12 - [Optimize rusqlite bulk inserts with prepare]
**Learning:** In rusqlite, using `tx.execute` or `conn.execute` inside a loop incurs significant overhead because the SQL statement is repeatedly parsed and compiled by SQLite for every iteration.
**Action:** Always move the statement preparation (`tx.prepare`) outside of loops for bulk database operations and call `stmt.execute` inside the loop. Remember to explicitly `drop(stmt)` before calling `tx.commit()` to release the mutable borrow on the transaction.
