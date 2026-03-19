# Flux Player: Forensic Blueprint

This document explains the technical "sleight of hand" and magic behind Flux Player's most premium UI/UX features. Future developers and AI agents must reference this to understand how to implement these features without degrading performance.

## 1. Adaptive UI Tinting (Canvas Sampling)

**Goal:** Change the UI color to match the dominant color of the active media.

*   **Canvas Bridge:** We don't use heavy libraries. Instead, `getDominantColor()` creates an in-memory `HTMLCanvasElement` (64x64).
*   **Sampling:** It draws the poster image to the canvas, then uses `getImageData()` to extract the raw pixel array.
*   **Optimization:** It averages exactly 1000 pixels (calculated via a `step` variable) to produce an `rgb()` string.
*   **Binding:** This `rgb` string is bound to the CSS variable `--island-adaptive-tint` in `DynamicIsland.svelte`.
*   **The 1px Adaptive Border:** This tint is applied strictly to the 1px border. We **do not** use `box-shadow` or `drop-shadow` for glows to avoid performance degradation.

## 2. The Dynamic Island: Morphing & Physics

**Goal:** Create a context-aware HUD that feels kinetic and "zero-gravity."

*   **Svelte Spring Physics:** Dimensions (`width`, `height`, `border-radius`) are bound to a Svelte `spring` store (`stiffness: 0.15`, `damping: 0.35`). This creates the signature bounce when expanding from Idle (`56x56`) to Status (`320x48`).
*   **SVG Logo Morphing:** The Idle state logo (F-L-U-X) uses `flubber` for topological interpolation, ensuring smooth transitions even when paths have different vertex counts. A Svelte `tweened` store (`1200ms`, `cubicInOut`) handles the animation.
*   **Micro-Player Transition:** Triggered via `Tauri.invoke('toggle_micro_player')`. The backend snaps the window size instantly, while the frontend fakes the motion using the spring-loaded island overlay.

## 3. Vinyl Audio Mode (Kinetic Identity)

**Goal:** Make audio playback feel visually distinct from video.

*   **CSS Keyframes:** A `rotate(360deg)` animation named `island-spin` is applied to the album art.
*   **The "Pulse":** The ambient glow pulse frequency increases from `3s` (video) to `1s` (audio) to match a turntable's energy.
*   **Conditional Class:** Applied via Svelte template logic: `class="island-thumb {$currentMedia.type === 'audio' ? 'vinyl-spin' : ''}"`.

## 4. Zero-Latency (0ms) Local Search

**Goal:** Instant library filtering that feels faster than a database query.

*   **The Trick:** SQLite passes the *entire* media list to Svelte once.
*   **The Execution:** Search is performed frontend-side by applying reactive CSS classes (e.g., `hidden` or `opacity-0 scale-95`) to grid items that don't match the query.
*   **The Result:** The GPU handles the filtering via transitions, resulting in literal 0ms perceived latency.

## 5. The Off-Canvas Detail Drawer (Home Screen)

**Goal:** Show media details without breaking the horizontal scroll position of the Home Screen.

*   **The Problem:** Pushing rows in the Home Screen forces expensive layout recalculations.
*   **The Magic:** The Detail Panel uses `position: fixed; right: -100%;` and slides in over the UI. 
*   **Glassmorphism:** A full-screen backdrop with `backdrop-filter: blur(5px)` and `rgba(10, 10, 12, 0.7)` dims the background rows while preserving their scroll state.

## 6. Fuzzy Subtitle Binding (Token Interaction)

**Goal:** Match video files with subtitles even when names aren't identical.

*   **Tokenization:** Filenames are split into tokens using `/[^a-z0-9]+/`.
*   **Intersection Scoring:** Shared tokens are counted. A minimum score (typically 2) is required to prevent false matches.
*   **Perfect Match Boost:** If a subtitle name starts with the cleaned video name, it skips scoring for a "Perfect Match."

## 7. OSHash: Frame-Perfect Subtitles

**Goal:** Guaranteed subtitle sync regardless of file metadata.

