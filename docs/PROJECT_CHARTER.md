# Flux Player: Project Charter & Roadmap

## 1. Project Summary

Flux Player is a blazing-fast, cross-platform desktop media player built using Tauri and Svelte. Starting as a personal project, it bypasses standard browser limits to interact directly with the PC's file system, wrapped in a sleek, cinematic UI featuring a morphing "Dynamic Island."

## 2. Goals and Objectives

* Build a fully functional, locally hosted desktop media player.

* Implement a highly dynamic, context-aware user interface.

* Maintain "zero-gravity" performance using native OS hardware acceleration.

* Seamlessly blend local playback with advanced streaming and algorithmic audio features.

* **Offline-First Architecture:** Ensure the application remains 100% functional (including cached cinematic images) for local media playback regardless of internet connectivity.

* **Sustainable API Delegation:** Protect the developer's core API keys by smoothly onboarding shared users to generate their own free API keys via a guided, frictionless UI.

## 3. Core Application Sections

* **Global Titlebar & Command Center:** A custom, frameless OS titlebar with native window management, a Global Refresh button, a Connectivity Indicator, and a Mini Profile Avatar (squircle).

* **The Dynamic Island (Floating HUD):** Positioned at the top-center, floating below the titlebar. Morphs between Idle, Hover, Drop, and Playing states. Features an adaptively tinted border during playback, and a solid red border when offline (strictly no outer glow).

* **The Discovery screen (Dashboard):** A Netflix-style cinematic landing page.

  * **Online Search:** A glassmorphic search bar that queries the TMDB API (500ms debounce) to discover movies/shows not currently downloaded.

  * **Personalized Greeting & Curated Rows:** Dynamic user greetings and horizontally scrolling rows (Just Released, Trending, Upcoming).

* **The Library:** A unified view aggregating multiple "Base Folders."

  * **Local Search:** An instant, zero-latency (0ms) search bar querying the local SQLite database.

  * **Filter & Sort:** Robust Status Filters (Watched/Not Watched), Sort Logic, and Grid/List view modes.

* **The Detail Panel (Context-Aware Split View):**

  * Acts as a docked panel in the Library, and an "Off-Canvas Drawer" on the Discovery  screen.

  * *Streaming Intelligence:* Dynamically morphs to offer "Stream Online" when media is not stored locally.

  * *Resume State:* Detects previously watched media and offers a "Resume" button.

* **Smart Interaction & Batch Selection:** Single-click audio, double-click video, and right-click batch selection.

* **The Settings Hub:** Control center featuring a dedicated **My Profile** tab, Base Folders, FFmpeg/GPU toggles, UI theming, and an Auto-Fetch Subtitles toggle.

## 4. Project Scope & Expanded Version Roadmap

### Version 1 (V1): The MVP (Minimum Viable Player)

* **UI/UX Skeleton:** Frameless titlebar, floating Dynamic Island, Discovery screen greeting, and CSS Grid layout.

* **Search & Filters:** Instant local search bar for the Library. Primary filters and A-Z sorting.

* **Identity & Settings:** Basic "My Profile" setup, Base Folder configuration, and the API Delegation On-Ramp (150 free calls).

* **Playback:** Video.js core with local file access, playback state memory (resume watching), and Flux logo fallbacks.

### Version 2 (V2): The Power User

* **Cinematic Discovery  & Discovery:** TMDB online search bar, Trending rows, and the Off-Canvas Detail Drawer.

* **Offline Image Caching:** Rust backend seamlessly downloads and serves TMDB poster/backdrop assets locally.

* **Streaming Pathways (Scraper Cascade):** Integration with redundant APIs (VidSrc, VidLink, Consumet) to guarantee successful "Click and Stream" movie scraping. Combined with a background subtitle fetcher and clipboard interception for `yt-dlp` extraction via the Dynamic Island.

* **The Format Breaker:** FFmpeg/FFprobe sidecar for `.mkv` and `.avi` transcoding. Includes intelligent on-demand conversion with progress tracking, background batch processing for V3, and automatic codec detection.

* **Advanced Library Brain:** SQLite integration (tracking Watched status, generating User Stats, and caching metadata).

### Version 3 (V3): The Ultimate

* **The Audio Forge:** FFT BPM Analyzer, RMS Smart Crossfader, and weighted Grandmaster Shuffle logic.

* **Visual Polish:** Interactive liquid visualizer engines for the Island.

* **Local Ecosystem:** Local network casting (Chromecast/DLNA) and in-app download management.

# Project Charter - Status Update

This updates Section 5 (Execution Map) to reflect newly documented features.

---

## 5. Execution Map (Status) — UPDATED

### 🟢 ACCOMPLISHED

* Native Shell (Tauri + SvelteKit)
* Dynamic Island physics, morphing logic, spacing geometry, and exact border aesthetic rules
* Library Layouts (Grid/List/Detail) and Batch Selection memory logic
* **Detail Panel Structure** (backdrop + poster overlap design, metadata refresh button)
* **Playback Footer** (persistent across sections, gradient scrubber, three-card queue preview)
* **VLC-Style Volume Control** (progressive bars with gradient)
* **Playlist Architecture** (manual, smart, favorites)
* **Queue System** (three-card preview, session-based storage, auto-advance)
* **Subtitle Binder Modal** (fuzzy matching, OpenSubtitles search, manual binding)

---

### 🟡 IN PROGRESS (Documented, Not Yet Implemented)

* Context-Aware Search Bars (Discovery  vs. Library)
* User Identity UI (Titlebar Avatar, Discovery  Greeting, Avatar Uploader)
* Offline Image Caching & Fallback UI logic
* Scraper Cascade & Auto-Fetch Subtitle engine
* API Key Delegation Flow (150-call rotation pool and Setup UI)
* Playback Resume State Memory
* Sidecars (FFmpeg & `yt-dlp`)
* In-App Mini Player (section-switching PiP)
* OS-Level PiP Mode (dedicated button)

