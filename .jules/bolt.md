## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2023-10-27 - [SQLite Transaction Prepare Efficiency]
**Learning:** In the Rust backend, loops that do repetitive database inserts/deletes within a transaction (e.g. `save_media_items` or `clean_stale_media`) were re-preparing SQLite queries inside the loop, causing redundant parsing overhead. Furthermore, extracting the prepared statement outside the loop while inside a transaction (`let mut stmt = tx.prepare(...)`) borrows the transaction. If we attempt to commit (`tx.commit()`) while the statement still borrows it, rustc complains.
**Action:** Always extract SQLite statement preparations (`conn.prepare` or `tx.prepare`) out of loops for bulk operations. When using transactions, wrap the statement preparation and the loop inside a block `{ ... }` so the prepared statement gets naturally dropped and releases its mutable borrow on `tx` before `tx.commit()` is called.
