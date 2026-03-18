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

* **The Home Screen (Dashboard):** A Netflix-style cinematic landing page.

  * **Online Search:** A glassmorphic search bar that queries the TMDB API (500ms debounce) to discover movies/shows not currently downloaded.

  * **Personalized Greeting & Curated Rows:** Dynamic user greetings and horizontally scrolling rows (Just Released, Trending, Upcoming).

* **The Library:** A unified view aggregating multiple "Base Folders."

  * **Local Search:** An instant, zero-latency (0ms) search bar querying the local SQLite database.

  * **Filter & Sort:** Robust Status Filters (Watched/Not Watched), Sort Logic, and Grid/List view modes.

* **The Detail Panel (Context-Aware Split View):**

  * Acts as a docked panel in the Library, and an "Off-Canvas Drawer" on the Home screen.

  * *Streaming Intelligence:* Dynamically morphs to offer "Stream Online" when media is not stored locally.

  * *Resume State:* Detects previously watched media and offers a "Resume" button.

* **Smart Interaction & Batch Selection:** Single-click audio, double-click video, and right-click batch selection.

* **The Settings Hub:** Control center featuring a dedicated **My Profile** tab, Base Folders, FFmpeg/GPU toggles, UI theming, and an Auto-Fetch Subtitles toggle.

## 4. Project Scope & Expanded Version Roadmap

### Version 1 (V1): The MVP (Minimum Viable Player)

* **UI/UX Skeleton:** Frameless titlebar, floating Dynamic Island, Home Screen greeting, and CSS Grid layout.

* **Search & Filters:** Instant local search bar for the Library. Primary filters and A-Z sorting.

* **Identity & Settings:** Basic "My Profile" setup, Base Folder configuration, and the API Delegation On-Ramp (150 free calls).

* **Playback:** Video.js core with local file access, playback state memory (resume watching), and Flux logo fallbacks.

### Version 2 (V2): The Power User

* **Cinematic Home & Discovery:** TMDB online search bar, Trending rows, and the Off-Canvas Detail Drawer.

* **Offline Image Caching:** Rust backend seamlessly downloads and serves TMDB poster/backdrop assets locally.

* **Streaming Pathways (Scraper Cascade):** Integration with redundant APIs (VidSrc, VidLink, Consumet) to guarantee successful "Click and Stream" movie scraping. Combined with a background subtitle fetcher and clipboard interception for `yt-dlp` extraction via the Dynamic Island.

* **The Format Breaker:** FFmpeg sidecar for `.mkv` and `.avi` transcoding.

* **Advanced Library Brain:** SQLite integration (tracking Watched status, generating User Stats, and caching metadata).

### Version 3 (V3): The Ultimate

* **The Audio Forge:** FFT BPM Analyzer, RMS Smart Crossfader, and weighted Grandmaster Shuffle logic.

* **Visual Polish:** Interactive liquid visualizer engines for the Island.

* **Local Ecosystem:** Local network casting (Chromecast/DLNA) and in-app download management.

## 5. Execution Map (Status)

**🟢 ACCOMPLISHED**

* Native Shell (Tauri + SvelteKit).

* Dynamic Island physics, morphing logic, spacing geometry, and exact border aesthetic rules.

* Library Layouts (Grid/List/Detail) and Batch Selection memory logic.

**🔴 NOT ACCOMPLISHED**

* Context-Aware Search Bars (Home vs. Library).

* User Identity UI (Titlebar Avatar, Home Greeting, Avatar Uploader).

* Offline Image Caching & Fallback UI logic.

* Scraper Cascade & Auto-Fetch Subtitle engine.

* API Key Delegation Flow (150-call rotation pool and Setup UI).

* Playback Resume State Memory.

* Sidecars (FFmpeg & `yt-dlp`).

* OpenSubtitles Local Downloader (OSHash).

**🟡 NEXT TO BE ACCOMPLISHED**

* **Target 1:** Construct V1 UI Skeleton (Frameless titlebar with profile icon, Search Bars, Home/Grid layout).

* **Target 2:** Build the **Settings Hub** (Profile tab + Base Folders + API Setup).