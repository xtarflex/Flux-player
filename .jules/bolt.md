## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2026-05-16 - Pre-compiling SQLite queries in loops with transactions
**Learning:** When performing bulk SQLite operations in Rust using `rusqlite`, preparing statements (`tx.prepare()` or `conn.prepare()`) inside loops causes redundant parsing, optimizing, and compiling overhead for every iteration. However, when moving the preparation outside the loop while using a transaction (`tx`), a mutable borrow conflict occurs when trying to commit (`tx.commit()`).
**Action:** Pre-compile SQLite statements outside the execution loops. To fix the borrow checker error when using transactions, wrap the `tx.prepare()` and loop in a block scope (`{ ... }`) to ensure the statement (`stmt`) is dropped and releases its mutable borrow on `tx` before calling `tx.commit()`.
