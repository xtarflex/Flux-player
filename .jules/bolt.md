## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-10 - [Bulk DB Transactions Prepared Statement Overhead]
**Learning:** In the backend Rust code using rusqlite, when executing bulk inserts/deletes within loops during transactions, writing tx.execute(SQL) inside the loop will parse/compile the statement on every iteration, leading to significant scanning overhead. Moving tx.prepare outside the loop optimizes this, but requires calling explicitly drop(stmt) before committing the transaction to release the mutable borrow on the transaction.
**Action:** For bulk operations, always initialize statements with tx.prepare outside the loop, use stmt.execute within, and explicit drop(stmt) before tx.commit().
