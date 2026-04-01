# Flux Player: Comprehensive Audit Report

**Date:** 2026-04-01
**Focus Areas:** Code Quality & Security, Architecture & Data, Compliance & Documentation, Infrastructure-as-Code (IaC)

---

## 1. Code Quality & Security

### Hardcoded Secrets
- **Finding:** A hardcoded API Key fallback error message `API_KEY_NOT_FOUND` exists, but `settings::get_tmdb_key` is used to fetch the TMDB API key dynamically from settings rather than hardcoding it in the source. This is good practice.
- **Location:** `src-tauri/src/scanner/tmdb.rs` and `src-tauri/src/commands/settings.rs`
- **Recommendation:** Ensure the initial setup flow securely prompts the user for the TMDB API key and stores it securely, possibly encrypting the SQLite `settings` table if sensitive keys are added later.

### Dependency Vulnerabilities
- **Frontend (`npm audit`):**
  - **High Severity:** `@xmldom/xmldom` (XML injection) and `picomatch` (ReDoS & Method Injection).
  - **Low Severity:** `@sveltejs/adapter-static`, `@sveltejs/kit`, `cookie`.
  - **Recommendation:** Run `npm audit fix` and update dependencies to patched versions.
- **Backend (`cargo audit`):**
  - **Unsoundness:** `glib` crate (v0.18.5) has a known unsoundness issue in `VariantStrIter`.
  - **Unmaintained:** Several `unic-*` crates and `proc-macro-error` are flagged as unmaintained.
  - **Recommendation:** Update `glib` to >= 0.20.0 to fix the memory unsoundness. Migrate away from the unmaintained `unic-*` crates toward recommended alternatives like `icu_properties`.

### Input Validation & SQL Injection
- **Finding:** All observed SQLite database queries in `src-tauri/src/database/queries.rs` and `commands/library.rs` correctly use parameterized queries (e.g., `rusqlite::params![...]`).
- **Conclusion:** Safe from standard SQL Injection.

### Error Handling
- **Finding:** Rust backend frequently uses `.map_err(|e| e.to_string())?` to pass errors to the frontend.
- **Risk:** This might leak internal database paths or Rust panic traces to the UI.
- **Recommendation:** Implement a custom `AppError` enum that implements `serde::Serialize` to return sanitized, user-friendly error codes to the Svelte frontend, logging the actual `e.to_string()` exclusively in the backend.

---

## 2. Architecture & Data

### Database Schema
- **Finding:** The SQLite schema (`schema.rs`) is well-structured with appropriate normalization.
- **Data Privacy:** Watch history (`is_watched`, `last_position`) is stored in plain text locally. As a desktop app, this is standard, but if telemetry is ever added, this PII must be anonymized.

### Architecture & video.js (`PlayerEngine`)
- **Finding:** The application strictly adheres to its "headless video.js" architecture. `PlayerEngine.svelte` initializes Video.js but strips its UI, relying entirely on Svelte for controls.
- **Finding:** The progress saving logic (`save_playback_progress`) is debounced securely using Tauri IPC `invoke`.
- **Logic Flaws:** The video player handles missing `dbSessionProgress` gracefully. No obvious logic flaws found.

---

## 3. Compliance & Documentation

### Version Control & Code Progress
- **Finding:** Both `package.json` and `Cargo.toml` declare version `0.1.0`.
- **Recommendation on Versioning:** Given the substantial logs in `DEV.log` (81 lines) and `flux-player/dev.log` (19 lines) outlining multiple feature additions (metadata parsing, database migrations, player engine setup), the current code state is actively evolving past a flat `0.1.0`. The `0.1.x` patch version **should indeed be bumped** (e.g., `0.1.1` or `0.1.2`) to reflect these iterative changes and align version control with the logged progress.

### Technical Debt, Logging, & Inline Comments
- **Logging vs. Audit:** The `DEV.log` and `flux-player/dev.log` files serve excellently as developer journals for architectural decisions, not traditional user-audit logs. This fulfills the `AGENTS.md` directive beautifully.
- **Inline Comments:** The codebase has reasonable inline comment density (found in ~30 distinct files). Comments like those in `PlayerEngine.svelte` (`// Kick off async setup in an IIFE so onMount stays synchronous`) are very helpful. However, **more inline comments explaining the *reasoning* behind complex state mutations in Svelte (`$state`/`$effect`) and Rust error handling would be highly beneficial** to ensure new developers can follow the "why" and not just the "what".

### README Scope
- **Finding:** The `README.md` is currently just the default Tauri + SvelteKit boilerplate.
- **Recommendation:** The `README.md` should be strictly focused on **Developer Onboarding**, not an "About the App" page for users (that belongs on a marketing site or in an internal "Help" menu). It should detail:
  1. Prerequisites (Rust, Node, system libraries like `libwebkit2gtk`).
  2. How to run the local development server (`npm run dev`).
  3. Where the database and logs are stored during development.

### License Compatibility
- **Finding:** Frontend `package.json` specifies `"license": "MIT"`. The Rust backend (`Cargo.toml`) lacks an explicit `license` field.
- **Recommendation:** Add `license = "MIT"` to `src-tauri/Cargo.toml`.

---

## 4. Infrastructure-as-Code (IaC) & Permissions

### Permissions & File System Access (`tauri.conf.json` & capabilities)
- **Finding:** The Tauri capability configuration uses `"fs:default"` and `"core:path:default"`.
- **Clarification on `fs` Restriction:** Media can be anywhere on the user's system. However, for maximum security, Tauri recommends using the `dialog` API (which you have enabled) so the OS prompts the user to grant access to specific files/folders at runtime. The app shouldn't inherently have silent background read/write access to the user's *entire* hard drive (e.g., `~/.ssh/` or `C:\Windows\System32`), only to the directories the user explicitly adds to their Flux Library. If possible, scope the `fs` permission to only allow reading from directories registered in the application's library settings.
- **SQL Execution:** `"sql:allow-execute"` allows arbitrary SQL execution from the frontend. This should be removed if the frontend only calls pre-defined Rust commands.

### Environment Parity
- **Finding:** There are no GitHub Actions workflows (`.github/workflows`) or CI/CD scripts present.
- **Recommendation:** Establish a basic CI pipeline that runs `npm run check`, `cargo clippy`, and `cargo audit`.
