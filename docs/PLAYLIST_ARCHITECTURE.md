# Flux Player: Playlist & Queue Architecture

This document provides a complete technical specification for Flux's playlist and queue systems, including data structures, UI flows, and playback logic.

---

## Overview

Flux supports three distinct collection types:
1. **Playlists** — Permanent, user-curated collections saved to SQLite
2. **Queue** — Temporary, session-based playback list (cleared on app close)
3. **Smart Playlists** — Auto-generated collections based on criteria

**Key Distinction:**
- **Playlists** are like "albums" or "mixtapes" — you create them, name them, and they persist forever
- **Queue** is like "play next" — temporary additions that only exist during your current session

---

## Types of Playlists

### 1. Manual Playlists (User-Created)
User explicitly selects items and saves them as a named collection.

**Examples:**
- "Workout Mix" (15 songs)
- "MCU Marathon" (8 movies in chronological order)
- "90s Action Movies" (12 films)
- "Study Vibes" (30 instrumental tracks)

**Characteristics:**
- User controls all content
- Can contain audio, video, or mixed media
- Order is manually defined (can be reordered)
- Persistent (saved to SQLite)

---

### 2. Smart Playlists (Auto-Generated)
System automatically populates playlists based on user-defined rules.

**Pre-Built Smart Playlists:**
- **Albums** — Groups audio files by album metadata
- **Artists** — Groups audio files by artist
- **Series** — Groups video files detected as TV series
- **Recently Added** — Last 50 items added to library

**User-Created Smart Playlists:**
Users can create custom smart playlists with criteria like:
- Date Added (last 30 days, last week, etc.)
- Play Count (most played, never watched)
- Rating (★ ≥ 7.0)
- Genre (Action, Comedy, etc.)
- Year (1990-1999, 2020+)
- Duration (< 90 mins, > 2 hours)

**Characteristics:**
- Content updates automatically
- User defines rules, not individual items
- Can sort and limit results
- Persistent (rules saved to SQLite)

---

### 3. Favorites
A special auto-playlist containing all media marked as "favorite."

**Behavior:**
- Appears as a single playlist card in the Playlist Screen
- Automatically includes any item the user stars/favorites
- Cannot be manually edited (add/remove via favoriting items)
- Sorted by date favorited (most recent first)

---

## Data Structure (SQLite)

### Playlists Table
```sql
CREATE TABLE playlists (
    id TEXT PRIMARY KEY,                    -- UUID
    name TEXT NOT NULL,                     -- "MCU Marathon"
    description TEXT,                       -- Optional user description
    type TEXT CHECK(type IN ('audio', 'video', 'mixed')),
    created_at INTEGER NOT NULL,            -- Unix timestamp
    updated_at INTEGER NOT NULL,            -- Unix timestamp
    is_smart BOOLEAN DEFAULT 0,             -- 0 = manual, 1 = smart
    smart_criteria TEXT,                    -- JSON string (for smart playlists)
    cover_image TEXT                        -- Path to custom cover (optional)
);
```

### Playlist Items Table (Junction)
```sql
CREATE TABLE playlist_items (
    playlist_id TEXT NOT NULL,
    media_path TEXT NOT NULL,               -- References media.path
    position INTEGER NOT NULL,              -- Order in playlist (0, 1, 2...)
    added_at INTEGER NOT NULL,              -- Unix timestamp
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
    FOREIGN KEY (media_path) REFERENCES media(path) ON DELETE CASCADE,
    PRIMARY KEY (playlist_id, media_path)
);
```

### Smart Playlist Criteria (JSON Format)
```json
{
  "rules": [
    {
      "field": "date_added",
      "operator": "is_in_last",
      "value": 30,
      "unit": "days"
    },
    {
      "field": "genre",
      "operator": "contains",
      "value": "Action"
    }
  ],
  "logic": "AND",              // "AND" or "OR" between rules
  "sort_by": "date_added",
  "sort_order": "desc",
  "limit": 50
}
```

---

## The Queue System

### What is the Queue?
The queue is a **session-based, temporary playback list** that defines what plays next. Unlike playlists:
- Not saved to SQLite (exists only in memory)
- Clears when app closes
- Can be manually modified during playback
- Auto-populated when playing a playlist

### Queue Behavior

#### Manual Queue Addition
```
User right-clicks media card → "Add to Queue"
       ↓
Item appends to end of queue
       ↓
Continues playing after current media finishes
```

