# Skill: Aesthetic & UI/UX Enforcer

When writing Svelte components and CSS for Flux Player, you MUST strictly adhere to these design rules. Any deviation will break the premium cinematic feel of the application.

## 1. Typography
* **Headings:** MUST use `'Syncopate', sans-serif` (Bold, Letter-spacing `0.05em`).
* **Body/UI Text:** MUST use `'Outfit', system-ui, -apple-system, sans-serif`.
* Do not use standard system fonts for prominent UI elements.

## 2. Color Palette (Cyber Dark Theme)
* `Base Background`: `#0a0a0c`
* `Surface Elevated`: `#1a1a1f`
* `Electric Violet` (Primary Accent): `#8a2be2`
* `Cyber Cyan` (Secondary Accent): `#00ffff`

## 3. The Dynamic Island (CRITICAL RULES)
* **NO AMBIENT GLOW:** The Dynamic Island must **never** feature a `box-shadow` or outer ambient pulse/glow. 
* **Crisp Borders Only:** The Island relies strictly on a crisp `1px` border. 
* **Adaptive Tint:** During playback, this `1px` border dynamically changes color to match the dominant color of the active media's poster/album art.
* **Offline State:** If the network drops, the `1px` border turns sharp red to indicate offline status.
* **Morphing:** Use Svelte spring physics (Stiffness: 0.15, Damping: 0.35) for morphing between Idle (56x56), Status (320x48), and Audio (240x56) states.

## 4. UI Shapes & Masks
* **Avatars (Squircles):** User avatars must NEVER be standard circles (`border-radius: 50%`). They must use a CSS `clip-path` super-ellipse (Squircle) to maintain a premium Apple-like aesthetic. 
  *CSS implementation: `clip-path: path('M 0,16 C 0,0 0,0 16,0 C 32,0 32,0 32,16 C 32,32 32,32 16,32 C 0,32 0,32 0,16 Z');` (Scale relative to size).*
* **Glassmorphism:** Overlays (like the Detail Panel Drawer, Trailer Lightbox, and API Cheat Sheet modal) MUST use `backdrop-filter: blur(5px)` (or `10px` for deep focus) with a dark semi-transparent tint.

## 5. CSS Architecture
* **Layout:** Use standard CSS Grid for the macro layout (`.app-layout`).
* **Absolute Positioning:** Reserved **EXCLUSIVELY** for overlays (Context Menus, Dynamic Island, Modals, Off-Canvas Drawers). Do not use absolute positioning for standard grid items.
* **Animations:** The Ken Burns effect on posters must be exactly `1.7s` using `cubic-bezier(0.23, 1, 0.320, 1)` scaling to `1.1`.