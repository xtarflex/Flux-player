# Flux Player: UX & User Journeys

This document outlines the step-by-step user experience (UX) flows for key interactions within Flux Player, ensuring the app always feels kinetic, intuitive, and premium.

## Scenario 1: The First-Time Launch (Onboarding)

**Goal:** Transform an empty, newly-installed app into a fully populated cinematic dashboard with zero friction, prioritizing offline reliability.

### Phase 1: The Zero-State Welcome

1. **The Launch:** The user opens Flux for the first time. The custom frameless window appears with the `Cyber Dark` theme. The app defaults to the **Library View**.

2. **The Greeting:** The UI reads the local OS via Tauri and displays a massive, `Syncopate`-font greeting: "Welcome to Flux, [OS Username].".

3. **The API Heads-Up:** A small, elegant toast notification appears at the bottom: *"Flux is using a shared metadata key. You have 150 free cinematic fetches remaining before you'll need to link your own."*

4. **The Island:** The Dynamic Island sits at the top center in its `56x56` Idle state. It smoothly morphs between the F-L-U-X SVG paths, featuring a slight delay and a reverse sequence animation. **(Aesthetic Rule: There is no ambient glow. Crisp 1px border only).**

### Phase 2: Automatic Indexing & The Empty State

1. **The Auto-Scan:** Upon launch, Flux automatically triggers a background scan of the system's default "Videos" and "Music" folders. If media is found, the app bypasses the empty state and jumps directly to Phase 4.

2. **The Empty Dashboard:** If the default OS folders are completely empty, the Library displays a beautifully designed, glassmorphism "Drop Zone" filling the center of the grid.

3. **The Prompt:** It reads: *"Your theater is empty. Drag and drop media here, or add a Base Folder to begin."*

### Phase 3: Adding Media (The Sidebar Action)

1. **Folder Selection:** To add a custom drive (like an external HDD), the user clicks the dedicated **"Add Folder"** button conveniently located directly in the **Sidebar**.

2. **The Intercept:** The native OS file picker opens. The user selects their directory.

3. **Island Morph (Status State):** The Dynamic Island immediately proves its utility. It expands from the tiny pill into the `320x48` Status state. A cyan dashed border appears, and text reads "Scanning local files... 240 found".

### Phase 4: The Cinematic Population (The "Wow" Moment)

1. **The Grid Fills:** As the local SQLite database logs the newly discovered files, the Library grid begins to populate instantly with placeholder `flux.logo` cards.

2. **The TMDB Magic (If Online):** In the background, Flux uses the embedded developer API key pool (deducting from the 150 free calls). One by one, the placeholder logos cross-fade into high-resolution cinematic movie posters and backdrops.

3. **Offline Image Sync:** As the URLs resolve, the Rust backend quietly downloads the `.jpg` posters to the local `%APPDATA%` cache so they remain visible forever, even if the PC goes offline.

4. **Ready State:** The onboarding is complete. The user is now fully immersed in their populated Library.

## Scenario 2: The "API Key Delegation" Flow

**Goal:** Seamlessly transition the user from the embedded developer key pool to their own personal TMDB key without causing frustration or confusion.

### Phase 1: The Limit Reached

1. **The Trigger:** As the user scans their library or searches for movies, the background local counter hits 150 API calls.

2. **The Intercept:** The next attempt to fetch metadata is paused. A sleek, glassmorphic modal overlay appears: **"Unlock Infinite Discovery."**

3. **The Message:** *"Your 150 free metadata fetches have been used! To keep Flux fast and free for everyone, please link your own personal (and free!) TMDB API key."*

### Phase 2: The Cheat Sheet UI

1. **The Guide:** Inside the modal, Flux provides a direct, clickable link to `https://www.themoviedb.org/settings/api`.