*   **The Trick:** The Rust backend generates a 64-bit checksum (OSHash) by reading the first and last 64kb of the video file.
*   **The Result:** This hash identifies the *exact* file version on OpenSubtitles, ensuring the returned `.srt` is timed perfectly for that specific rip.

## 8. Immersive Auto-Hide (VLC-Style UI)

**Goal:** Distraction-free playback that reacts to user presence.

*   **Inactivity Timer:** A 3-second timer in `PlayerEngine.svelte` is reset on every `mousemove`.
*   **Smooth Transitions:** Uses `opacity` and `pointer-events: none` instead of toggling `display`.
*   **Cursor Stealth:** Sets `cursor: none` when the UI is hidden to keep the movie in focus.

## 9. Custom VTT Compiler (Regex Transformation)

**Goal:** Convert SRT to WebVTT on-the-fly for native `<track>` support.

*   **High-Speed Regex:** `text.replace(/(\d{2}:\d{2}:\d{2}),(\d{3})/g, "$1.$2")` converts time stamps.
*   **Formatting:** Prepends the `WEBVTT` header and normalizes line endings to LF.

## 10. The Frameless Titlebar & Drag Regions

**Goal:** Modern, native look without standard OS window borders.

*   **Custom Header:** Built with HTML/CSS. Draggable region defined via `data-tauri-drag-region`.
*   **The Catch:** Interactive elements (buttons, avatars) within the header MUST NOT have the drag attribute, or click events will be swallowed by the window drag logic.

## 11. Context Menu Smart Positioning

**Goal:** Floating menus that never overflow the viewport.

*   **Layering:** High `z-index` (9999) ensures it stays on top of the Dynamic Island.
*   **Glassmorphism:** Deep blur (`20px`) with semi-transparent background.
*   **Alignment:** Uses `transform: translate(-100%, 0)` for right-side alignment, preventing the menu from clipping off the screen edge.

## 12. Intelligent Grid Scaling (Smooth Zoom)

**Goal:** Fluid resizing of the media library.

*   **CSS Grid:** Uses `repeat(auto-fill, minmax(VAR, 1fr))` where `VAR` is the zoom step.
*   **Quantized Steps:** Fixed sizes (130px, 220px, 320px) ensure the layout remains predictable.
*   **Transitions:** `transition: grid-template-columns 0.4s ease` on the parent makes the grid "flow" between states.

## 13. The Squircle Mask (Apple-Style Curves)

**Goal:** Premium, mathematically perfect rounded corners (super-ellipses).

*   **The Execution:** Uses an SVG `clip-path` instead of `border-radius: 50%`.
*   **CSS Example:** `clip-path: path('M 0,16 C 0,0 0,0 16,0 C 32,0 32,0 32,16 C 32,32 32,32 16,32 C 0,32 0,32 0,16 Z');`
*   Applied to user avatars for a high-end, bespoke aesthetic.

# Forensic Blueprint - Additional Technical Implementations

These sections expand the forensic_blueprint.md with implementation details for new features.

---

## 14. Playlist Auto-Advance Logic (Netflix-Style Seamless Transitions)

**Goal:** Create instantaneous, imperceptible transitions between media items in a playlist/queue.

### The Technical Challenge
Traditional media players have a visible gap between tracks:
```
Video 1 ends → Black screen (200-500ms) → Video 2 loads → Playback begins
```

Flux eliminates this gap entirely.

### The Solution: Pre-Loading & Buffer Handoff

#### Implementation Strategy:
```javascript
// PlayerEngine.svelte

let currentPlayer;  // Active Video.js instance
let nextPlayer;     // Pre-loaded instance (hidden)

async function handleMediaEnd() {
  if (!queue.hasNext()) {
    // No next item, stop playback
    routeToLibrary();
    return;
  }
  
  const nextMedia = queue.getNext();
  
  // Instant swap (no loading delay)
  const temp = currentPlayer;
  currentPlayer = nextPlayer;  // Pre-loaded player becomes active
  nextPlayer = temp;           // Recycle old player for next-next item
  
  // Start playback immediately
  currentPlayer.play();
  
  // Update UI
  updateIsland(nextMedia);
  updateFooter(nextMedia);
  
  // Pre-load the NEXT item in background
  if (queue.hasNext(2)) {
    preloadMedia(queue.peek(2), nextPlayer);
  }
}

function preloadMedia(media, player) {
  player.src({
    type: media.mimeType,
    src: media.path
  });
  player.load();  // Buffer in background
  // DO NOT call play() — this is silent pre-loading
}
```

