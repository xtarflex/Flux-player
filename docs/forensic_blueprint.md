# Flux Player: Forensic Blueprint

This document explains the technical "sleight of hand" and magic behind Flux Player's most premium UI/UX features. Future developers and AI agents must reference this to understand how to implement these features without degrading performance.

## 1. Adaptive UI Tinting (Canvas Sampling)

**Goal:** Change the UI color to match the dominant color of the active media.

*   **Canvas Bridge:** We don't use heavy libraries. Instead, `getDominantColor()` creates an in-memory `HTMLCanvasElement` (64x64).
*   **Sampling:** It draws the poster image to the canvas, then uses `getImageData()` to extract the raw pixel array.
*   **Optimization:** It averages exactly 1000 pixels (calculated via a `step` variable) to produce an `rgb()` string.
*   **Binding:** This `rgb` string is bound to the CSS variable `--island-adaptive-tint` in `DynamicIsland.svelte`.
*   **The 1px Adaptive Border:** This tint is applied strictly to the 1px border. We **do not** use `box-shadow` or `drop-shadow` for glows to avoid performance degradation.

## 2. The Dynamic Island: Morphing & Physics

**Goal:** Create a context-aware HUD that feels kinetic and "zero-gravity."

*   **Svelte Spring Physics:** Dimensions (`width`, `height`, `border-radius`) are bound to a Svelte `spring` store (`stiffness: 0.15`, `damping: 0.35`). This creates the signature bounce when expanding from Idle (`56x56`) to Status (`320x48`).
*   **SVG Logo Morphing:** The Idle state logo (F-L-U-X) uses `flubber` for topological interpolation, ensuring smooth transitions even when paths have different vertex counts. A Svelte `tweened` store (`1200ms`, `cubicInOut`) handles the animation.
*   **Micro-Player Transition:** Triggered via `Tauri.invoke('toggle_micro_player')`. The backend snaps the window size instantly, while the frontend fakes the motion using the spring-loaded island overlay.

## 3. Vinyl Audio Mode (Kinetic Identity)

**Goal:** Make audio playback feel visually distinct from video.

*   **CSS Keyframes:** A `rotate(360deg)` animation named `island-spin` is applied to the album art.
*   **The "Pulse":** The ambient glow pulse frequency increases from `3s` (video) to `1s` (audio) to match a turntable's energy.
*   **Conditional Class:** Applied via Svelte template logic: `class="island-thumb {$currentMedia.type === 'audio' ? 'vinyl-spin' : ''}"`.

## 4. Zero-Latency (0ms) Local Search

**Goal:** Instant library filtering that feels faster than a database query.

*   **The Trick:** SQLite passes the *entire* media list to Svelte once.
*   **The Execution:** Search is performed frontend-side by applying reactive CSS classes (e.g., `hidden` or `opacity-0 scale-95`) to grid items that don't match the query.
*   **The Result:** The GPU handles the filtering via transitions, resulting in literal 0ms perceived latency.

## 5. The Off-Canvas Detail Drawer (Home Screen)

**Goal:** Show media details without breaking the horizontal scroll position of the Home Screen.

*   **The Problem:** Pushing rows in the Home Screen forces expensive layout recalculations.
*   **The Magic:** The Detail Panel uses `position: fixed; right: -100%;` and slides in over the UI. 
*   **Glassmorphism:** A full-screen backdrop with `backdrop-filter: blur(5px)` and `rgba(10, 10, 12, 0.7)` dims the background rows while preserving their scroll state.

## 6. Fuzzy Subtitle Binding (Token Interaction)

**Goal:** Match video files with subtitles even when names aren't identical.

*   **Tokenization:** Filenames are split into tokens using `/[^a-z0-9]+/`.
*   **Intersection Scoring:** Shared tokens are counted. A minimum score (typically 2) is required to prevent false matches.
*   **Perfect Match Boost:** If a subtitle name starts with the cleaned video name, it skips scoring for a "Perfect Match."

## 7. OSHash: Frame-Perfect Subtitles

**Goal:** Guaranteed subtitle sync regardless of file metadata.

*   **The Trick:** The Rust backend generates a 64-bit checksum (OSHash) by reading the first and last 64kb of the video file.
*   **The Result:** This hash identifies the *exact* file version on OpenSubtitles, ensuring the returned `.srt` is timed perfectly for that specific rip.

## 8. Immersive Auto-Hide (VLC-Style UI)

**Goal:** Distraction-free playback that reacts to user presence.

*   **Inactivity Timer:** A 3-second timer in `PlayerEngine.svelte` is reset on every `mousemove`.
*   **Smooth Transitions:** Uses `opacity` and `pointer-events: none` instead of toggling `display`.
*   **Cursor Stealth:** Sets `cursor: none` when the UI is hidden to keep the movie in focus.

## 9. Custom VTT Compiler (Regex Transformation)

**Goal:** Convert SRT to WebVTT on-the-fly for native `<track>` support.

*   **High-Speed Regex:** `text.replace(/(\d{2}:\d{2}:\d{2}),(\d{3})/g, "$1.$2")` converts time stamps.
*   **Formatting:** Prepends the `WEBVTT` header and normalizes line endings to LF.

## 10. The Frameless Titlebar & Drag Regions

**Goal:** Modern, native look without standard OS window borders.

*   **Custom Header:** Built with HTML/CSS. Draggable region defined via `data-tauri-drag-region`.
*   **The Catch:** Interactive elements (buttons, avatars) within the header MUST NOT have the drag attribute, or click events will be swallowed by the window drag logic.

## 11. Context Menu Smart Positioning

**Goal:** Floating menus that never overflow the viewport.

*   **Layering:** High `z-index` (9999) ensures it stays on top of the Dynamic Island.
*   **Glassmorphism:** Deep blur (`20px`) with semi-transparent background.
*   **Alignment:** Uses `transform: translate(-100%, 0)` for right-side alignment, preventing the menu from clipping off the screen edge.

## 12. Intelligent Grid Scaling (Smooth Zoom)

**Goal:** Fluid resizing of the media library.

*   **CSS Grid:** Uses `repeat(auto-fill, minmax(VAR, 1fr))` where `VAR` is the zoom step.
*   **Quantized Steps:** Fixed sizes (130px, 220px, 320px) ensure the layout remains predictable.
*   **Transitions:** `transition: grid-template-columns 0.4s ease` on the parent makes the grid "flow" between states.

## 13. The Squircle Mask (Apple-Style Curves)

**Goal:** Premium, mathematically perfect rounded corners (super-ellipses).

*   **The Execution:** Uses an SVG `clip-path` instead of `border-radius: 50%`.
*   **CSS Example:** `clip-path: path('M 0,16 C 0,0 0,0 16,0 C 32,0 32,0 32,16 C 32,32 32,32 16,32 C 0,32 0,32 0,16 Z');`
*   Applied to user avatars for a high-end, bespoke aesthetic.
