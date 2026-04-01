# Flux Player: Comprehensive Audit Report

**Date:** 2026-04-01
**Focus Areas:** Code Quality & Security, Architecture & Data, Compliance & Documentation, Infrastructure-as-Code (IaC)

---

## 1. Code Quality & Security

### Hardcoded Secrets
- **Finding:** A hardcoded API Key fallback error message `API_KEY_NOT_FOUND` exists, but `settings::get_tmdb_key` is used to fetch the TMDB API key dynamically from settings rather than hardcoding it in the source. This is good practice.
- **Location:** `src-tauri/src/scanner/tmdb.rs` and `src-tauri/src/commands/settings.rs`
- **Recommendation:** Ensure the initial setup flow securely prompts the user for the TMDB API key.

### Dependency Vulnerabilities
- **Frontend (`npm audit`):**
  - **High Severity:** `@xmldom/xmldom` (XML injection) and `picomatch` (ReDoS & Method Injection).
  - **Recommendation:** Run `npm audit fix` and update dependencies.
- **Backend (`cargo audit`):**
  - **Unsoundness:** `glib` crate (v0.18.5) has a known unsoundness issue.
  - **Recommendation:** Update `glib` to >= 0.20.0.

### Input Validation & SQL Injection
- **Finding:** All observed SQLite database queries in `src-tauri/src/database/queries.rs` and `commands/library.rs` correctly use parameterized queries (e.g., `rusqlite::params![...]`).
- **Conclusion:** Safe from standard SQL Injection.

### Error Handling
- **Finding:** Rust backend frequently uses `.map_err(|e| e.to_string())?` to pass errors to the frontend.
- **Risk:** This might leak internal database paths or Rust panic traces to the UI.
- **Recommendation:** Implement a custom `AppError` enum that implements `serde::Serialize` to return sanitized, user-friendly error codes.

---

## 2. Architecture & Data

### Database Schema
- **Finding:** The SQLite schema (`schema.rs`) is well-structured with appropriate normalization.
- **Data Privacy:** Watch history (`is_watched`, `last_position`) is stored in plain text locally.

### Architecture & video.js (`PlayerEngine`)
- **Finding:** The application strictly adheres to its "headless video.js" architecture. `PlayerEngine.svelte` initializes Video.js but strips its UI, relying entirely on Svelte for controls.
- **Finding:** The progress saving logic (`save_playback_progress`) is debounced securely using Tauri IPC `invoke`.

---

## 3. Compliance & Documentation

### Version Control & Code Progress
- **Finding:** Both `package.json` and `Cargo.toml` declare version `0.1.0`.
- **Recommendation:** The `0.1.x` patch version **should be bumped** (e.g., `0.1.1`) to reflect iterative changes.

### Technical Debt, Logging, & Inline Comments
- **Logging vs. Audit:** The `DEV.log` and `flux-player/dev.log` files serve excellently as developer journals for architectural decisions.
- **Inline Comments:** The codebase has reasonable inline comment density. More comments explaining the *reasoning* behind state mutations in Svelte and Rust error handling would be beneficial.

### README Scope
- **Finding:** The `README.md` is currently just the default Tauri + SvelteKit boilerplate.
- **Recommendation:** The `README.md` should be strictly focused on **Developer Onboarding**.

### License Compatibility
- **Finding:** Frontend `package.json` specifies `"license": "MIT"`. The Rust backend (`Cargo.toml`) lacks an explicit `license` field.
- **Recommendation:** Add `license = "MIT"` to `src-tauri/Cargo.toml`.

---

## 4. Infrastructure-as-Code (IaC) & Permissions

### Permissions & File System Access (`tauri.conf.json` & capabilities)
- **Finding:** The Tauri capability configuration uses `"fs:default"` and `"core:path:default"`.
- **Clarification on `fs` Restriction:** Tauri recommends using the `dialog` API so the OS prompts the user at runtime. Scope the `fs` permission to only allow reading from directories registered in settings.
- **SQL Execution:** `"sql:allow-execute"` allows arbitrary SQL execution from the frontend. This should be removed if the frontend only calls pre-defined Rust commands.

### Environment Parity
- **Finding:** There are no GitHub Actions workflows (`.github/workflows`) or CI/CD scripts present.
- **Recommendation:** Establish a basic CI pipeline that runs `npm run check`, `cargo clippy`, and `cargo audit`.
