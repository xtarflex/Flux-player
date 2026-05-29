
## 2024-05-29 - Missing Focus Visible on Custom Inputs
**Learning:** Custom interactive components (like switches/toggles) in this app that hide their native `<input>` element with `opacity: 0` lose default browser keyboard focus indicators, making keyboard navigation inaccessible since the input can't be seen when it receives tab focus.
**Action:** When styling custom inputs, always remember to add an `input:focus-visible + .sibling` rule to explicitly show focus on the visible mock element (e.g., `outline: 2px solid var(--secondary); outline-offset: 2px;`).
