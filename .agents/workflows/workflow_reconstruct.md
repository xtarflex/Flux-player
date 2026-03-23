# Workflow: Flux V1 Reconstruction

When instructed to begin building, follow this sequential execution map. Refer to `docs/PROJECT_CHARTER.md` and `docs/UX_JOURNEYS.md` for full context.

## Target 1: The OS Skeleton & Navigation
1. **Initialize Shell:** Configure `tauri.conf.json` for a frameless window (`decorations: false`).
2. **Global Titlebar:** Build the custom drag-region titlebar. Include the Squircle Profile Avatar in the top right.
3. **Dynamic Island Foundation:** Build the `<DynamicIsland />` component. Implement the Idle state with the morphing F-L-U-X SVG paths (Remember: Crisp border, NO glow).
4. **Macro Layout:** Set up the CSS Grid (Sidebar, Main Content Area, Footer Playback Bar).
5. **Context Search:** Add the instant 0ms local search bar for the Library, and the 500ms debounced search bar for the Discovery  screen.

## Target 2: The Settings Hub & API Delegation
1. **Settings Layout:** Build the Hub with vertical tabs (Identity, Storage, Playback, Appearance, Network).
2. **API Delegation UI (The 150 Rule):**
   * Implement the local SQLite tracker for the 150-call shared API pool.
   * Build the "Unlock Infinite Discovery" Modal.
   * Implement the TMDB "Cheat Sheet" copy-paste UI for users to input their own TMDB API key.
3. **User Profile:** Implement the local OS username extraction and the Squircle avatar uploader/cropper.

## Target 3: Core Library Brain & Detail Drawer
1. **Local SQLite Engine:** Setup the SQLite schema for tracking files, watch time, and `currentTime` resume states.
2. **Offline Image Sync:** Implement the Rust command to intercept TMDB image URLs, download the `.jpg` blobs to `%APPDATA%`, and serve them locally.
3. **The Detail Panel:**
   * Build the Off-Canvas Drawer logic for the Discovery screen (sliding from the right over a blurred backdrop).
   * Include the Trailer Lightbox logic.
   * Include the "Resume from [MM:SS]" conditional button.

## Target 4: Streaming & Subtitles (The Scrapers)
1. **yt-dlp Sidecar:** Implement the Zero-Click clipboard intercept logic. Route URLs to the sidecar and pipe the `.m3u8` into Video.js.
2. **Scraper Cascade:** Build the silent failover logic: VidSrc -> VidLink -> Consumet.
3. **Local Subtitles (OSHash):** Implement the OpenSubtitles API integration, ensuring files are downloaded locally alongside the `.mkv` files and bound instantly via `flux-bind-subtitle`.