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

- **Offline Image Asset Caching**: When TMDB returns metadata, the payload includes both a `poster_path` (vertical) and a `backdrop_path` (horizontal 16:9). The Rust backend MUST intercept both URLs, download the binary image blobs into `%APPDATA%/flux-player/cache/images/posters/` and `/backdrops/` respectively, and return the local `asset://` paths to the Svelte frontend. This guarantees the Discovery screen and Detail Panel backgrounds remain visually stunning when fully offline.

* **Playback Resume Memory**: During playback, `PlayerEngine.svelte` fires a debounced update to the SQLite database every 10 seconds, logging the `currentTime`. When returning to that file, the Detail Panel reads this value and conditionally displays a "Resume from \[MM:SS\]" button next to the "Play from Beginning" button.

## 5. Subtitles & Streaming Interceptions

* **Local Subtitle Hashing Engine (OSHash)**: Computes an `OSHash` of the local `.mkv`/`.mp4` file to guarantee frame-rate perfect `.srt` syncs from OpenSubtitles.

* **Local Save Execution**: Subtitles downloaded for local files are written directly to the same directory as the video (e.g., `[MovieName].[lang].srt`).

* **Fuzzy Subtitle Binding**: A `flux-bind-subtitle` window event that force-attaches a newly downloaded `.srt` file to the current Video.js player state without requiring a reload.

* **Scraper Cascade System**: Silent failover from VidSrc -> VidLink -> Consumet to ensure streams always launch.

* **Zero-Gravity Clipboard Intercept**: Global window listener for URLs, extracting via `yt-dlp` in the background while the UI remains unblocked.

# Deep Logic Supplement - Additional Technical Details

These additions expand Section 5 (Subtitles) and add a new Section 6 (Playlists & Queue Engine).

---

## Section 5 Additions: Subtitle System Enhancements

### Subtitle Binder UI (Modal Structure)

**Modal Trigger:** User clicks **[Change]** button next to "Subtitle: [filename]" in Detail Panel

**Modal Layout:**
```
┌─────────────────────────────────────────────────────────┐
│  SUBTITLE BINDER                              [✕ Close] │
├─────────────────────────────────────────────────────────┤
│  Movie: [Current Video Title]                          │
│  ─────────────────────────────────────────────────────  │
│                                                         │
│  LOCAL SUBTITLES (Auto-Detected)                       │
│  ○ subtitle_option_1.srt        [Match: 85%]           │
│  ● subtitle_option_2.srt        [Match: 95%] ✓         │
│  ○ subtitle_option_3.srt        [Match: 70%]           │
│                                                         │
│  ─────────────────────────────────────────────────────  │
│                                                         │
│  SEARCH OPENSUBTITLES                                   │
│  [Enter movie name or leave blank...] [🔍 Search]      │
│                                                         │
│  Results (0 found):                                     │
│  [No results yet - click Search to query]              │
│                                                         │
│                                 [Cancel]  [Apply]       │
└─────────────────────────────────────────────────────────┘
```

### Match Percentage Calculation
Fuzzy matching scores are displayed to help users identify the best subtitle:
- **95-100%:** Perfect Match (green indicator)
- **80-94%:** Strong Match (cyan indicator)
- **60-79%:** Weak Match (yellow indicator)
- **Below 60%:** Poor Match (grey indicator, likely incorrect)

### Manual OpenSubtitles Querying
When user clicks **[🔍 Search]**:
1. System computes OSHash of video file
2. Queries OpenSubtitles API using hash + movie name (if provided)
3. Returns exact matches for this file version
4. Results show download buttons
5. Downloaded `.srt` saves to video's directory with matching filename

### Context Menu Integration
Added **"Get Subtitles"** option to media card context menu:
```
[⋮] Menu:
  Play Now
  Add to Queue
  ───────────
  Select Item
  Add to Playlist ──→
  Add to Favorite
  Get Subtitles       ← Opens Subtitle Binder Modal
  Remove from Library
  Details
```

---

## Section 6 (NEW): Playlist & Queue Engine

### Playlist Data Structure (SQLite)

#### Playlists Table
```sql
CREATE TABLE playlists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    type TEXT CHECK(type IN ('audio', 'video', 'mixed')),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    is_smart BOOLEAN DEFAULT 0,
    smart_criteria TEXT,
    cover_image TEXT
);
```

#### Playlist Items Table
```sql
CREATE TABLE playlist_items (
    playlist_id TEXT NOT NULL,
    media_path TEXT NOT NULL,
    position INTEGER NOT NULL,
    added_at INTEGER NOT NULL,
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (media_path) REFERENCES media(path) ON DELETE CASCADE,
    PRIMARY KEY (playlist_id, media_path)
);
```

### Queue Storage (Session-Based)
Queue is **NOT** stored in SQLite — it exists only in memory during the app session:

```javascript
// Queue stored as JavaScript array in memory
const queue = {
  items: [],              // Array of media objects
  currentIndex: 0,        // Currently playing item
  sourcePlaylist: null,   // UUID of source playlist (if any)
  
  add(media) {
    this.items.push(media);
  },
  
  clear() {
    this.items = [];
    this.currentIndex = 0;
    this.sourcePlaylist = null;
  },
  
  getNext() {
    if (this.currentIndex < this.items.length - 1) {
      this.currentIndex++;
      return this.items[this.currentIndex];
    }
    return null;
  }
};
```

### Auto-Advance Trigger Logic

```javascript
// PlayerEngine.svelte

player.on('ended', async () => {
  // Check repeat mode first
  if (repeatMode === 'one') {
    player.currentTime(0);
    player.play();
    return;
  }
  
  // Check if queue has next item
  const nextMedia = queue.getNext();
  
  if (nextMedia) {
    // Auto-advance to next item
    await loadMedia(nextMedia);
    player.play();
    updateIsland(nextMedia);
  } else {
    // End of queue
    if (repeatMode === 'playlist' && queue.sourcePlaylist) {
      // Loop back to start
      queue.currentIndex = 0;
      await loadMedia(queue.items[0]);
      player.play();
    } else {
      // Playback complete
      routeToLibrary();
      Island.setState('idle');
    }
  }
});
```

### Library Queue Behavior (Settings-Controlled)

**Settings → Playback → Auto-Queue Library Items**

Three modes:

#### Mode 1: Never (Default)
Single-item playback only.
```javascript
// When user plays media from Library
function playFromLibrary(media) {
  queue.clear();
  queue.add(media);
  loadMedia(media);
  player.play();
}
// When media ends, queue is empty → playback stops
```

#### Mode 2: Smart
Auto-queue only for series/albums.
```javascript
function playFromLibrary(media) {
  queue.clear();
  
  // Check if media is part of a series or album
  if (isSeriesEpisode(media) || isAlbumTrack(media)) {
    // Load all related items
    const relatedItems = getRelatedMedia(media);
    queue.items = relatedItems;
    queue.currentIndex = relatedItems.indexOf(media);
  } else {
    // Single item
    queue.add(media);
  }
  
  loadMedia(media);
  player.play();
}
```

#### Mode 3: Always
Any Library playback creates queue from filtered view.
```javascript
function playFromLibrary(media, currentFilteredList) {
  queue.clear();
  
  // Find clicked item's position in filtered list
  const clickedIndex = currentFilteredList.indexOf(media);
  
  // Queue all items AFTER the clicked one
  queue.items = currentFilteredList.slice(clickedIndex);
  queue.currentIndex = 0;
  
  loadMedia(media);
  player.play();
}
```

### Playback Mode Handling

**Sequential:** Default, plays items in order
**Shuffle:** Randomizes queue order (Fisher-Yates shuffle)
**Repeat Playlist:** Loops queue indefinitely
**Repeat One:** Loops current track indefinitely

```javascript
let repeatMode = 'off';  // 'off', 'playlist', 'one'
let shuffleMode = false;

function toggleRepeat() {
  const modes = ['off', 'playlist', 'one'];
  const currentIndex = modes.indexOf(repeatMode);
  repeatMode = modes[(currentIndex + 1) % modes.length];
  updateRepeatIcon();
}

function toggleShuffle() {
  shuffleMode = !shuffleMode;
  
  if (shuffleMode) {
    // Shuffle queue (preserve current item)
    const current = queue.items[queue.currentIndex];
    const remaining = queue.items.slice(queue.currentIndex + 1);
    
    // Fisher-Yates shuffle
    for (let i = remaining.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [remaining[i], remaining[j]] = [remaining[j], remaining[i]];
    }
    
    queue.items = [current, ...remaining];
    queue.currentIndex = 0;
  }
  
  updateShuffleIcon();
}
```

### File System Sync (Playlist Maintenance)

When files are renamed/moved on disk:

```rust
// src-tauri/src/file_watcher.rs

use notify::{Watcher, RecursiveMode, watcher};

pub fn watch_library_folders(paths: Vec<String>) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2)).unwrap();
    
    for path in paths {
        watcher.watch(path, RecursiveMode::Recursive).unwrap();
    }
    
    loop {
        match rx.recv() {
            Ok(event) => handle_fs_event(event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn handle_fs_event(event: DebouncedEvent) {
    match event {
        DebouncedEvent::Rename(old_path, new_path) => {
            // Update SQLite paths
            update_media_path(&old_path, &new_path);
            update_playlist_references(&old_path, &new_path);
        },
        DebouncedEvent::Remove(path) => {
            // Mark as missing in playlists
            mark_media_missing(&path);
        },
        _ => {}
    }
}
```

---

These additions complete the deep technical logic for playlists, queue management, and enhanced subtitle workflows.

---

## Section 7 (NEW): Complete Audio Album Art Caching Flow

### **First Scan (Extraction + Cache)**