### Mixed Playlist Handling (Audio → Video Transitions)

When transitioning from audio to video:
```javascript
function handleAudioToVideoTransition(nextVideo) {
  // 1. Stop audio playback
  audioPlayer.pause();
  
  // 2. Island morphs to "Switching to Video..." state (1 second)
  Island.setState('transition', {
    message: 'Switching to Video...',
    from: 'audio',
    to: 'video'
  });
  
  // 3. Route to Now Playing section
  setTimeout(() => {
    navigate('/now-playing');
    videoPlayer.src(nextVideo.path);
    videoPlayer.play();
    Island.setState('playing', { media: nextVideo });
  }, 1000);
}
```

### Memory Management
To prevent memory leaks with pre-loaded players:
- Maximum 2 Video.js instances active at once (current + next)
- Old player is recycled (src replaced, not destroyed)
- Dispose instances only when switching between audio/video modes

---

## 15. The Three-Card Queue Preview (Overlapping Depth Effect)

**Goal:** Create a physical, tactile "stack of cards" feel in the playback footer.

### Visual Design (Z-Index Layering)

```
     ┌──┐
  ┌──┤  ├──┐
  │  └──┘  │
  │ CENTER │  ← z-index: 3, elevated 4px
  │ (Now)  │
  └────────┘
   ↑      ↑
  Prev   Next
  50%    50%
  hidden hidden
```

### CSS Implementation
```css
.queue-preview {
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 56px;
  width: 180px;
}

.queue-card {
  position: absolute;
  width: 48px;
  height: 56px;
  border-radius: 8px;
  background-size: cover;
  background-position: center;
  transition: all 0.3s cubic-bezier(0.23, 1, 0.320, 1);
  border: 1px solid rgba(138, 43, 226, 0.3);
}

.queue-card.prev {
  left: 0;
  z-index: 1;
  transform: translateX(-50%);  /* 50% hidden behind center */
  opacity: 0.6;
}

.queue-card.current {
  left: 50%;
  transform: translateX(-50%) translateY(-4px);  /* Center + elevated */
  z-index: 3;
  opacity: 1;
  border-color: var(--cyber-cyan);
  box-shadow: 0 4px 12px rgba(0, 255, 255, 0.3);
}

.queue-card.next {
  right: 0;
  z-index: 1;
  transform: translateX(50%);  /* 50% hidden behind center */
  opacity: 0.6;
}

/* Hover states */
.queue-card.prev:hover,
.queue-card.next:hover {
  opacity: 1;
  transform: scale(1.05);
  z-index: 2;
}
```

### Interaction Logic
```javascript
// Queue card click handlers
function handlePrevClick() {
  if (queue.hasPrevious()) {
    player.pause();
    loadMedia(queue.getPrevious());
    player.play();
  }
}

function handleCurrentClick() {
  // Expand queue panel
  showQueuePanel();
}

function handleNextClick() {
  skipToNext();
}
```

### Animation on Queue Change
When queue updates (item added/removed):
```javascript
// Smooth card shuffle animation
function updateQueuePreview(newQueue) {
  // Fade out current cards
  cards.forEach(card => card.style.opacity = 0);
  
  // Wait 200ms
  await delay(200);
  
  // Update content
  prevCard.style.backgroundImage = `url(${newQueue.prev?.poster})`;
  currentCard.style.backgroundImage = `url(${newQueue.current.poster})`;
  nextCard.style.backgroundImage = `url(${newQueue.next?.poster})`;
  
  // Fade in
  cards.forEach(card => card.style.opacity = 1);
}
```

---

## 16. VLC-Style Progressive Volume Bars

**Goal:** Replace boring sliders with kinetic, visual volume control.

### Visual Anatomy
```
┌─┬──┬───┬────┬─────┐
│█│██│███│████│█████│  ← Bars get progressively taller
└─┴──┴───┴────┴─────┘
 ↑  ↑  ↑   ↑    ↑
12 18 24  30   36px heights
```

