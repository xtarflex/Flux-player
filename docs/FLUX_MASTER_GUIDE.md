# Flux Master Reconstruction Guide

> [!IMPORTANT]
> This guide contains the "DNA" of Flux. Use this to restore the app after reverting to commit `8218db7`.

## 1. Brand & Aesthetics
- **Core Palette**:
  - `Electric Violet`: `#8a2be2`
  - `Cyber Cyan`: `#00ffff`
  - `Neon White`: `#f8f9fa`
  - `Base Background`: `#0a0a0c`
  - `Surface Elevated`: `#1a1a1f`
- **Typography**:
  - **Import**: `<link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700&family=Syncopate:wght@400;700&display=swap" rel="stylesheet">`
  - **Headings**: `var(--font-heading)` -> `'Syncopate', sans-serif` (Bold, Letter-spacing `0.05em`)
  - **Body**: `var(--font-body)` -> `'Outfit', system-ui, -apple-system, sans-serif`
- **Ken Burns Effect**:
  - Applied to `.poster-img` and `.fallback-icon` in `MediaGrid.svelte`.
  - Timing: `1.7s`
  - Easing: `cubic-bezier(0.23, 1, 0.320, 1)`
  - Scale: `1.1` on hover.

## 2. Dynamic Island Physics
- **Spring Parameters**:
  - Stiffness: `0.15`
  - Damping: `0.35`
- **Morphing Paths (F-L-U-X)**:
  - F: `M54.25 131V84.125V37.25H114.25V58.8125H79V73.3125H111.75V94.875H79V131H54.25Z`
  - L: `M116 131H84.125H52.25V84.125V37.25H77V73.3438V109.438H96.5H116V131Z`
  - U: `M126.25 36.25V61.9688V87.6875...` (extracted from `DynamicIsland.svelte`)
- **State Dimensions**:
  - Idle: `56x56` (Radius 28)
  - Hover: `220x60` (Radius 30)
  - Status: `320x48` (Radius 24)
  - Playing: `130x48` / `240x56` (Audio)

## 3. Structural Layout (Absolute System)
- **Top Offset**: `30px` (Titlebar)
- **Sidebar Width**: `250px` (Regular) / `80px` (Collapsed)
- **Footer Height**: `90px` (PlaybackBar)
- **CSS Hierarchy**:
  - `.app-layout` -> `position: relative; overflow: hidden;`
  - `.gallery-content` -> `position: absolute; inset: 0; top: 30px; left: 250px; bottom: 90px;`

## 4. Critical Logic Fragments
- **TMDB Scan / Refresh**:
  - Background concurrency limit: `5`
  - Merge logic: Keep TMDB data (`metadata?.title`), update basic file info (`size`, `dateAdded`).
  - Cache first load: `getSavedMediaList()` then merge with `scanDirectoryForVideos(dir)`.
- **Subtitle Binding**:
  - Global pool of SRT/VTT files extracted during directory scan.
  - Fuzzy matching based on filename tokens.
- **Player Resilience**:
  - Always check `if (!player) return;` after async `await` in `PlayerEngine.svelte` to prevent race condition crashes.

## 5. Deployment Checklist
- [ ] frameless window: `true` in `tauri.conf.json`.
- [ ] `vite.config.js` shim: `global: 'globalThis'`.
- [ ] `optimizeDeps.include: ["video.js"]` for ESM/CJS interop.
