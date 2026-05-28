## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2026-05-28 - [SQL Statement Preparation Scope]
**Learning:** When using transactions (e.g. `tx.prepare`) for bulk database operations in Rust/rusqlite, the prepared statement mutable borrows the transaction. This means the statement must be explicitly dropped before `tx.commit()` can be called, which can be cleanly achieved using an inner block scope `{ ... }`.
**Action:** Use inner scopes when hoisting `prepare` statements out of loops in transaction contexts.
