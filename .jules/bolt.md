## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-05-30 - [Optimize Bulk Database Transactions]
**Learning:** In rusqlite, preparing SQL statements inside a transaction loop incurs significant performance overhead due to redundant parsing and query optimization.
**Action:** Move statement preparation (`tx.prepare()`) outside of loops. When using transactions, explicitly drop the statement before calling `tx.commit()` to release the mutable borrow on the transaction.