```rust
// src-tauri/src/audio_scanner.rs

pub async fn scan_audio_file(path: &Path) -> AudioMetadata {
    let mut metadata = extract_basic_metadata(path); // title, artist, album, duration
    
    // 1. Try to extract embedded album art
    match extract_album_art(path) {
        Ok(image_bytes) => {
            // 2. Generate unique hash for this album
            let cache_key = generate_cache_key(&metadata.artist, &metadata.album);
            
            // 3. Save to disk cache ONCE
            let cache_path = format!(
                "{}/flux-player/cache/images/album-art/{}.jpg",
                get_appdata_dir(),
                cache_key
            );
            
            fs::write(&cache_path, image_bytes)?;
            
            // 4. Store the LOCAL asset:// path in SQLite (NOT the binary data)
            metadata.album_art_path = Some(format!("asset://localhost/{}", cache_path));
        }
        Err(_) => {
            // 5. Try online fallback (MusicBrainz)
            if let Ok(online_art) = fetch_album_art_online(&metadata).await {
                let cache_key = generate_cache_key(&metadata.artist, &metadata.album);
                let cache_path = format!("{}/album-art/{}.jpg", cache_dir, cache_key);
                fs::write(&cache_path, online_art)?;
                metadata.album_art_path = Some(format!("asset://localhost/{}", cache_path));
            } else {
                // 6. No art found - will use Flux logo fallback in UI
                metadata.album_art_path = None;
            }
        }
    }
    
    metadata
}
```

---

### **SQLite Schema (Stores Path, Not Binary)**

```sql
CREATE TABLE media (
    path TEXT PRIMARY KEY,
    title TEXT,
    artist TEXT,
    album TEXT,
    duration INTEGER,
    album_art_path TEXT,  -- Stores "asset://localhost/C:/Users/.../album-art/abc123.jpg"
    -- NOT storing binary image data in SQLite
    ...
);
```

---

### **Every Subsequent Access (Zero Extraction)**

```svelte
<!-- AudioCard.svelte -->
<script>
  // media.album_art_path comes directly from SQLite
  // NO lofty calls, NO API calls, just a cached file path
  
  let posterSrc = $derived(
    media.album_art_path ||       // "asset://localhost/..." (cached)
    '/flux2d.png'     // Flux logo if no art exists
  );
</script>

<img src={posterSrc} alt={media.title} class="album-cover" />
```

---

### **Why This Is Efficient**

| Operation | First Scan | Library Refresh | UI Render (Grid) | UI Render (Detail Panel) |
|-----------|-----------|----------------|-----------------|-------------------------|
| **lofty extraction** | ✅ Once | ❌ No | ❌ No | ❌ No |
| **MusicBrainz API** | ✅ If needed | ❌ No | ❌ No | ❌ No |
| **File I/O (read cache)** | ✅ Write once | ❌ No | ✅ Read cached file | ✅ Read cached file |
| **SQLite read** | ✅ Insert path | ✅ Read path | ✅ Read path | ✅ Read path |

---

### **Cache Key Generation (Deduplication)**

Multiple songs from the same album should share ONE cached image:

```rust
use sha2::{Sha256, Digest};

fn generate_cache_key(artist: &str, album: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(artist.to_lowercase().as_bytes());
    hasher.update(album.to_lowercase().as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string() // First 16 hex chars
}
```

**Example:**
```
Artist: "Kendrick Lamar"
Album: "DAMN."
Cache Key: "a3f7c9e2b1d4f6a8"
Cache Path: "%APPDATA%/flux-player/cache/images/album-art/a3f7c9e2b1d4f6a8.jpg"
```

All 14 tracks from "DAMN." will reference the **same cached file**.

---

### **Manual Refresh Behavior**

When the user clicks the **"Refresh Metadata"** button in the Detail Panel:

```rust
pub async fn refresh_audio_metadata(path: &Path) -> Result<(), Error> {
    // 1. Re-extract metadata (in case tags were updated)
    let new_metadata = extract_basic_metadata(path);
    
    // 2. Check if album art cache still exists
    let cache_key = generate_cache_key(&new_metadata.artist, &new_metadata.album);
    let cache_path = format!("{}/album-art/{}.jpg", cache_dir, cache_key);
    
    if !Path::new(&cache_path).exists() {
        // Cache was deleted or album changed → re-extract
        if let Ok(art) = extract_album_art(path) {
            fs::write(&cache_path, art)?;
        }
    }
    
    // 3. Update SQLite with new metadata + existing cache path
    update_media_in_db(path, new_metadata)?;
    
    Ok(())
}
```

---

### **Global Refresh (Titlebar Button)**

The **Global Refresh** button should:
1. ✅ Re-scan all Base Folders for new/removed files
2. ✅ Update SQLite with any metadata changes
3. ❌ **NOT** re-extract album art from files that already have cached images
4. ✅ Only extract art for **newly added** files

```rust
pub async fn global_refresh() {
    for folder in get_base_folders() {
        for file in scan_directory(folder) {
            if !file_exists_in_db(&file.path) {
                // New file → full scan including art extraction
                let metadata = scan_audio_file(&file.path).await;
                insert_into_db(metadata);
            } else {
                // Existing file → only update lightweight metadata (duration, tags)
                let basic_meta = extract_basic_metadata(&file.path);
                update_db(file.path, basic_meta);
                // Album art cache remains untouched
            }
        }
    }
}
```