2. **The Cheat Sheet:** Below the link, Flux provides a literal "Copy-Paste" cheat sheet for the TMDB registration form.

   * *Type:* "Developer"

   * *Application Name:* [Copy Button] `Flux Player Local`

   * *Application URL:* [Copy Button] `https://github.com/flux-player`

   * *Application Summary:* [Copy Button] `A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection.`

3. **The Input:** A clear, glowing text field labeled **"Paste Your API Read Access Token (v4) Here"**.

4. **The Validation:** The user pastes the key. The app instantly sends a test ping to TMDB. If valid, the field turns green, the modal dismisses itself, and the paused scan/search instantly resumes.

## Scenario 3: The "Discovery to Stream" & Trailer Flow

**Goal:** Allow users to seamlessly explore online media via search or curated rows, watch high-quality trailers, and launch external streams flawlessly.

### Phase 1: The Unified Search Experience

1. **Context-Aware Input:** Both the Library and Home screen feature a sleek, expanding glassmorphic search bar in their respective header areas.

2. **Library Search (Local):** Typing in the Library triggers an instant, millisecond filter (0ms) of the local SQLite database.

3. **Home Search (Online):** Typing in the Home screen triggers a debounce function (500ms) that pings the TMDB Search API.

### Phase 2: The Off-Canvas Detail Drawer

1. **The Click:** The user clicks a movie poster from the Home Screen's "Trending Now" row or their TMDB Search Results.

2. **The Overlay:** The Home Screen dims with a dark glassmorphism overlay (`backdrop-filter: blur(5px)`).

3. **The Slide-In:** The `<DetailPanel />` slides in smoothly from the right edge of the screen.

### Phase 3: The Cinematic Trailer Lightbox

1. **The Action:** Inside the Detail Panel, the user clicks the "Watch Trailer" button.

2. **The Extraction:** The Dynamic Island briefly displays an "Extracting..." status as the `yt-dlp` sidecar grabs the raw `.mp4` from the TMDB-provided YouTube key.

3. **The Lightbox:** A borderless, centralized video modal fades into the center of the screen, playing the ad-free trailer natively over the dimmed background.

### Phase 4: Launching the Full Stream & Subtitle Fetch

1. **The Smart Button:** Because the app recognizes this media is not local, the button reads **"Stream Online"**.

2. **The Fetch:** The user clicks "Stream Online". Flux's backend initiates a **Scraper Cascade** to extract the best available stream. Simultaneously, it scrapes available `.srt` or `.vtt` tracks.

3. **The Theater:** Once the raw `.m3u8` link and subtitles are secured, the UI morphs entirely into the Spatial Theater.

## Scenario 4: The "Direct Link" Flow (Clipboard Intercept)

**Goal:** Allow users to bypass standard web browsers and watch YouTube, Vimeo, or raw `.m3u8` links in a premium, hardware-accelerated environment.

### Phase 1: The Zero-Click Paste

1. **The Action:** The user copies a video URL. While Flux is the active window (and not typing), they press `Ctrl + V`.

2. **The Intercept:** A global window listener catches the paste event. The UI **does not** immediately switch to the player.

3. **The Island Morph:** The Dynamic Island expands into Status state (`320x48`). A neon cyan indicator reads *"Extracting Stream..."*.

### Phase 2: The Seamless Transition

1. **The Handshake:** In the background, the `yt-dlp` sidecar resolves the URL.

2. **The Morph:** The millisecond the stream is ready, the Spatial Theater takes over the full window.

3. **Error Handling:** If invalid, the Island's crisp border turns red, displays *"Invalid Media Link"*, and shrinks back to Idle.

## Scenario 5: The "Audio DJ" Flow

**Goal:** Create a kinetic, non-intrusive audio experience that allows users to listen to music while managing their library.

### Phase 1: Instant Playback

1. **The Action:** The user single-clicks an audio file in the Library grid.

2. **The Split UX:** The user remains exactly where they are in the Library.

### Phase 2: The Micro-Player Transformation

1. **The Island Morph:** The Dynamic Island expands into its Audio Playing state (`240x56`).

