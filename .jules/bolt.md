## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-10 - [Bulk DB Queries]
**Learning:** `rusqlite::Connection::execute` and `rusqlite::Transaction::execute` implicitly prepare the SQL statement on every call. When executing the same statement in a loop (e.g., during bulk inserts or deletes), this causes redundant parsing and optimization overhead on every iteration.
**Action:** Move `tx.prepare("...")` outside the loop, use `stmt.execute(...)` inside the loop, and wrap the preparation in a block scope `{ ... }` so the statement is dropped, releasing the mutable borrow on the transaction before calling `tx.commit()`.
