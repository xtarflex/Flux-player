## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.

## 2023-10-27 - [Optimize Database Bulk Operations]
**Learning:** In the Rust backend using rusqlite, calling `tx.execute` inside a loop requires parsing, compiling, and optimizing the SQL query for every single iteration, which becomes a severe bottleneck for large bulk operations (e.g., scanning thousands of media items).
**Action:** When performing loop-based database mutations, prepare the statement outside the loop using `tx.prepare("...")`, and run `stmt.execute(...)` inside the loop. Remember to encapsulate the statement within its own block `{ let mut stmt = tx.prepare(...)?; ... }` so it is safely dropped before `tx.commit()`, which avoids borrowing issues.