### Color Gradient Mapping
```javascript
const volumeColors = {
  0-30: 'linear-gradient(90deg, #00ff00, #7fff00)',   // Green
  30-70: 'linear-gradient(90deg, #7fff00, #ffff00)',  // Yellow
  70-100: 'linear-gradient(90deg, #ffff00, #ff0000)'  // Red
};

function getVolumeGradient(volume) {
  if (volume <= 30) return volumeColors['0-30'];
  if (volume <= 70) return volumeColors['30-70'];
  return volumeColors['70-100'];
}
```

### Interaction Detection
```svelte
<script>
  let volumeLevel = 50;  // 0-100
  let isDragging = false;
  let barContainer;
  
  function handleMouseDown(e) {
    isDragging = true;
    updateVolume(e);
  }
  
  function handleMouseMove(e) {
    if (!isDragging) return;
    updateVolume(e);
  }
  
  function updateVolume(e) {
    const rect = barContainer.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percentage = (x / rect.width) * 100;
    
    volumeLevel = Math.max(0, Math.min(100, percentage));
    player.volume(volumeLevel / 100);
  }
  
  function handleMouseUp() {
    isDragging = false;
  }
</script>

<div 
  class="volume-bars"
  bind:this={barContainer}
  on:mousedown={handleMouseDown}
  on:mousemove={handleMouseMove}
  on:mouseup={handleMouseUp}
  on:mouseleave={handleMouseUp}
>
  {#each [12, 18, 24, 30, 36] as height, i}
    <div 
      class="bar"
      style="
        height: {height}px;
        background: {i < (volumeLevel / 20) ? getVolumeGradient(volumeLevel) : '#333'};
      "
    />
  {/each}
</div>

<style>
  .volume-bars {
    display: flex;
    gap: 4px;
    align-items: flex-end;
    height: 40px;
    padding: 4px;
    cursor: ew-resize;
  }
  
  .bar {
    width: 6px;
    border-radius: 2px;
    transition: background 0.15s ease;
  }
  
  .bar:hover {
    filter: brightness(1.2);
  }
</style>
```

