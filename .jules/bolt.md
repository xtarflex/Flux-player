## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-23 - Optimize Bulk DB Operations
**Learning:** Reusing SQLite prepared statements via `tx.prepare()` outside loops drastically reduces re-parsing overhead for bulk database inserts/deletions (like in ), preventing implicit  on every iteration by `tx.execute()`. Using a scoped block ensures the statement is dropped before `tx.commit()`.
**Action:** Always extract statement preparation out of loops for SQLite operations on large sets.
## 2026-05-23 - Optimize Bulk DB Operations
**Learning:** Reusing SQLite prepared statements via `tx.prepare()` outside loops drastically reduces re-parsing overhead for bulk database inserts/deletions, preventing implicit `.prepare()` on every iteration by `tx.execute()`. Using a scoped block ensures the statement is dropped before `tx.commit()`.
**Action:** Always extract statement preparation out of loops for SQLite operations on large sets.