---

### 🔴 NOT YET ACCOMPLISHED

* Discovery screen Curated Rows (TMDB Trending, Upcoming, Recently Released)
* Off-Canvas Detail Drawer (Discovery screen only)
* Trailer Lightbox & yt-dlp clipboard intercept
* FFmpeg Format Breaker (`.mkv` and `.avi` transcoding)
* Advanced Library Brain (watch stats, user insights)
* Audio Forge (FFT BPM Analyzer, RMS Crossfader, Smart Shuffle)
* Liquid Visualizer Engines

---

### 📝 NEWLY DOCUMENTED

These features are now fully documented with specifications, UX flows, and technical implementation details:

1. **Playlist System**
   - Manual playlist creation (batch select, context menu, playlist screen)
   - Smart playlists (auto-generated by criteria)
   - Favorites auto-playlist
   - SQLite schema and data structures
   - Sidebar display rules (5 most recent + pinned)

2. **Queue System**
   - Session-based queue (memory-only, not persisted)
   - Three-card preview in playback footer
   - Queue panel (expandable popup)
   - Full queue view (dedicated screen)
   - Auto-advance logic (Netflix-style seamless transitions)

3. **Playback Footer**
   - Persistent across all sections
   - Gradient progress scrubber (violet → cyan)
   - Three-card queue preview (overlapping depth effect)
   - VLC-style volume bars (progressive heights, gradient colors)
   - Context-aware thumbnail (Flux logo → active media poster)

4. **Detail Panel (Library)**
   - Large 16:9 backdrop (40% panel height)
   - Poster overlay (extends below backdrop)
   - Metadata beside poster (year, duration, rating, genres)
   - Full-width gradient play button
   - Subtitle binder integration
   - Metadata edit/refresh controls

5. **Subtitle Binder Modal**
   - Fuzzy matching with percentage scores
   - Local subtitle pool display
   - OpenSubtitles search integration
   - OSHash-based exact matching
   - Auto-download and binding
   - Context menu shortcut ("Get Subtitles")

6. **Playback Modes**
   - Sequential (default)
   - Shuffle (Fisher-Yates randomization)
   - Repeat Playlist (loop indefinitely)
   - Repeat One (loop current track)

7. **Library Queue Behavior (Settings-Controlled)**
   - Never (single-item playback)
   - Smart (auto-queue series/albums only)
   - Always (queue from filtered view)

8. **Mixed Playlists**
   - Audio + video in single playlist
   - Smooth transitions between types
   - User-configurable transition behavior

---

### 🎯 NEXT TO BE ACCOMPLISHED (V1 Priority)

Based on our comprehensive documentation, the next implementation targets are:

**Target 1: UI Skeleton Completion**
- Frameless titlebar with profile icon
- Context-aware search bars (Library 0ms, Discovery  500ms)
- Discovery screen basic layout

**Target 2: Settings Hub**
- Profile tab (OS username extraction, avatar uploader)
- Base Folders management
- API Setup (delegation flow with 150-call tracker)
- New playback settings (Auto-Queue, Video-to-Audio Transition)

**Target 3: Playlist & Queue Foundation**
- SQLite schema implementation
- Playlist creation flows (batch select, context menu)
- Queue management (add, clear, auto-advance)
- Playback footer with three-card preview

**Target 4: Subtitle Engine**
- Fuzzy matching algorithm
- Subtitle Binder Modal UI
- OpenSubtitles API integration
- OSHash computation (Rust)

---

## Summary of Documentation Created

| Document | Status | Purpose |
|----------|--------|---------|
| **PLAYLIST_ARCHITECTURE.md** | ✅ NEW | Complete playlist/queue system specification |
| **UX_JOURNEYS.md** | ✅ EXPANDED | Added Scenarios 7-9 (playlists, queue, subtitles) |
| **forensic_blueprint.md** | ✅ EXPANDED | Added Sections 14-17 (technical implementations) |
| **deep_logic_supplement.md** | ✅ EXPANDED | Section 5 additions + new Section 6 |
| **FLUX_MASTER_GUIDE.md** | ✅ EXPANDED | Added Sections 6-7 (footer + detail panel specs) |
| **SETTINGS_SPEC.md** | ✅ EXPANDED | New playback settings |
| **PROJECT_CHARTER.md** | ✅ UPDATED | This status update |

All documentation is now synchronized and reflects the complete architectural vision for Flux Player V1.

**🟢 ACCOMPLISHED**

* Native Shell (Tauri + SvelteKit).

* Dynamic Island physics, morphing logic, spacing geometry, and exact border aesthetic rules.

* Library Layouts (Grid/List/Detail) and Batch Selection memory logic.

**🔴 NOT ACCOMPLISHED**

* Context-Aware Search Bars (Discovery  vs. Library).

* User Identity UI (Titlebar Avatar, Discovery  Greeting, Avatar Uploader).

* Offline Image Caching & Fallback UI logic.

* Scraper Cascade & Auto-Fetch Subtitle engine.

* API Key Delegation Flow (150-call rotation pool and Setup UI).

* Playback Resume State Memory.

* Sidecars (FFmpeg & `yt-dlp`).

* OpenSubtitles Local Downloader (OSHash).

**🟡 NEXT TO BE ACCOMPLISHED**

* **Target 1:** Construct V1 UI Skeleton (Frameless titlebar with profile icon, Search Bars, Discovery /Grid layout).

* **Target 2:** Build the **Settings Hub** (Profile tab + Base Folders + API Setup).