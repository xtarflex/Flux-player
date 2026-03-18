# Flux: Deep Logic Supplement

These are the "hidden" technical details found during deep logic scans. We MUST implement these to achieve true feature parity.

## 1. Interaction & Events

* **Context-Aware Search Logic**: Instant local search filtering via SQLite (0ms), and debounced (500ms) TMDB online search.

* **Preview Hard Reset**: Explicitly wipes the command queue and resets `currentTime` to 0 when switching media to prevent "sound leaking".

## 2. Dynamic Island Nuances

* **Adaptive Tinting**: The island's solid border and icon colors aren't static; they use `getDominantColor()` from the movie poster or album art. (Strict Rule: There is no ambient outer glow. The tint applies strictly to the crisp 1px border to maintain a premium feel).

* **Offline Red State**: If the network connection drops, the border overrides the adaptive tint and turns sharp red.

## 3. The API Delegation System (TMDB Rate Limit Protection)

* **Local Tracker & Rotation Pool**: The `settings.json` file MUST track a `tmdb_shared_calls_used` integer. The Rust backend automatically rotates between 3 embedded API keys. Every time the app makes a network request to `api.themoviedb.org` using the embedded pool, this integer increments by 1.

* **The Hard Stop**: Inside the global `fetch` wrapper or Axios interceptor, check `if (tmdb_shared_calls_used >= 150 && !user_custom_api_key)`. If true, the request is immediately aborted and a custom window event `flux-require-api-key` is dispatched.

* **The Intercept UI**: Svelte listens for `flux-require-api-key` and immediately renders the "Unlock Infinite Discovery" cheat-sheet modal over the current view.

## 4. Offline Resilience & State Memory

* **Offline Image Asset Caching**: When TMDB returns an image URL for a poster or backdrop, the frontend does *not* link directly to `image.tmdb.org`. Instead, it passes the URL to a Tauri Rust command, which downloads the binary image blob into `%APPDATA%/flux-player/cache/images/` and returns the `asset://` localhost path to the frontend. This guarantees the library remains visually stunning when fully offline.

* **Playback Resume Memory**: During playback, `PlayerEngine.svelte` fires a debounced update to the SQLite database every 10 seconds, logging the `currentTime`. When returning to that file, the Detail Panel reads this value and conditionally displays a "Resume from \[MM:SS\]" button next to the "Play from Beginning" button.

## 5. Subtitles & Streaming Interceptions

* **Local Subtitle Hashing Engine (OSHash)**: Computes an `OSHash` of the local `.mkv`/`.mp4` file to guarantee frame-rate perfect `.srt` syncs from OpenSubtitles.

* **Local Save Execution**: Subtitles downloaded for local files are written directly to the same directory as the video (e.g., `[MovieName].[lang].srt`).

* **Fuzzy Subtitle Binding**: A `flux-bind-subtitle` window event that force-attaches a newly downloaded `.srt` file to the current Video.js player state without requiring a reload.

* **Scraper Cascade System**: Silent failover from VidSrc -> VidLink -> Consumet to ensure streams always launch.

* **Zero-Gravity Clipboard Intercept**: Global window listener for URLs, extracting via `yt-dlp` in the background while the UI remains unblocked.