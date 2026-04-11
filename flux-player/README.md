# Flux Player Developer Guide

For the general user overview and project features, please see the [Root README](../README.md).

# Flux Player (v0.2.0)

Flux is a high-performance, glassmorphic media suite for local and streaming content, built with **Tauri 2.0**, **Svelte 5**, and **Rust**.

## 🛠️ Developer Onboarding

### Prerequisites
- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (1.75+)
- **Node.js**: [Install Node.js](https://nodejs.org/) (20+) 
- **System Dependencies** (Linux only):
  - `libwebkit2gtk-4.1`
  - `libappindicator3`
- **Tauri CLI**: `npm install -g @tauri-apps/cli`

### Setup & Local Development
1. **Install Dependencies**:
   ```bash
   cd flux-player
   npm install
   ```
2. **Environment Variables**:
   - Create a `.env` file in `src-tauri/` (see `src-tauri/.env.example` if available).
   - Add `TMDB_KEY_1=your_api_key` for metadata enrichment.
3. **Run Development Server**:
   ```bash
   npm run tauri dev
   ```
   This will start the Vite frontend and compile the Rust backend with hot-reload.

### Project Structure
- `src/`: Svelte frontend (Svelte 5 `$state` and `$effect`).
  - `lib/stores/`: Global state management (Playback, Library, Queue).
  - `lib/components/`: Modular UI components.
- `src-tauri/`: Rust backend.
  - `src/commands/`: Tauri IPC command implementations.
  - `src/database/`: SQLite schema and migrations.
  - `src/scanner/`: Media metadata extraction (Lofty + TMDB).
  - `capabilities/`: Tauri v2 permission system.

### Security & Audits
- **SQL Safety**: All queries are parameterized. Direct frontend SQL is disabled in favor of pre-defined Rust commands.
- **Error Handling**: Uses a centralized `AppError` enum for sanitized error reporting via IPC.
- **Vulnerability Scans**: Regularly run `npm audit` and `cargo audit`.

## 🎨 Design System
Flux follows a **"Cyber Dark"** aesthetic:
- **Glassmorphism**: 1px translucent borders and high-key blurs.
- **Dynamic Island**: Context-aware UI morphing for playback notifications.
- **Ken Burns**: Cinematic backdrop animations in the detail view.

---
*Developed by the Flux Development Team.*