2. **Kinetic Identity:** The standard track icon is replaced by the album art, masked in a circle. It spins continuously to mimic a vinyl record.

3. **Visualizer & Border:** A bouncing-line EQ visualizer appears. **The Island's solid 1px border adaptively changes color to match the dominant tint of the album art.**

## Scenario 6: The Local Subtitle Downloader

**Goal:** Allow users to instantly discover, download, and bind subtitles for local media files directly within the app.

### Phase 1: The Missing Subtitle State

1. **The Context:** The user clicks a local movie. The Detail Panel slides in.

2. **The Indicator:** Next to the "Play Local" button, the Subtitle icon has a subtle slash through it (or says "Subs: None").

### Phase 2: The In-App Discovery

1. **The Click:** The user clicks the Subtitle icon.

2. **The Modal:** A glassmorphic modal pops up: "Searching for Subtitles...".

3. **The Results:** Flux's backend pings OpenSubtitles (using the video's OSHash byte signature). A clean list of available subtitles appears.

### Phase 3: The Download & Bind

1. **The Selection:** The user clicks their preferred subtitle track.

2. **The Save:** Flux downloads the `.srt` file and seamlessly saves it directly into the local hard drive folder, perfectly renamed to match the video.

3. **The Bind:** The UI dispatches the `flux-bind-subtitle` event. The video launches with the subtitles already active.

# Additional UX Scenarios for Flux Player

These scenarios expand the UX_JOURNEYS.md document with playlist, queue, and subtitle binder workflows.

---

## Scenario 7: The Playlist Experience

**Goal:** Allow users to organize their media into curated collections for seamless playback sessions.

### Phase 1: Creating a Playlist (Batch Select Method)

1. **The Entry:** User is browsing the Library in Grid View, looking at their MCU movies collection.

2. **Batch Select Mode:** User right-clicks on "Iron Man" → Batch Select Mode activates. Checkboxes appear on all cards.

3. **The Selection:** User clicks 6 movies:
   - Iron Man ✓
   - The Incredible Hulk ✓
   - Iron Man 2 ✓
   - Thor ✓
   - Captain America ✓
   - The Avengers ✓

4. **The Creation:** User clicks **[Create Playlist]** button in the top action bar.

5. **The Modal:** A glassmorphic modal slides in:
   ```
   Create New Playlist
   ───────────────────
   Name: [MCU Phase 1_________]
   Type: ● Movies  ○ Music  ○ Mixed
   Description: [Chronological MCU Phase 1 marathon]
   
   6 items will be added
   
   [Cancel]  [Create Playlist]
   ```

6. **The Confirmation:** User clicks **[Create Playlist]** → Batch Select Mode exits → Success toast: "MCU Phase 1 created with 6 items"

7. **The Visibility:** Playlist appears in sidebar (as one of the 5 most recent) and in the Playlists Screen.

---

### Phase 2: Adding to Existing Playlist (Context Menu)

1. **The Discovery:** User finds a new MCU movie they forgot: "Ant-Man."

2. **The Action:** User clicks the **[⋮]** menu button on the "Ant-Man" card.

3. **The Menu:** Context menu appears:
   ```
   Play Now
   Add to Queue
   ───────────────
   Select Item
   Add to Playlist ──→  [● MCU Phase 1]
   Add to Favorite      [○ 90s Action  ]
   Get Subtitles        [─────────────]
   Details              [+ New Playlist]
   ```

4. **The Addition:** User clicks "MCU Phase 1" → Item instantly added → Toast: "Added to MCU Phase 1"

---

### Phase 3: Playing a Playlist

1. **The Navigation:** User clicks "MCU Phase 1" in the sidebar.

