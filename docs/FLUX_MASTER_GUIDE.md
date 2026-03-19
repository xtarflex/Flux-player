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

# Flux Master Guide - Additional Specifications

These additions provide visual specifications for the playback footer and detail panel.

---

## 6. Playback Footer Structure

### Dimensions & Layout
- **Height:** 90px (fixed)
- **Width:** 100% viewport width
- **Background:** `rgba(26, 26, 31, 0.95)` with `backdrop-filter: blur(10px)`
- **Z-Index:** 9997 (below Dynamic Island at 9999)

### Progress Scrubber
- **Position:** Above all playback controls (top of footer)
- **Height:** 
  - Inactive: 2px
  - Active (playing): 4px
  - Hover: 6px
- **Color:** Gradient `linear-gradient(90deg, #8a2be2, #00ffff)` (Electric Violet → Cyber Cyan)
- **Thumb:** 12px circle on hover, same gradient fill
- **Transition:** `height 0.2s ease, opacity 0.3s ease`

### Control Layout (Three Sections)
```
┌────────────────────────────────────────────────────────────────────┐
│ ━━━━━━━━━━━━━●━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━     │ ← Scrubber
│                                                                    │
│ [Thumb]  Title      [Controls]   [Queue Cards]   [Right Icons]     │
│ 64x64    + Time     Center         180px          Right-aligned    │
└────────────────────────────────────────────────────────────────────┘
```

#### Left Section (Thumbnail + Info)
- **Thumbnail:** 64x64px square
  - Default: Flux logo when no media selected
  - Active: Current media poster
  - Border-radius: 8px
  - Transition: `opacity 0.3s ease` when changing media
- **Title:** Max 2 lines, truncate with ellipsis
- **Time:** Format `MM:SS / MM:SS` in `Outfit` font

#### Center Section (Playback Controls)
- **Buttons:** [⟲ Shuffle] [⏮ Prev] [⏯ Play/Pause] [⏭ Next] [🔁 Repeat]
- **Size:** 40x40px each
- **Spacing:** 12px gap between buttons
- **Active State:** Cyan color (#00ffff) when enabled
- **Inactive State:** Grey (#666)

#### Queue Cards Section
- **Width:** 180px (fixed)
- **Card Stack:** 3 overlapping cards (prev, current, next)
- **Center Card:** 48x56px, elevated 4px, z-index 3
- **Side Cards:** 50% hidden behind center, z-index 1, opacity 0.6

#### Right Section (Icon Controls)
- **Buttons:** [1x Speed] [Volume Bars] [PiP] [Fullscreen]
- **Size:** 36x36px each
- **Spacing:** 8px gap
- **Alignment:** Right edge with 16px padding

### Volume Bars Specification
- **Container:** 48px width x 40px height
- **Bar Count:** 5 bars
- **Heights:** Progressive (12px, 18px, 24px, 30px, 36px)
- **Width per bar:** 6px
- **Gap:** 4px between bars
- **Colors:**
  - Active: Gradient based on volume level
    - 0-30%: Green to Yellow-Green
    - 30-70%: Yellow-Green to Yellow
    - 70-100%: Yellow to Red
  - Inactive: `#333`

---

## 7. Detail Panel Layout (Library)

### Panel Dimensions
- **Width:** 35% of viewport (min 400px, max 600px)
- **Height:** 100% of content area (minus titlebar/footer)
- **Background:** `#1a1a1f`
- **Border-left:** 1px solid `rgba(138, 43, 226, 0.2)`

### Hero Section (Backdrop + Poster)
```
┌────────────────────────────────────┐
│                                    │
│   LARGE BACKDROP (16:9 ratio)     │
│   Height: 40% of panel height     │
│                                    │
│  ┌─────┐                           │
│  │POST │                           │
│  │ ER  │                           │
│  │     │                           │
│  └─────┘                           │
│   ↑ Overlaps backdrop              │
│     bottom-left position           │
│     Extends 80px below backdrop    │
└────────────────────────────────────┘
```

#### Backdrop Image
- **Aspect Ratio:** 16:9 (forced)
- **Object-fit:** `cover`
- **Filter:** Subtle gradient overlay `linear-gradient(to bottom, rgba(0,0,0,0) 0%, rgba(26,26,31,0.8) 100%)`

#### Poster Overlay
- **Size:** 180x270px (2:3 aspect ratio)
- **Position:** Absolute, bottom-left of backdrop
  - Left: 24px from panel edge
  - Bottom: -80px from backdrop bottom (extends below)
- **Border:** 2px solid `rgba(138, 43, 226, 0.5)`
- **Border-radius:** 8px
- **Shadow:** `0 8px 24px rgba(0, 0, 0, 0.5)`

### Metadata Section (Beside Poster)
- **Position:** To the right of poster, aligned with poster bottom
- **Padding:** 24px from poster right edge

#### Title
- **Font:** `Syncopate`, Bold, 24px
- **Color:** `#f8f9fa` (Neon White)
- **Letter-spacing:** `0.05em`
- **Max lines:** 2, truncate with ellipsis

#### Info Row (Year • Duration • Rating)
- **Font:** `Outfit`, 14px
- **Color:** `rgba(248, 249, 250, 0.7)`
- **Separator:** ` • ` between items
- **Rating:** Star icon (★) + number

#### Genre Tags
- **Display:** Inline badges
- **Background:** `rgba(138, 43, 226, 0.2)`
- **Border:** 1px solid `rgba(138, 43, 226, 0.4)`
- **Border-radius:** 12px
- **Padding:** 4px 12px
- **Font:** `Outfit`, 12px
- **Gap:** 8px between tags

### Synopsis
- **Font:** `Outfit`, 15px, line-height 1.6
- **Color:** `rgba(248, 249, 250, 0.8)`
- **Max lines:** 4, fade-out overflow with `...`
- **Margin-top:** 16px from genre tags

### Play Button
- **Width:** 100% of panel width (minus 48px padding)
- **Height:** 48px
- **Background:** `linear-gradient(90deg, #8a2be2, #00ffff)`
- **Border:** none
- **Border-radius:** 8px
- **Font:** `Outfit`, Bold, 16px
- **Color:** `#ffffff`
- **Icon:** ▶ (12px before text)
- **Hover:** Brightness 1.1, scale 1.02
- **Transition:** `all 0.2s ease`

### Subtitle Row
- **Layout:** Flex row, space-between
- **Label:** "Subtitle:" in `Outfit` 14px, grey
- **Filename:** Current subtitle in `Outfit` 14px, cyan
- **Button:** [Change] in `Outfit` 14px, violet background, 6px border-radius

### Metadata Grid (Bottom Section)
- **Layout:** Vertical list
- **Row Height:** 40px per item
- **Border-top:** 1px solid `rgba(255, 255, 255, 0.1)` per row

#### Row Structure
```
┌─────────────────────────────────────┐
│ TITLE        Blue Lock The M...    │ [✎] [🔄]
│ DIRECTOR     Shunsuke Ishikawa     │
│ STARRING     Nobunaga Shimaz...    │
└─────────────────────────────────────┘
```

- **Label:** `Outfit` 12px, uppercase, grey, 80px width
- **Value:** `Outfit` 14px, white, flex-grow, truncate
- **Icons:** 24x24px, hover brightness 1.2

---

These specifications ensure pixel-perfect implementation of the playback footer and detail panel.