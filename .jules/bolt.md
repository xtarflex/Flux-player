## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-05-18 - Optimize bulk SQLite queries in Rust

**Learning:** When using `rusqlite` for bulk database operations in Rust, `tx.execute(...)` inside a loop incurs significant overhead due to redundant parsing and query optimization on every iteration. Additionally, statement objects hold a mutable borrow on the transaction/connection; therefore, they must be explicitly dropped before committing (`drop(stmt)` before `tx.commit()`) to avoid mutable borrow compilation errors.
**Action:** Extract query preparation (`tx.prepare`) outside loops for bulk inserts or deletes, and explicitly drop the prepared statement before calling `tx.commit()`.
