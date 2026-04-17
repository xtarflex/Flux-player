# Flux Skill Map: Agential Expertise Index

This document maps the 180+ localized skills in `c:\Users\sunny\.agents\skills` to specific domains within the Flux Player project. AI agents must use this map to identify and load the correct expertise for their task.

## 1. Core Architecture & Backend (Rust/Tauri)
*   **Skill:** `senior-architect`
    *   *Usage:* Major refactors, new module design, complex data flows.
*   **Skill:** `senior-backend`
    *   *Usage:* Rust logic, file system interactions, database optimizations.
*   **Skill:** `api-design-reviewer`
    *   *Usage:* Creating or auditing Tauri `commands` and internal APIs.
*   **Skill:** `database-schema-designer`
    *   *Usage:* SQLite migrations and model definitions.

## 2. Cinematic Frontend (Svelte/CSS)
*   **Skill:** `senior-frontend`
    *   *Usage:* Core Svelte logic, state management (stores), and component architecture.
*   **Skill:** `frontend-design`
    *   *Usage:* Drafting the "BOLD aesthetic" and high-fidelity UI layouts.
*   **Skill:** `ui-design-system`
    *   *Usage:* Maintaining the variable-driven CSS landscape and grid macro-architecture.
*   **Skill:** `algorithmic-art`
    *   *Usage:* Specialized canvas sampling or complex animation curves.

## 3. Performance & Quality (The "Forensic" Layer)
*   **Skill:** `performance-profiler`
    *   *Usage:* Investigating frame drops (below 60fps) or high memory usage.
*   **Skill:** `code-reviewer`
    *   *Usage:* Mandatory for the "Initial Agent Review" phase.
*   **Skill:** `playwright-pro`
    *   *Usage:* End-to-end testing for critical playback flows.
*   **Skill:** `tdd-guide`
    *   *Usage:* Implementing complex logic that requires "Definition of Done" unit tests.

## 4. Stability & DevOps (Beta Lifecycle)
*   **Skill:** `release-manager`
    *   *Usage:* Finalizing a Beta release, updating `CHANGELOG.md`, and version bumping.
*   **Skill:** `ci-cd-pipeline-builder`
    *   *Usage:* Modifying GitHub Actions or build scripts.
*   **Skill:** `knowledge-item-generator` (Implicit)
    *   *Usage:* Capturing complex bug resolutions in `DEV.log` as KIs.

## 5. Automation & Process
*   **Skill:** `agent-workflow-designer`
    *   *Usage:* Modifying or creating new `.agents/workflows/`.
*   **Skill:** `git`
    *   *Usage:* Complex branch management or history surgery.

> [!TIP]
> Always check for the `SKILL.md` in the folder before ingesting to ensure the persona matches the task context.
