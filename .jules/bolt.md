## 2026-05-07 - [Debounce Search]
**Learning:**  blocks the main thread during typing when reacting to a fast-updating state like an input bind if the derived block does a lot of work (like filtering an array of strings in a large library).
**Action:** Introduce debouncing using an $effect and setTimeout so the main filtering calculation waits until the user finishes typing.
## 2024-05-24 - Avoid expect with format string
**Learning:** In Rust, `expect(&format!(...))` is flagged by clippy because it eagerly evaluates the format string even on success.
**Action:** Use `.unwrap_or_else(|_| panic!(...))` to lazily evaluate the panic message only on failure.
