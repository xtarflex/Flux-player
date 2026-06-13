## 2024-06-13 - Focus Visible for Hidden Inputs
**Learning:** When hiding native inputs visually (e.g., in a custom toggle using `opacity: 0`), native focus rings are lost. It is critical to use adjacent sibling selectors (like `input:focus-visible + .slider`) to re-apply a focus indicator (`outline`) to the visual element representing the input.
**Action:** Always check custom form elements (switches, custom checkboxes/radios) to ensure they have an explicit `:focus-visible` outline applied to their visible proxy element.
