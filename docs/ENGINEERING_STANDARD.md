# Flux Player: Engineering Standard (FES)

This document defines the formal engineering standards for Flux Player. Adherence to these standards is mandatory for all contributors (Human and AI) to ensure Beta-level stability and a premium user experience.

## 1. The Core Philosophy
*   **Performance is UX:** Every frame counts. Aim for a locked 60fps.
*   **The Forensic Blueprint:** Leverage "sleight of hand" and technical magic before brute-force solutions.
*   **Agential Transparency:** Write code and documentation so that the next agent doesn't need to guess.

## 2. Git & Branching Policy
*   **Mandatory Feature Branches:** Direct commits to `main` are strictly prohibited.
*   **Naming Convention:**
    *   `feat/[feature-name]` for new functionality.
    *   `fix/[bug-description]` for bug fixes.
    *   `perf/[target-area]` for performance optimizations.
*   **Verification:** A branch must pass all automated tests and receive human "Aesthetic Approval" before merging.

## 3. Definition of Ready & Done

### Definition of Ready (DoR)
Before starting a task:
1.  **Context Load:** Ingest `ARCHITECTURE.md` and `forensic_blueprint.md`.
2.  **Skill Discovery:** Search `c:\Users\sunny\.agents\skills` for relevant expertise (e.g., `api-design-reviewer`, `senior-frontend`). Load and state which skills are active.
3.  **Plan Approval:** A technical plan must be drafted and approved by the human developer.
4.  **Branch Isolated:** A dedicated branch must be checked out.

### Definition of Done (DoD)
A task is complete only when:
1.  **Code Quality:**
    *   JS follows ES6 modules and uses JSDoc.
    *   CSS is modular and respects the global variable system.
    *   Rust backend is properly guarded against panics.
2.  **Verification:**
    *   `npm run check` passes.
    *   Manual verification of animations and responsive layout.
3.  **Documentation:**
    *   `DEV.log` is updated with **Action** and **Rationale**.
    *   Knowledge Items (KIs) are extracted for complex resolutions.
4.  **Polish:** Pixel-perfect alignment and 1px border crispness verified.

## 4. Coding Standards

### JavaScript / TypeScript
*   **Modularity:** Use ES6 modules. Export functions individually, not as large objects.
*   **JSDoc:** Every non-trivial function must have a JSDoc block.
    ```javascript
    /**
     * Extracts dominant color from image data with canvas skip-sampling.
     * @param {ImageData} data - The raw pixel data.
     * @returns {string} The computed rgb() string.
     */
    ```
*   **Reactivity:** Leverage Svelte 5 snippets and stores for cross-component state.

### CSS (Vanilla)
*   **Component-Based:** One CSS file per component where possible, or high-level partials.
*   **Variables First:** Use `--color-primary`, `--island-bg`, etc. instead of hardcoded hex codes.
*   **Macro-Architecture:** Use `display: grid` for layout. Avoid `position: absolute` unless creating an overlay.

### Rust (Tauri)
*   **Safety:** Use `Result<T, E>` and `Option<T>` instead of `unwrap()`.
*   **Async Patterns:** Long-running tasks (scanning, scraping) must be run in async commands or background threads.

## 5. Premium UX Audit (The "Sleight of Hand" Check)
Every UI feature must be audited against the **Forensic Blueprint**:
*   **Canvas vs. DOM:** Use high-speed pixel sampling (64x64 canvas) for adaptive effects instead of heavy libraries.
*   **Springs vs. Keyframes:** Use Svelte spring physics (`stiffness: 0.15`, `damping: 0.35`) for kinetic elements like the Dynamic Island.
*   **1px Precision:** Ensure all borders are exactly 1px with no outer glows unless specified.
*   **Search Latency:** Maintain 0ms perceived latency via frontend filtering tricks.
*   **Budgeting:** Prohibit new dependencies if a high-performance native solution exists in the Blueprint.

## 6. Release Management
*   **Semantic Versioning:** Follow `MAJOR.MINOR.PATCH-beta.x`.
*   **Changelog:** Update `CHANGELOG.md` for every merged PR.