#### Playlist-Generated Queue
```
User clicks "Play All" on a playlist
       ↓
All playlist items populate the queue in order
       ↓
Queue shows "From Playlist: MCU Marathon" indicator
       ↓
Items auto-advance as each finishes
```

#### Queue Persistence Rules
- **During session:** Queue persists across section navigation
- **On app close:** Queue is completely cleared
- **On playlist change:** Replaces current queue with new playlist items
- **Mixed additions:** User can add individual items to a playlist-generated queue

---

## Playlist Creation Flows

### Method 1: Batch Select
```
Library → Right-click media card (enters Batch Select Mode)
       ↓
User selects 5 movies via checkboxes
       ↓
Clicks [Create Playlist] button
       ↓
Modal appears:
  ┌─────────────────────────────────────┐
  │  Create New Playlist                │
  │  ─────────────────────────────────  │
  │  Name: [MCU Marathon_________]      │
  │  Type: ● Movies  ○ Music  ○ Mixed  │
  │  Description (optional):            │
  │  [________________________]         │
  │                                     │
  │  5 items will be added              │
  │                                     │
  │        [Cancel]  [Create Playlist]  │
  └─────────────────────────────────────┘
       ↓
Playlist saved to SQLite
       ↓
Appears in sidebar (if in top 5 most recent)
```

---

### Method 2: Context Menu (Single Item)
```
User clicks [⋮] menu button on media card
       ↓
Context menu appears:
  ┌───────────────────────────┐
  │  Play Now                 │
  │  Add to Queue             │
  │  ──────────────────────   │
  │  Select Item              │
  │  Add to Playlist ────────►│ ┌─────────────────┐
  │  Add to Favorite          │ │ ● MCU Marathon  │
  │  Remove from Library      │ │ ○ 90s Action    │
  │  Get Subtitles            │ │ ─────────────   │
  │  Details                  │ │ + New Playlist  │
  └───────────────────────────┘ └─────────────────┘
       ↓
If "+ New Playlist" selected → Opens creation modal
If existing playlist selected → Item added instantly
```

---

### Method 3: From Playlist Screen
```
Playlist Screen → Click [+ Add New] in "Created Playlists" section
       ↓
Opens creation modal (empty playlist)
       ↓
User names playlist, selects type
       ↓
Playlist created → Opens Playlist Detail View
       ↓
User clicks [+ Add Items] button
       ↓
Library overlay appears (can select multiple items)
       ↓
Items added to playlist
```

---

## Sidebar Playlist Section

### Structure
```
┌─────────────────────────────┐
│  📑 PLAYLISTS               │ ← Click to open Playlist Screen
│  ───────────────────────    │
│  📌 MCU Marathon (pinned)   │ ← Pinned playlists (always visible)
│  🎵 Workout Mix             │ ← 1st most recent
│  🎬 90s Action              │ ← 2nd most recent
│  🎵 Study Vibes             │ ← 3rd most recent
│  🎬 Anime Binge             │ ← 4th most recent
│  🎵 Road Trip Mix           │ ← 5th most recent
│                             │
│  [+ 12 more...] ───────────►│ Opens Playlist Screen
└─────────────────────────────┘
```

### Display Rules
- **Maximum 5 playlists** shown directly in sidebar
- Sort by most recently created/modified
- **Pinned playlists** always appear at top (with 📌 icon)
- Pinned playlists don't count toward the 5-item limit
- If more than 5 exist, show "[+ X more...]" link

### Pinning Logic
```
User right-clicks playlist in sidebar → "Pin to Sidebar"
       ↓
Playlist moves to top section with 📌 icon
       ↓
Remains visible regardless of creation date
       ↓
To unpin: Right-click → "Unpin from Sidebar"
```

---

## Playlist Screen (Main View)

Clicking "PLAYLISTS" in the sidebar opens this dedicated full-screen view:

```
┌──────────────────────────────────────────────────────────┐
│  PLAYLISTS                                    [+ New]    │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  ╔═══════════════════════════════════════════════════╗  │
│  ║  CREATED PLAYLISTS                    [+ Add New] ║  │
│  ╚═══════════════════════════════════════════════════╝  │
│                                                           │
│  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐        │
│  │ 🎬     │  │ 🎵     │  │ 🎬     │  │ 🎵     │        │
│  │ MCU    │  │ Workout│  │ 90s    │  │ Study  │        │
│  │ Marathon│  │ Mix    │  │ Action │  │ Vibes  │        │
│  │ 6 items│  │ 15 item│  │ 8 items│  │ 30 item│        │
│  └────────┘  └────────┘  └────────┘  └────────┘        │
│                                                           │
│  ╔═══════════════════════════════════════════════════╗  │
│  ║  SMART PLAYLISTS                      [+ Add New] ║  │
│  ╚═══════════════════════════════════════════════════╝  │
│                                                           │
│  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐        │
│  │ 🔮     │  │ 🔮     │  │ 🔮     │  │ 🔮     │        │
│  │ Albums │  │ Artists│  │ Series │  │ Recent │        │
│  │ [Auto] │  │ [Auto] │  │ [Auto] │  │ Added  │        │
│  │ 45 item│  │ 23 item│  │ 12 item│  │ 50 item│        │
│  └────────┘  └────────┘  └────────┘  └────────┘        │
│                                                           │
│  ╔═══════════════════════════════════════════════════╗  │
│  ║  FAVORITES                                        ║  │
│  ╚═══════════════════════════════════════════════════╝  │
│                                                           │
│  ┌────────┐                                              │
│  │ ⭐     │  Shows single playlist card                  │
│  │ Favs   │  containing all favorited items             │
│  │ [Auto] │                                              │
│  │ 18 item│                                              │
│  └────────┘                                              │
└──────────────────────────────────────────────────────────┘
```

### Section Behaviors

#### Created Playlists
- All manually created playlists appear as cards
- Sorted by creation date (most recent first)
- **[+ Add New]** → Opens "Create Playlist" modal

#### Smart Playlists
- Pre-built: Albums, Artists, Series, Recently Added
- User-created smart playlists appear here
- **[+ Add New]** → Opens Smart Playlist criteria builder

#### Favorites
- Single auto-playlist card
- Clicking opens Playlist Detail View showing all favorited items
- Items auto-added when user favorites media
- Sorted by date favorited (newest first)

---

## Playlist Detail View

Clicking any playlist card opens this dedicated view:

```
┌──────────────────────────────────────────────────────────┐
│  ← Back to Playlists        MCU MARATHON          [Edit] │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  [▶ Play All]  [🔀 Shuffle Play]  [+ Add Items]          │
│                                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │ 1. Iron Man (2008)               [⋮]  ━━●━━ 45:23 │  │
│  │ 2. The Incredible Hulk (2008)    [⋮]  ━━━━━ 0:00  │  │
│  │ 3. Iron Man 2 (2010)             [⋮]  ━━━━━ 0:00  │  │
│  │ 4. Thor (2011)                   [⋮]  ━━━━━ 0:00  │  │
│  │ 5. Captain America (2011)        [⋮]  ━━━━━ 0:00  │  │
│  │ 6. The Avengers (2012)           [⋮]  ━━━━━ 0:00  │  │
│  └────────────────────────────────────────────────────┘  │
│                                                           │
│  Total Duration: 12h 34m  •  6 items  •  Created: Mar 24│
└──────────────────────────────────────────────────────────┘
```

### Features

#### Header Actions
- **[← Back]** — Returns to Playlist Screen
- **[Edit]** — Enters edit mode (rename, change description, delete playlist)

#### Playback Actions
- **[▶ Play All]** — Starts playback from first item, populates queue with all items
- **[🔀 Shuffle Play]** — Randomizes order, starts playback, populates queue
- **[+ Add Items]** — Opens Library overlay for selecting additional items

