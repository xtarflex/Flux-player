# Flux Player: Responsive Design & Layout Rules

This document defines the responsive scaling behavior and layout shifting for Flux Player. As a desktop application (via Tauri), Flux does not rely on typical web "device" breakpoints (Mobile/Tablet), but rather OS-specific window size classes.

---

## 1. DESIGN PHILOSOPHY
Our layout must gracefully adapt to window resizing. The objective is fluid reduction: we don't break the UI immediately; we gradually hide non-critical elements (Queue Stack, Secondary Controls, Sidebar labels) to always protect the core media UX.

**Key Concepts:**
- **The 4px Grid:** All margins, paddings, and font sizes strictly use multiples of 4px.
- **Fluid Spring (`clamp`)**: Used for micro-spacing (icon gaps, scrubber margins) so they organically squish together before a hard breakpoint is reached.
- **Size Classes**: Hard CSS `@media` queries used exclusively for structural layout shifts.

---

## 2. OS-SPECIFIC SIZE CLASSES (Breakpoints)

Flux uses three primary size classes governing the layout:

### A. Small (Compact Mode)
**Breakpoint**: `< 640px` width
**Target**: Split-screen mode or snapped window widget.
**Layout Changes:**
- **Sidebar**: Fully collapses to **80px** icon-only mode (`var(--sidebar-collapsed-width)`).
- **Library Grid**: Single column list or 2-column micro grid.
- **Footer**: Drops the Queue Stack, Speed Badge, Subtitles, and PiP controls. Only Play/Pause, Seek, and Volume remain visible.
- **Media Info**: Truncates heavily with `text-overflow: ellipsis`.

### B. Medium (Floating Multitask)
**Breakpoint**: `641px - 1007px` width
**Target**: Casual floating window or tablet-size layout.
**Layout Changes:**
- **Sidebar**: Visible but compact (or dismissable via hamburger if desired later).
- **Library Grid**: Fluid grid mapping (3-4 columns) via `repeat(auto-fill, minmax(...))`.
- **Footer**: Retains full right-side actions but hides the Queue Stack to protect space.

### C. Large (Full Desktop Experience)
**Breakpoint**: `1008px` and up
**Target**: Standard PC or Laptop maximizing.
**Layout Changes:**
- **Sidebar**: Standard **250px** width (`var(--sidebar-width)`).
- **Library Grid**: Full expansion (5+ columns).
- **Footer**: All components visible. Queue Stack cards elegantly offset.

### D. Ultrawide Lock
**Breakpoint**: `>= 1920px` width
**Target**: Extra-large desktop monitors.
**Layout Changes:**
- Centers the main `.app-container` with a `max-width: 1600px; margin: 0 auto;` to prevent text layout stretching into infinity.

---

## 3. VERTICAL RESPONSIVENESS (Height Restraints)

Horizontal width is typically the focus, but a desktop window can become horizontally wide and vertically squished (banner shape). 

**Breakpoint**: `max-height: 500px`
**Layout Shifts:**
- `.playback-footer` shrinks from `90px` to `70px`.
- Large buttons and avatars scale down slightly.
- The Player Queue posters slice down vertically.
- Island `margin-top` reduces to maximize vertical real estate.

---

## 4. CSS IMPLEMENTATION STANDARDS

All variables are tracked in `app.css`. 

**Example Fluid implementation in CSS:**
```css
.footer-gap {
  gap: clamp(8px, 2vw, 24px); 
}
```

**Example Media Query Usage:**
```css
@media (max-width: 1007px) {
  .queue-stack {
    display: none;
  }
}

@media (max-width: 640px) {
  .sidebar {
    width: var(--sidebar-collapsed-width); /* 80px */
  }
  .visualizer-btn, .speed-badge {
    display: none;
  }
}
```

## 5. RELATION TO OTHER DOCS
- Works hand-in-hand with `app.css` (`--sidebar-width`, `--footer-height` tokens).
- Influences the **Discovery Hub** components (as per `UX_JOURNEYS.md`).
- Driven entirely by OS size specs outlined in the `PROJECT_CHARTER.md` for a premium desktop feel.
