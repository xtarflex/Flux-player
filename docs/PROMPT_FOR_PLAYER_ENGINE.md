# Prompt for Player Engine Developer: Flux Now Playing & Persistence

## Task: Implement Video.js Engine, Theater Mode, and Playback Persistence

### Objective
Build the immersive playback experience for Flux, connecting the `$playback` store to a functional engine, implementing full-screen "Theater Mode" with overlay UI, and ensuring state persistence via the SQLite backend.

---

### Step 1: Backend Persistence (Milestone 1.6)
1.  **Database Update**: Ensure the `media` table in SQLite has a `last_position` (INTEGER) column for tracking seconds.
2.  **Tauri Commands**: Implement:
    - `save_playback_progress(path, seconds)`: Debounced update to the database.
    - `get_playback_progress(path)`: Fetch stored timestamp on media load.
3.  **The "Watched" Rule**: Automatically mark a file as `is_watched = true` when `currentTime / duration >= 0.9` (90% completion).

---

### Step 2: Global Layout & Theater Mode
1.  **CSS Grid Transformation**: Modify `.app-container` in `app.css` to support a `.player-mode` state. In this mode:
    - `main` grid area expands to fill the entire window (`1 / 1 / -1 / -1`).
    - `sidebar` and `playback-footer` become **floating glass overlays** with high `z-index`.
2.  **Kinetic Animations**: Implement `transition: transform` for the Sidebar and Footer.
3.  **Edge Detection Reveal**:
    - **Footer**: Slide up when mouse hovers within the bottom 50px.
    - **Sidebar**: Slide right when mouse hovers within the left 50px.
    - **Auto-Hide**: Hide UI and cursor after 3 seconds of inactivity during video playback.

---

### Step 3: Navigation & In-App Mini Player
1.  **Route Transition**: Intercept navigation away from `/playing` while a video is active.
2.  **The Mini-Player**: Instead of stopping, the video should shrink into a **Mini-Player** overlay (approx. 320x180px) in the bottom-right of the app.
3.  **Interaction**: Clicking the Mini-Player must route the user back to the full `/playing` view.

---

### Step 4: The Player Route (`/playing`)
1.  **Video Engine**: Integrate **Video.js**. Resolve local files via the `asset://` protocol.
2.  **Audio Engine (Vinyl Mode)**: 
    - Implement a large rotating album art disc in the center.
    - **Adaptive Tinting**: Extract dominant color from the art and apply it to the Dynamic Island 1px border.
    - **Sidebar Logic**: During audio playback, the sidebar must automatically collapse to **icon-only mode** to preserve the ambient visualizer space.

---

### Step 5: Detail Panel & Library Integration
1.  **Hydration**: Ensure a single click on a MediaCard hydrates the footer, while a double click routes to `/playing`.
2.  **Dynamic Action Button**: In `DetailPanel.svelte`, the primary action button (Play) must dynamically transform into **"Resume from MM:SS"** if `last_position > 0`. Clicking it must seek the player to that timestamp upon initialization.

---

### Deliverables
- Functional Video.js player in `/playing` route.
- Working SQLite persistence for progress and watched status.
- Immersive Theater Mode with auto-hiding/edge-hover UI.
- In-app Mini-Player functionality for video navigation.
- Kinetic Vinyl visualizer for audio tracks.
