# Prompt for Backend Developer: Flux Player Infrastructure

## Task: Initialize SQL Plugin, Store Plugin, and API Delegation Logic

### Objective
Install necessary Tauri plugins and implement the backend foundations for the Flux Media Scanner and TMDB API Delegation system as defined in the `deep_logic_supplement.md`.

---

### Step 1: Install Plugins
Perform the following installations across the Rust and JavaScript environments:
1.  **tauri-plugin-sql**: Install with `sqlite` features enabled (Required for media/playlists).
2.  **tauri-plugin-store**: (Required for settings/API call counter).

---

### Step 2: Initialize Plugins in `lib.rs` (or `main.rs`)
1.  Initialize the **SQL Builder** and the **Store Builder**.
2.  Set up the **SQLite Migration** to create the following three tables exactly as defined in Section 6 of `deep_logic_supplement.md`:
    - `media` (path, title, year, poster_path, backdrop_path, duration, etc.)
    - `playlists` (id, name, description, etc.)
    - `playlist_items` (playlist_id, media_path, etc.)

---

### Step 3: Implement TMDB API Delegation (Section 3)
1.  Create a Rust command or state that manages the **Rotation Pool** of 3 embedded API keys.
2.  Implement a counter for `tmdb_shared_calls_used` stored in `settings.json`.
3.  **The "Hard Stop" Logic:** Before every TMDB network request, check if the counter exceeds **150** and if a custom user key is absent. If so, abort and emit the `flux-require-api-key` window event.

---

### Step 4: Asset Interception & Offline Caching (Section 4)
1.  Ensure that when metadata is fetched, the Rust side downloads the images to `%APPDATA%/flux-player/cache/images/` and returns the local `asset://` path to the Svelte frontend. This is critical for offline resilience.

---

### Deliverables
- Functional SQLite database with defined schema.
- Secure settings store for API keys and call counts.
- Working API key rotation and call limit events.
- Documented Rust commands for querying media and playlists.
