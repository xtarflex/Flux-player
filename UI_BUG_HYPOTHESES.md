# Dynamic Island & Titlebar UI Bug Hypotheses

Based on a synthetic analysis of the `flux-player` codebase (specifically `DynamicIsland.svelte`, `PlayerEngine.svelte`, and `+layout.svelte`), here are 4 hypotheses that could explain why the UI vanishes for certain movies, and how to stress test them:

### Hypothesis 1: The Adaptive Tint Canvas Crash (SecurityError)
**The Suspect:** `DynamicIsland.svelte` line 39 (`updateAdaptiveTint`)
**The Theory:** When a video plays, the Dynamic Island attempts to dynamically change its border color to match the movie's poster. It does this by drawing the poster image to a hidden HTML `<canvas>` and reading the pixels. If the poster image (perhaps fetched from TMDB or a local directory without the right permissions) triggers a Cross-Origin (CORS) restriction, calling `ctx.getImageData()` will throw a fatal `SecurityError`.
**Why it matches the bug:** If an error throws inside a Svelte 5 `$effect` block, it can completely halt rendering for that component subtree, causing the Island (and potentially the Titlebar if they share a layout context) to vanish.
**How to stress test it:**
- Strip the poster/album art completely from a "broken" movie and see if it plays fine.
- Alternatively, check if the movies that break are fetching external TMDB posters, while working movies use local embedded art.

### Hypothesis 2: Layout Overflow via Extreme Metadata (CSS Flexbox Collapse)
**The Suspect:** `Titlebar.svelte` and `IslandMedia.svelte`
**The Theory:** The Titlebar and Dynamic Island rely on CSS Flexbox and Grid. If a movie's metadata (Title, Artist, or Synopsis) contains an extremely long unbroken string, bizarre Unicode characters, or right-to-left text, it might cause the Flexbox containers to infinitely expand or collapse.
**Why it matches the bug:** If the Titlebar expands beyond `100vw`, it might push the Dynamic Island completely off the screen, making it look like they both disappeared.
**How to stress test it:**
- Take a "working" video file and artificially inject a massive 300-character title with no spaces into its metadata.
- Take a "broken" video file and completely clear its metadata (Title, Artist, Album) using an external tag editor before loading it into Flux.

### Hypothesis 3: Video.js Decoder Choke & Svelte Mouse Event Flooding
**The Suspect:** `+layout.svelte` (`handleTheaterMouseMove`)
**The Theory:** In Theater mode, moving your mouse triggers `handleTheaterMouseMove` which rapidly mutates Svelte state (`footerReveal`, `sidebarReveal`, and `playbackState.isIdle`).
**Why it matches the bug:** If a specific movie uses a very heavy codec (like 4K HEVC/x265) that the embedded Chromium webview struggles to decode, the CPU/GPU thread gets maxed out. When you move the mouse, hundreds of Svelte state updates fire simultaneously. The event loop gets so choked that Svelte drops the render frames for the UI overlays, leaving them stuck in the hidden `ui-idle` state. Scrolling specifically to the sidebar might trigger a CSS-hardware-accelerated transition that forces a repaint, temporarily "fixing" it.
**How to stress test it:**
- Check the exact codec of the working vs broken movies (e.g., using VLC or MediaInfo). Are the broken ones HEVC/x265 and the working ones H.264/AVC?
- When opening a broken movie, take your hand completely off the mouse for 5 seconds. Let it load without triggering any `mousemove` events, then slowly move it to the bottom.

### Hypothesis 4: The `hasSavedFinished` Race Condition
**The Suspect:** `PlayerEngine.svelte`
**The Theory:** There is a flag called `hasSavedFinished` that prevents progress saves if the movie has ended. If a specific movie has a corrupted duration (e.g., the file header says it's 0 seconds or `NaN` long initially), the player logic might immediately assume it has finished playing.
**Why it matches the bug:** If the app thinks the movie is over the millisecond it starts, it might trigger the `ended` event loop, which aggressively attempts to auto-advance the queue or teardown the player state, putting the UI in a ghost state where it's playing a video but the global store thinks it's idle/finished.
**How to stress test it:**
- Check the duration parsing of the broken files. Do they show up with the correct time length in the library grid before you click play?
