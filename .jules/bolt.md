## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2024-05-18 - SQLite Performance Bottleneck in Loops
**Learning:** In the Rust backend, bulk database operations (like `clean_stale_media` or `save_media_items`) executed within loops using `conn.execute(...)` or `tx.execute(...)` cause significant overhead because the SQL statements are parsed, compiled, and optimized redundantly on every iteration.
**Action:** Always extract SQLite statement preparation out of execution loops using `tx.prepare(...)`. Keep in mind that rusqlite transactions must outlive the prepared statements; therefore, use block scoping or `drop(stmt)` to explicitly release the mutable borrow of the transaction before calling `tx.commit()`.
