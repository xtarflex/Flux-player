## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-05-18 - Optimize bulk database operations
**Learning:** In Rust/SQLite with Tauri (`rusqlite`), executing statements directly in loops using `tx.execute(...)` inside a transaction forces query parsing and optimization on every iteration, creating a measurable performance bottleneck for large media libraries.
**Action:** Always move SQL parsing outside of loops. Use `let mut stmt = tx.prepare(...)` before the loop, iterate over data by calling `stmt.execute(params)`, and ensure the prepared statement is explicitly dropped (e.g., using `{ }` block scoping) before calling `tx.commit()` to satisfy borrow checker requirements.