#### Item Actions (Per Item [⋮] Menu)
- **Play Next** — Moves item to front of queue
- **Move Up/Down** — Reorders position in playlist
- **Remove from Playlist** — Deletes from this playlist only (doesn't delete file)
- **Go to File** — Highlights item in Library view

#### Progress Indicators
Each item shows:
- Current watch/listen progress (━━●━━ or time remaining)
- "0:00" for unwatched/unplayed items
- "Completed" badge for 100% watched

---

## Playback Modes

### Sequential (Default)
Plays items in listed order, stops at the end.
```
Track 1 → Track 2 → Track 3 → ... → Track N → Stop
```

### Repeat Playlist
Loops the entire playlist indefinitely.
```
Track 1 → Track 2 → ... → Track N → Track 1 (repeat)
```

### Repeat One
Loops the current track indefinitely.
```
Track 5 → Track 5 → Track 5 → ... (forever)
```

### Shuffle
Plays items in random order (with no repeats until all played).
```
Track 3 → Track 7 → Track 1 → Track 5 → ... (randomized)
```

### UI Controls (In Playback Footer)
```
[🔀] Shuffle — Toggle on/off (cyan when active)
[🔁] Repeat  — Cycles through: Off → Playlist → One
```

---

## Auto-Advance Logic

### How Auto-Play Next Works

#### Video Playback
```
Video reaches end (currentTime >= duration)
       ↓
Check: Is there a next item in queue?
       ↓
YES → Netflix-style instant jump to next video
      (No countdown, seamless transition)
       ↓
NO → Route back to Library, Island → Idle
```

#### Audio Playback
```
Audio track reaches end
       ↓
Check: Is there a next item in queue?
       ↓
YES → Next track begins immediately
      Island updates to new track info
      User remains in current section
       ↓
NO → Island morphs to Idle, playback stops
```

#### Mixed Playlist (Audio → Video Transition)
```
Audio track finishes
       ↓
Next item is a video
       ↓
Audio stops, Island briefly shows "Switching to Video..." state
       ↓
Routes to Now Playing section
       ↓
Video begins playback
```

---

## Library Queue Behavior (Settings-Controlled)

### The Question:
When a user plays media from the Library (not from a playlist), should it:
- Play only that one item, then stop?
- Auto-generate a queue from the current filtered view?

### The Solution: Hybrid Approach (User Configurable)

**Settings → Playback → Auto-Queue Library Items**

#### Option 1: Never (Default)
Single-item playback only. No automatic queue generation.
```
User double-clicks "Inception" in Library
       ↓
Inception plays
       ↓
Inception ends
       ↓
Routes back to Library, playback stops
```

#### Option 2: Smart
Auto-queues only for detected series/albums.
```
User double-clicks "Blue Lock Episode 1" (detected as series)
       ↓
System creates implicit queue: [Ep 1, Ep 2, Ep 3, ...]
       ↓
Episodes auto-advance
       ↓
User double-clicks "Inception" (standalone movie)
       ↓
Plays only Inception, no queue
```

#### Option 3: Always
Any Library playback creates a queue from the current view.
```
Library filtered to "Action Movies, A-Z"
       ↓
User double-clicks "Blue Lock" (3rd item alphabetically)
       ↓
Queue = [Blue Lock, Guardians of the Galaxy, Inception, ...]
       (All items after the clicked one)
       ↓
Auto-advances through filtered list
```

---

## Queue UI Components

### 1. Three-Card Preview (In Playback Footer)

Located between center controls and right-side icons:
```
[⟲][⏮][▶][⏭][⟳]    ┌──┐┌────┐┌──┐    [1x][Vol][▭][⛶]
                     │  ││Curr││  │
                     └──┘└────┘└──┘
                      ↑    ↑    ↑
                    Prev  Now  Next
```

#### Visual Design
- **Left Card:** Previous track (50% hidden behind center)
- **Center Card:** Currently playing (fully visible, elevated 4px)
- **Right Card:** Next in queue (50% hidden behind center)
- Cards show poster/album art thumbnails (no titles)
- Center card has subtle glow/border to indicate active state

#### Interaction
- **Clicking center card** → Expands Queue Panel (see below)
- **Clicking left card** → Jumps to previous track
- **Clicking right card** → Skips to next track

---

### 2. Queue Panel (Expandable)

Clicking the three-card stack opens this floating panel:
```
┌─────────────────────────────────────┐
│  CURRENT QUEUE          [Clear All] │
│  ─────────────────────────────────  │
│  Now Playing:                       │
│  🎬 Blue Lock Ep 1       ━━●━━ 12:34│
│                                     │
│  Up Next (2 items):                 │
│  1. 🎬 Blue Lock Ep 2    ━━━━━ 24:15│
│  2. 🎬 Blue Lock Ep 3    ━━━━━ 23:48│
│                                     │
│  From Playlist: "Anime Binge"       │
│                                     │
│  [View Full Queue ──→]              │
└─────────────────────────────────────┘
```

#### Features
- **[Clear All]** — Clears entire queue (shows confirmation warning)
- **Progress bars** — Shows watch/listen progress for each item
- **"From Playlist: ..."** — Indicator if queue came from a playlist
- **[View Full Queue]** — Opens full-screen queue management view

---

### 3. Full Queue View (Dedicated Screen)

Clicking "View Full Queue" opens a dedicated queue management screen:
```
┌──────────────────────────────────────────────────────────┐
│  ← Back                PLAYBACK QUEUE        [Clear All] │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  NOW PLAYING                                             │
│  ┌────────────────────────────────────────────────────┐  │
│  │ 🎬 Blue Lock Episode 1       ━━━●━━━  12:34 / 24:15│  │
│  └────────────────────────────────────────────────────┘  │
│                                                           │
│  UP NEXT (5 items)                                       │
│  ┌────────────────────────────────────────────────────┐  │
│  │ 1. 🎬 Blue Lock Ep 2          [⋮]  ━━━━━━━  24:15  │  │
│  │ 2. 🎬 Blue Lock Ep 3          [⋮]  ━━━━━━━  23:48  │  │
│  │ 3. 🎬 Blue Lock Ep 4          [⋮]  ━━━━━━━  24:02  │  │
│  │ 4. 🎬 Guardians of the Galaxy [⋮]  ━━━━━━━  2:01:00│  │
│  │ 5. 🎬 Inception               [⋮]  ━━━━━━━  2:28:00│  │
│  └────────────────────────────────────────────────────┘  │
│                                                           │
│  From Playlist: "Anime Binge"                            │
│  Mixed with manual additions                             │
└──────────────────────────────────────────────────────────┘
```

#### Item Actions ([⋮] Menu)
- **Play Next** — Moves item to position 1 in queue
- **Move Up/Down** — Reorders within queue
- **Remove from Queue** — Deletes from queue only
- **Save Queue as Playlist** — Converts current queue to permanent playlist

---

## Mixed Playlists (Audio + Video)

### Behavior
Flux allows creating playlists with both audio and video:
```
Playlist: "Chill Vibes"
  1. Song A (audio)
  2. Song B (audio)
  3. Music Video C (video)
  4. Song D (audio)
  5. Song E (audio)
```

### Playback Flow
```
Song A plays → User stays in Library, Island shows audio state
       ↓
Song B plays → Still in Library, Island updates
       ↓
Music Video C next → Audio stops, Island shows "Switching to Video..."
       ↓
Routes to Now Playing section, video begins
       ↓
Music Video C ends → Song D begins
       ↓
Routes back to Library (or stays in Now Playing if user preference)
       ↓
Island morphs to audio state
```

### User Setting
**Settings → Playback → Video-to-Audio Transition**
- **Stay in Now Playing** (show video player with album art for audio)
- **Return to Library** (default, route back when audio resumes)

---

## Playlist Limits

### Maximum Items per Playlist
**Soft Cap: 100 items**

When adding the 101st item:
```
┌──────────────────────────────────────────┐
│  ⚠ Large Playlist Warning                │
│  ────────────────────────────────────    │
│  This playlist has 100 items.            │
│  Adding more may affect performance.     │
│                                          │
│  Continue adding items?                  │
│                                          │
│        [Cancel]  [Add Anyway]            │
└──────────────────────────────────────────┘
```

**Behavior:**
- Warning appears but doesn't block addition
- After 100 items, [+ Add Items] button shows "(100+)" indicator
- Scrolling performance may degrade on older hardware

---

## Smart Playlist Creation UI

### Criteria Builder Modal
```
┌─────────────────────────────────────────────────────────┐
│  Create Smart Playlist                        [✕ Close] │
├─────────────────────────────────────────────────────────┤
│  Name: [Recently Added Movies________________]          │
│                                                         │
│  Rules:                                                 │
│  ┌───────────────────────────────────────────────────┐ │
│  │ [Type ▼] [is ▼] [Video ▼]                         │ │
│  │ [Date Added ▼] [is in the last ▼] [30] [days ▼]  │ │
│  │ [+ Add Rule]                                       │ │
│  │                                                     │ │
│  │ Match: ● All rules (AND)  ○ Any rule (OR)         │ │
│  └───────────────────────────────────────────────────┘ │
│                                                         │
│  Sort by: [Date Added ▼]  Order: [Newest First ▼]     │
│  Limit to: [50] items                                  │
│                                                         │
│  Preview: 23 items match these rules                   │
│                                                         │
│                    [Cancel]  [Create Smart Playlist]   │
└─────────────────────────────────────────────────────────┘
```

### Available Rule Fields
- **Type** — Audio, Video
- **Date Added** — is before, is after, is in the last X days/weeks/months
- **Play Count** — is, is greater than, is less than
- **Rating** — is, is greater than, is less than
- **Genre** — contains, does not contain
- **Year** — is, is not, is between
- **Duration** — is less than, is greater than
- **File Format** — is, is not (.mp4, .mkv, .mp3, .flac, etc.)
- **Resolution** — 720p, 1080p, 4K (video only)

### Logic Options
- **Match All (AND)** — Item must satisfy every rule
- **Match Any (OR)** — Item must satisfy at least one rule

---

## Playlist Sync & Maintenance

### File System Changes
When files are renamed, moved, or deleted on disk:

#### Scenario 1: File Renamed
```
User renames "Episode 1.mp4" → "S01E01.mp4" on disk
       ↓
Flux's background file watcher detects change
       ↓
SQLite updates `media.path` to new filename
       ↓
All playlists containing this file auto-update
       ↓
No user intervention required
```

#### Scenario 2: File Moved
```
User moves "Movie.mp4" from D:\ to E:\
       ↓
Flux detects file missing at old path
       ↓
Scans library for matching file (by size + duration)
       ↓
If found → Updates path in SQLite
If not found → Marks as "Missing" in playlists
```

#### Scenario 3: File Deleted
```
User deletes "Movie.mp4" from disk
       ↓
Flux detects file missing
       ↓
Playlist item shows as "Missing" with red indicator
       ↓
User can:
  - Remove from playlist
  - "Locate File" (browse to new location)
  - Restore from backup (if applicable)
```

---

## Edge Cases & Special Behaviors

### Empty Playlist
When a playlist has zero items:
```
┌──────────────────────────────────────────┐
│  MCU MARATHON                     [Edit] │
│                                          │
│  [+ Add Items]                           │
│                                          │
│  This playlist is empty.                 │
│  Add media to get started.               │
│                                          │
│  0 items  •  Created: Mar 24, 2024      │
└──────────────────────────────────────────┘
```

---

### Playlist with Missing Items
```
┌──────────────────────────────────────────────────────────┐
│  MCU MARATHON                                     [Edit] │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  [▶ Play Available]  [🔧 Fix Missing Items]             │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ 1. Iron Man (2008)          [⋮]  ━━●━━ 45:23        │  │
│  │ 2. ⚠ Hulk (Missing)         [⋮]  [Locate File]     │  │
│  │ 3. Iron Man 2 (2010)        [⋮]  ━━━━━ 0:00         │ │
│  └────────────────────────────────────────────────────┘  │
│                                                          │
│  ⚠ 1 of 3 items missing  •  Last verified: 2 hours ago  │
└──────────────────────────────────────────────────────────┘
```

**[🔧 Fix Missing Items]** opens a batch repair modal:
- Shows all missing files
- "Locate" button for each (browse to new location)
- "Remove All Missing" option
- "Auto-Search Library" (attempts to find matches)

---

### Duplicate Detection
When adding an item already in the playlist:
```
┌──────────────────────────────────────────┐
│  Duplicate Item                          │
│  ────────────────────────────────────    │
│  "Inception" is already in this          │
│  playlist at position 3.                 │
│                                          │
│  Add it again?                           │
│                                          │
│        [Cancel]  [Add Anyway]            │
└──────────────────────────────────────────┘
```

---

## Performance Considerations

### Large Playlists (100+ items)
- Virtualized scrolling in Playlist Detail View
- Lazy-load item metadata (only load visible items)
- Thumbnail caching to prevent repeated disk reads

### Queue Performance
- Queue stored in memory (JavaScript array)
- Maximum 500 items in queue (soft limit)
- If queue exceeds 500, warn user of potential performance impact

### Smart Playlist Updates
- Smart playlists recalculate on:
  - App launch
  - Manual refresh (user clicks refresh icon)
  - When criteria field changes (e.g., new file added, play count updated)
- Heavy smart playlists (complex criteria) cache results for 5 minutes

---

## Summary

This architecture provides:
✅ **Flexibility** — Manual, smart, and favorite playlists  
✅ **Performance** — Session-based queue, smart caching  
✅ **User Control** — Extensive customization and reordering  
✅ **Resilience** — Handles missing files, renames, moves  
✅ **Seamless Playback** — Auto-advance, mixed media support  

All components integrate with Flux's existing Library, Search, and Playback systems for a cohesive, premium media player experience.