2. **The View:** Playlist Detail View opens:
   ```
   ← Back          MCU PHASE 1          [Edit]
   
   [▶ Play All]  [🔀 Shuffle Play]  [+ Add Items]
   
   1. Iron Man (2008)                   ━━●━━ 45:23
   2. The Incredible Hulk (2008)        ━━━━━ 0:00
   3. Iron Man 2 (2010)                 ━━━━━ 0:00
   4. Thor (2011)                       ━━━━━ 0:00
   5. Captain America (2011)            ━━━━━ 0:00
   6. The Avengers (2012)               ━━━━━ 0:00
   
   Total: 12h 34m • 6 items • Created: Mar 24
   ```

3. **The Playback:** User clicks **[▶ Play All]**.

4. **The Queue Population:** All 6 movies populate the queue in order.

5. **The Experience:**
   - Iron Man begins playing in Now Playing section
   - Dynamic Island morphs to Playing state
   - Playback Footer shows three-card queue preview (Iron Man center, Hulk right)
   - Iron Man finishes → Hulk instantly begins (Netflix-style seamless transition)
   - User can navigate to Library while watching → Video enters in-app mini player mode
   - Playlist continues auto-advancing through all 6 movies

---

## Scenario 8: The Queue System

**Goal:** Provide temporary, session-based "play next" functionality without creating permanent playlists.

### Phase 1: Manual Queue Building

1. **The Context:** User is watching "Blue Lock Episode 1" in Now Playing section.

2. **The Addition:** User navigates to Library (video enters mini player) → Right-clicks "Episode 2" → **"Add to Queue"**

3. **The Confirmation:** Toast appears: "Added to queue"

4. **The Visual Update:** Playback Footer's three-card preview updates:
   - Left: (empty, no previous)
   - Center: Episode 1 (currently playing)
   - Right: Episode 2 (next in queue)

5. **The Expansion:** User clicks the three-card stack → Queue Panel expands:
   ```
   CURRENT QUEUE          [Clear All]
   ───────────────────────────────────
   Now Playing:
   🎬 Blue Lock Ep 1       ━━●━━ 12:34
   
   Up Next (1 item):
   1. 🎬 Blue Lock Ep 2    ━━━━━ 24:15
   
   [View Full Queue ──→]
   ```

6. **The Auto-Play:** Episode 1 finishes → Episode 2 instantly begins → User didn't have to click anything.

---

### Phase 2: Queue from Playlist

1. **The Action:** User clicks "Anime Binge" playlist in sidebar → Clicks **[▶ Play All]**

2. **The Population:** All 12 episodes populate the queue.

3. **The Indicator:** Queue Panel shows:
   ```
   Up Next (11 items):
   1. Episode 2
   2. Episode 3
   ...
   
   From Playlist: "Anime Binge"
   ```

4. **The Flexibility:** User can still manually add items:
   - Right-clicks "Guardians of the Galaxy" → "Add to Queue"
   - Movie appends to end of queue (after Episode 12)
   - Queue now contains: 12 episodes + 1 movie

---

### Phase 3: Clearing the Queue

1. **The Decision:** User decides they want to stop the auto-play marathon.

2. **The Action:** User clicks **[Clear All]** in Queue Panel.

3. **The Warning:**
   ```
   Clear Queue?
   ────────────────────────
   This will remove 11 items from
   the playback queue.
   
   This action cannot be undone.
   
   [Cancel]  [Clear Queue]
   ```

4. **The Result:** User clicks **[Clear Queue]** → Queue empties → Currently playing media continues, but nothing plays after it finishes.

---

## Scenario 9: The Enhanced Subtitle Workflow

**Goal:** Seamlessly bind subtitles to videos through fuzzy matching, manual selection, and online search.

### Phase 1: Auto-Detection (Zero Friction)

1. **The Scan:** User adds a folder containing:
   ```
   Blue.Lock.Movie.2024.1080p.mp4
   Blue.Lock.Movie.2024.EN.srt
   Blue.Lock.Movie.2024.ES.srt
   ```

2. **The Magic:** Flux's fuzzy matching instantly pairs the `.srt` files with the `.mp4` (95% match score).

