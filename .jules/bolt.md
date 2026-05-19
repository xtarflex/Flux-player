## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-05-19 - [Optimize SQLite batch operation preparation]
**Learning:** When inserting or deleting items in bulk using `rusqlite` with `tx.execute(...)` in a loop, it inherently suffers performance penalties by preparing and optimizing the statement per iteration. We can avoid this and achieve significant speedups by using `tx.prepare("...")` outside the loop, keeping mutable borrowing block-scoped to allow dropping before `tx.commit()`.
**Action:** Always move `tx.prepare()` out of loops for large metadata queries and enclose them in block scopes `{ ... }` to auto-drop the statement before transaction commit.