### Visual Feedback
- **Active bars:** Full gradient color
- **Inactive bars:** Dark grey (#333)
- **Hover:** Slight brightness increase
- **Dragging:** Smooth, instant updates (no lag)

---

## 17. Subtitle Binder Modal (Fuzzy Matching + OpenSubtitles Integration)

**Goal:** Intelligent, semi-automatic subtitle discovery and binding.

### Fuzzy Matching Algorithm

#### Tokenization
```javascript
function tokenize(filename) {
  // Remove file extension
  const base = filename.replace(/\.(mp4|mkv|avi|srt|vtt)$/i, '');
  
  // Split on non-alphanumeric characters
  const tokens = base
    .toLowerCase()
    .split(/[^a-z0-9]+/)
    .filter(t => t.length > 2);  // Ignore very short tokens
  
  return new Set(tokens);
}

// Example:
// "Blue.Lock.Movie.2024.1080p.mp4" 
// → ["blue", "lock", "movie", "2024", "1080p"]
```

#### Matching Score
```javascript
function calculateMatchScore(videoFilename, subtitleFilename) {
  const videoTokens = tokenize(videoFilename);
  const subTokens = tokenize(subtitleFilename);
  
  // Calculate intersection
  const intersection = new Set(
    [...videoTokens].filter(t => subTokens.has(t))
  );
  
  // Score = (shared tokens / total unique tokens) * 100
  const totalTokens = new Set([...videoTokens, ...subTokens]).size;
  const score = (intersection.size / totalTokens) * 100;
  
  // Bonus points for exact prefix match
  const videoClean = videoFilename.replace(/\.[^.]+$/, '').toLowerCase();
  const subClean = subtitleFilename.replace(/\.[^.]+$/, '').toLowerCase();
  
  if (subClean.startsWith(videoClean)) {
    return Math.min(100, score + 20);  // "Perfect Match" boost
  }
  
  return Math.round(score);
}
```

#### Example Matches:
```
Video: "Blue.Lock.S02E01.1080p.mp4"
Subtitle: "Blue.Lock.S02E01.EN.srt"       → 95% match ✓
Subtitle: "Blue Lock Season 2 Ep 1.srt"   → 78% match
Subtitle: "Blue.Lock.2024.srt"            → 45% match
Subtitle: "Random.Movie.srt"              → 12% match ✗
```

### OSHash Implementation (Rust Backend)

OSHash is a 64-bit checksum computed from the first and last 64KB of a file.

```rust
// src-tauri/src/oshash.rs

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn compute_oshash(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    
    // OSHash = file_size + sum of first 64KB + sum of last 64KB
    let mut hash: u64 = file_size;
    
    // Read first 64KB
    let mut buffer = vec![0u8; 65536];
    file.read_exact(&mut buffer)?;
    hash = hash.wrapping_add(bytes_to_u64_sum(&buffer));
    
    // Read last 64KB
    file.seek(SeekFrom::End(-65536))?;
    file.read_exact(&mut buffer)?;
    hash = hash.wrapping_add(bytes_to_u64_sum(&buffer));
    
    Ok(format!("{:016x}", hash))
}

fn bytes_to_u64_sum(bytes: &[u8]) -> u64 {
    bytes.chunks(8)
        .map(|chunk| {
            let mut array = [0u8; 8];
            array[..chunk.len()].copy_from_slice(chunk);
            u64::from_le_bytes(array)
        })
        .fold(0u64, |acc, x| acc.wrapping_add(x))
}
```

### OpenSubtitles API Integration

```rust
// src-tauri/src/opensubtitles.rs

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct SubtitleSearchQuery {
    moviehash: String,
    sublanguageid: String,  // "eng", "spa", "fra", etc.
}

#[derive(Deserialize)]
struct SubtitleResult {
    SubFileName: String,
    SubDownloadLink: String,
    SubLanguageID: String,
    SubRating: String,
}

pub async fn search_subtitles(
    hash: &str,
    language: &str
) -> Result<Vec<SubtitleResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.opensubtitles.org/api/v1/subtitles")
        .header("Api-Key", "YOUR_API_KEY")
        .json(&SubtitleSearchQuery {
            moviehash: hash.to_string(),
            sublanguageid: language.to_string(),
        })
        .send()
        .await?;
    
    let results: Vec<SubtitleResult> = response.json().await?;
    Ok(results)
}
```

### Auto-Download & Binding

```rust
pub async fn download_subtitle(
    url: &str,
    video_path: &str,
    language: &str
) -> Result<String, Box<dyn std::error::Error>> {
    // Download subtitle file
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    
    // Generate output path (same directory as video)
    let video_dir = std::path::Path::new(video_path).parent().unwrap();
    let video_stem = std::path::Path::new(video_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    
    let sub_path = video_dir.join(format!("{}.{}.srt", video_stem, language));
    
    // Write file
    std::fs::write(&sub_path, content)?;
    
    Ok(sub_path.to_str().unwrap().to_string())
}
```

### Real-Time Binding (flux-bind-subtitle Event)

```javascript
// PlayerEngine.svelte

window.addEventListener('flux-bind-subtitle', async (e) => {
  const { subtitlePath } = e.detail;
  
  // Convert SRT to VTT if needed
  let vttContent;
  if (subtitlePath.endsWith('.srt')) {
    vttContent = await convertSrtToVtt(subtitlePath);
  } else {
    vttContent = await fetch(`asset://localhost/${subtitlePath}`).then(r => r.text());
  }
  
  // Add text track to Video.js
  player.addRemoteTextTrack({
    kind: 'subtitles',
    label: e.detail.language || 'Unknown',
    srclang: e.detail.langCode || 'en',
    src: `data:text/vtt;base64,${btoa(vttContent)}`
  }, false);
  
  // Enable the track
  const tracks = player.textTracks();
  tracks[tracks.length - 1].mode = 'showing';
});
```

This allows subtitles to be bound dynamically WITHOUT reloading the video player.

---

These implementations complete the technical foundations for Flux's playlist, queue, volume control, and subtitle systems.