3. **The Result:** When user clicks the movie in Library, Detail Panel shows:
   ```
   Subtitle: Blue Lock Movie 2024 EN   [Change]
   ```

4. **Seamless Playback:** User clicks **[▶ Play Movie]** → Subtitles are already active, perfectly synced.

---

### Phase 2: Manual Binding (Subtitle Binder Modal)

1. **The Problem:** User has a movie where auto-detection failed or chose the wrong subtitle.

2. **The Action:** User clicks **[Change]** button in Detail Panel.

3. **The Modal:** Subtitle Binder Modal opens:
   ```
   SUBTITLE BINDER                     [✕ Close]
   ──────────────────────────────────────────────
   Movie: Blue Lock The Movie - Episode Nagi
   
   LOCAL SUBTITLES (Auto-Detected)
   ○ Blue Lock S1-S2--[EN].srt       [Match: 85%]
   ● Blue Lock Episode Nagi [EN].srt [Match: 95%] ✓
   ○ Blue.Lock.2024.1080p.srt        [Match: 70%]
   
   ───────────────────────────────────────────────
   
   SEARCH OPENSUBTITLES
   [Search Online...────────]  [🔍 Search]
   
   [Cancel]  [Apply]
   ```

4. **The Selection:** User sees the second option is already selected (95% match) → Clicks **[Apply]**

5. **The Binding:** Modal closes → Detail Panel updates: "Subtitle: Blue Lock Episode Nagi EN"

---

### Phase 3: Online Search (Missing Subtitles)

1. **The Problem:** User's video has NO matching subtitles in the folder.

2. **The Discovery:** Detail Panel shows: "Subtitle: None   [Change]"

3. **The Search:** User clicks **[Change]** → Subtitle Binder Modal opens → User clicks **[🔍 Search]** in the OpenSubtitles section.

4. **The Query:** Flux's Rust backend:
   - Computes OSHash of the video file
   - Queries OpenSubtitles API
   - Returns exact matches for this file version

5. **The Results:**
   ```
   Results (3 found):
   ○ Blue Lock Movie [EN] (Official)   [⬇ Download]
   ○ Blue Lock Movie [ES] (Fan Sub)    [⬇ Download]
   ○ Blue Lock Movie [FR] (Official)   [⬇ Download]
   ```

6. **The Download:** User clicks **[⬇ Download]** on the English option.

7. **The Magic:**
   - Subtitle downloads to the same folder as the video
   - Renamed to match: `Blue.Lock.Movie.2024.EN.srt`
   - Automatically bound to the video
   - Modal closes → Ready to play

---

### Phase 4: Context Menu Shortcut

1. **The Shortcut:** User right-clicks a video card in Library → Clicks **"Get Subtitles"** in context menu.

2. **The Direct Access:** Subtitle Binder Modal opens instantly (same as clicking [Change] in Detail Panel).

3. **The Efficiency:** User can search and download subtitles without opening the Detail Panel first.

---

### Phase 5: Subtitle Sync Adjustment (In-Player)

1. **The Problem:** Subtitle is bound but timing is off by 500ms.

2. **The Discovery:** User starts playback → Notices subtitles appear too early.

3. **The Fix:** User clicks **[⚙️ Settings]** gear in playback controls → Mini settings panel appears:
   ```
   Playback Settings
   ─────────────────────────
   Quality: ● 1080p
   
   Playback Speed: ─────●──── 1.0x
   
   Subtitle Sync:
   [-500ms] [-100ms] [Reset]
   [+100ms] [+500ms]
   Current Offset: +0ms
   ```

4. **The Adjustment:** User clicks **[+500ms]** twice → Offset now +1000ms → Subtitles sync perfectly.

5. **The Persistence:** Flux saves this offset to SQLite for THIS specific video → Next time user plays it, offset auto-applies.

---

These scenarios complete the core UX flows for playlists, queue management, and subtitle workflows in Flux Player.