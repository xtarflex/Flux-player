## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-05-18 - [SQLite Batch Insert/Delete in Rust]
**Learning:** In Rust with `rusqlite`, preparing a statement using `tx.prepare()` borrows the transaction mutably. If prepared inside a loop, the database overhead is high. If prepared outside, the statement lives until the end of its scope, preventing `tx.commit()` from being called.
**Action:** Always wrap the `tx.prepare()` and its execution loop in an explicit block scope `{ ... }` so the prepared statement is dropped, releasing the mutable borrow, before calling `tx.commit()`.
