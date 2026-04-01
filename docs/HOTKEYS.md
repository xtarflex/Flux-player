# Flux Player: Keyboard Navigation & Global Hotkeys

This document defines all keyboard shortcuts, navigation patterns, and accessibility controls for Flux Player.

---

## 1. DESIGN PHILOSOPHY

### Core Principles
- **Power User First**: Keyboard should be as fast (or faster) than mouse
- **Industry Standard**: Follow VLC/Spotify conventions where possible
- **Context Aware**: Same key does different things in different sections
- **Visual Feedback**: Every action shows subtle confirmation UI
- **Escape Hatch**: ESC always returns to previous state
- **No Conflicts**: Global hotkeys don't interfere with text input

### Accessibility Goals
- Full keyboard navigation without mouse
- Screen reader compatibility (ARIA labels)
- Visual focus indicators (Cyan ring: `outline: 2px solid var(--secondary)`)
- Skip navigation shortcuts (jump to main content)

---

## 2. GLOBAL HOTKEYS (Work Everywhere)

These work regardless of which section you're in:

### Playback Control (Media Keys)
| Hotkey | Action | Notes |
|--------|--------|-------|
| `Space` | Play/Pause | Only when not typing in search |
| `K` | Play/Pause | Alternative (YouTube-style) |
| `←` | Rewind 5 seconds | |
| `→` | Forward 5 seconds | |
| `J` | Rewind 10 seconds | |
| `L` | Forward 10 seconds | |
| `Shift + ←` | Previous track/episode | |
| `Shift + →` | Next track/episode | |
| `M` | Mute/Unmute | |
| `↑` | Volume +10% | |
| `↓` | Volume -10% | |
| `0-9` | Jump to 0%-90% of timeline | `5` = 50% through video |
| `Home` | Jump to beginning | |
| `End` | Jump to end | |

### Window & Navigation
| Hotkey | Action | Notes |
|--------|--------|-------|
| ` Ctrl + D` | Go to Discovery | |
| `Ctrl + L` | Go to Library | |
| `Ctrl + ,` | Open Settings | |
| `Ctrl + P` | Go to Playlists | |
| `Ctrl + Q` | View Queue | |
| `Ctrl + F` | Focus search bar | Opens if closed |
| `Ctrl + R` | Refresh Context | Section-aware refresh |
| `Ctrl + Shift + R` | Global Refresh | App/Window reload |
| `Ctrl + B` | Toggle Sidebar | Collapse/Expand |
| `Ctrl + W` | Close window | Confirm if playing |
| `F11` | Toggle fullscreen | Video only |
| `ESC` | Exit fullscreen / Close modal | Context-aware |

### Quick Actions
| Hotkey | Action | Notes |
|--------|--------|-------|
| `Ctrl + N` | New playlist | Opens creation modal |
| `Ctrl + Shift + N` | New smart playlist | |
| `Ctrl + O` | Add folder to library | Opens folder picker |
| `Ctrl + S` | Save current queue as playlist | |
| `Ctrl + Shift + S` | Screenshot (Now Playing) | Saves to Pictures folder |
| `Ctrl + /` | Toggle keyboard shortcuts panel | Shows this reference |

---

## 3. SECTION-SPECIFIC SHORTCUTS

### A. Discovery SCREEN

#### Discovery Navigation
| Hotkey | Action | Visual Feedback |
|--------|--------|-----------------|
| `Tab` | Focus next row/card | Cyan focus ring |
| `Shift + Tab` | Focus previous row/card | |
| `←` `→` | Navigate cards horizontally | Smooth scroll |
| `↑` `↓` | Navigate rows vertically | |
| `Enter` | Open Detail Panel | Slide from right |
| `Ctrl + Enter` | Play immediately | Skips Detail Panel |
| `/` | Focus TMDB search bar | 500ms debounce |
| `ESC` | Clear search / Close Detail | |

#### Detail Panel (Discovery)
| Hotkey | Action | Notes |
|--------|--------|-------|
| `P` | Play / Stream Online | Context-aware button |
| `T` | Watch Trailer | Opens lightbox |
| `F` | Add to Favorites | Star icon fills |
| `Q` | Add to Queue | Toast confirmation |
| `Shift + P` | Add to Playlist | Opens playlist selector |
| `ESC` | Close Detail Panel | Returns to Discovery grid |

---

### B. LIBRARY SCREEN

#### Grid/List Navigation
| Hotkey | Action | Visual Feedback |
|--------|--------|-----------------|
| `Tab` | Focus next media card | Cyan ring |
| `Shift + Tab` | Focus previous card | |
| `←` `→` `↑` `↓` | Arrow key navigation | Grid-aware movement |
| `Enter` | Open Detail Panel | Docked panel updates |
| `Ctrl + Enter` | Play immediately | |
| `/` | Focus local search bar | 0ms instant filter |
| `Ctrl + A` | Select all (Batch Mode) | Enter multi-select |
| `Shift + Click` | Select range | Like Windows Explorer |
| `Ctrl + Click` | Toggle individual selection | |
| `ESC` | Clear selection / search | Exit Batch Mode |

#### View & Sort
| Hotkey | Action | Current State Shown |
|--------|--------|---------------------|
| `V` | Cycle view mode | Grid → List → Detail |
| `S` | Open Sort menu | Dropdown appears |
| `F` | Open Filter menu | Status/Genre/Year |
| `Ctrl + =` | Zoom in (Grid) | Larger cards |
| `Ctrl + -` | Zoom out (Grid) | Smaller cards |
| `Ctrl + 0` | Reset zoom | Default size |

#### Quick Actions (Library)
| Hotkey | Action | Notes |
|--------|--------|-------|
| `Delete` | Remove from Library | Confirmation modal |
| `Shift + Delete` | Delete file from disk | **Permanent!** Requires confirmation |
| `E` | Edit metadata | Opens inline editor |
| `R` | Refresh metadata from TMDB | Re-fetches poster/info |
| `I` | Get info / Details | Same as Enter |
| `W` | Toggle Watched status | Eye icon toggles |
| `F` | Add to Favorites | Star fills |

---

### C. NOW PLAYING (Video/Audio)

#### Playback Precision
| Hotkey | Action | UI Feedback |
|--------|--------|-------------|
| `Space` | Play/Pause | Icon toggles |
| `,` | Frame-by-frame backward | Video only |
| `.` | Frame-by-frame forward | Video only |
| `[` | Decrease speed (0.25x steps) | Speed indicator updates |
| `]` | Increase speed (0.25x steps) | Max 2x |
| `Shift + [` | Reset to 1x speed | |
| `<` | Slow motion (0.5x) | One-key toggle |
| `>` | Fast forward (1.5x) | One-key toggle |

#### Subtitles & Audio
| Hotkey | Action | Notes |
|--------|--------|-------|
| `C` | Toggle subtitles on/off | |
| `Shift + C` | Cycle subtitle tracks | EN → ES → FR → OFF |
| `Z` | Subtitle delay -100ms | Sync adjustment |
| `X` | Subtitle delay +100ms | |
| `Shift + Z` | Reset subtitle timing | |
| `A` | Cycle audio tracks | Multi-language support |

#### Display
| Hotkey | Action | Visual Change |
|--------|--------|--------------|
| `F` | Toggle fullscreen | Theater mode |
| `D` | Toggle playback controls | Auto-hide UI |
| `I` | Toggle video info overlay | FPS, bitrate, codec |
| `Shift + I` | Toggle detailed stats | Buffer health, dropped frames |
| `P` | Toggle PiP mode | OS-level PiP window |
| `Shift + P` | Toggle in-app mini player | Bottom-right overlay |

#### Navigation While Playing
| Hotkey | Action | Behavior |
|--------|--------|----------|
| ` Ctrl + D` | Return to Discovery | Player enters mini mode |
| `Ctrl + L` | Return to Library | Player enters mini mode |
| `Ctrl + Q` | View Queue | Overlay panel |
| `N` | Open Now Playing view | Exits mini mode |

---

### D. PLAYLIST SCREEN

#### Playlist Management
| Hotkey | Action | Notes |
|--------|--------|-------|
| `↑` `↓` | Navigate playlists | Vertical list |
| `Enter` | Open Playlist Detail View | |
| `Ctrl + N` | Create new playlist | Opens modal |
| `Ctrl + Shift + N` | Create smart playlist | Criteria builder |
| `Delete` | Delete playlist | Confirmation required |
| `F2` | Rename playlist | Inline edit |
| `Ctrl + D` | Duplicate playlist | Creates copy |

#### Playlist Detail View
| Hotkey | Action | Visual Feedback |
|--------|--------|-----------------|
| `↑` `↓` | Navigate tracks | Highlight moves |
| `Enter` | Play from selected track | Queue populates |
| `Space` | Play/Pause (if already playing) | |
| `Delete` | Remove from playlist | Item fades out |
| `Ctrl + ↑` | Move track up | Reorder animation |
| `Ctrl + ↓` | Move track down | |
| `Ctrl + A` | Select all tracks | Multi-select mode |
| `Ctrl + I` | Invert selection | |
| `S` | Shuffle playlist order | Randomize |
| `ESC` | Return to Playlist Screen | |

---

### E. SETTINGS HUB

#### Tab Navigation
| Hotkey | Action | Visual State |
|--------|--------|--------------|
| `Ctrl + Tab` | Next settings tab | Tabs highlight |
| `Ctrl + Shift + Tab` | Previous tab | |
| `1` | My Profile tab | |
| `2` | Storage & Library tab | |
| `3` | Playback & Performance tab | |
| `4` | Appearance & UI tab | |
| `5` | Streaming & Network tab | |

#### Profile Tab
| Hotkey | Action | Notes |
|--------|--------|-------|
| `Ctrl + U` | Upload avatar | Opens file picker |
| `Ctrl + E` | Edit display name | Focus text field |

#### Actions
| Hotkey | Action | Notes |
|--------|--------|-------|
| `Ctrl + S` | Save settings | Auto-saves anyway |
| `Ctrl + R` | Reset to defaults | Confirmation modal |
| `ESC` | Close Settings | Returns to previous view |

---

## 4. BATCH SELECTION MODE

**Entry:** `Ctrl + A` in Library or `Shift/Ctrl + Click`

### While in Batch Mode
| Hotkey | Action | UI State |
|--------|--------|----------|
| `Ctrl + A` | Select all | All checkboxes checked |
| `Ctrl + Shift + A` | Deselect all | Clear selection |
| `Ctrl + I` | Invert selection | Swap checked state |
| `Space` | Toggle current item | Checkbox toggle |
| `Shift + ↑/↓` | Extend selection | Range select |
| `Delete` | Remove selected from Library | Batch operation |
| `Ctrl + P` | Add selected to playlist | Multi-item add |
| `Ctrl + Q` | Add selected to queue | Batch queue |
| `ESC` | Exit Batch Mode | Clear all selections |

---

## 5. SEARCH BAR FOCUS MODE

**Entry:** `Ctrl + F` or `/` or click search bar

### While Typing in Search
| Hotkey | Action | Notes |
|--------|--------|-------|
| `ESC` | Clear search & exit | Returns focus to grid |
| `Enter` | Execute search / Open first result | Context-dependent |
| `↓` | Focus search results | Arrow navigation |
| `Ctrl + A` | Select all text | Standard text edit |
| `Ctrl + Backspace` | Delete word | |

**Note:** Playback shortcuts (`Space`, `K`, arrow keys) are **disabled** while typing to prevent conflicts.

---

## 6. MODAL DIALOG SHORTCUTS

Applies to: Playlist Creation, Confirmation Dialogs, API Key Entry, etc.

| Hotkey | Action | Notes |
|--------|--------|-------|
| `Enter` | Confirm / Submit | Primary action |
| `ESC` | Cancel / Close | Secondary action |
| `Tab` | Focus next field | Form navigation |
| `Shift + Tab` | Focus previous field | |
| `Ctrl + Enter` | Submit form | Skip validation |

---

## 7. DYNAMIC ISLAND SHORTCUTS

**Island States:** Idle, Hover, Status, Playing

### Island Interactions
| Hotkey | Action | State Change |
|--------|--------|--------------|
| `Ctrl + I` | Focus/Expand Island | Idle → Status |
| `ESC` | Collapse Island | Status → Idle |
| `Enter` | Execute Island action | Context-aware |
| `Space` | Play/Pause (if playing) | Toggle playback |
| `/` | Convert to search bar | Opens input |

---

## 8. ACCESSIBILITY SHORTCUTS

### Screen Reader Support
| Hotkey | Action | Purpose |
|--------|--------|---------|
| `Ctrl + Shift + H` | Skip to main content | Bypass navigation |
| `Ctrl + Shift + F` | Skip to footer controls | Jump to playback bar |
| `Ctrl + Shift + M` | Open accessibility menu | Font size, contrast |
| `H` | Next heading (browse mode) | ARIA navigation |
| `B` | Next button (browse mode) | |

### Visual Assistance
| Hotkey | Action | Effect |
|--------|--------|--------|
| `Ctrl + Shift + +` | Increase UI scale | Zoom entire app |
| `Ctrl + Shift + -` | Decrease UI scale | |
| `Ctrl + Shift + 0` | Reset UI scale | Default 100% |
| `Ctrl + Shift + C` | Toggle high contrast mode | Black/white theme |

---

## 9. DEBUGGING & POWER USER

**Hidden shortcuts for developers and power users:**

| Hotkey | Action | Purpose |
|--------|--------|---------|
| `Ctrl + Shift + D` | Open DevTools | Chromium inspector |
| `Ctrl + Shift + I` | Show app info overlay | Version, uptime, stats |
| `Ctrl + Shift + L` | Export logs | Saves to Documents/Flux/logs |
| `Ctrl + Shift + R` | Hard reload (clear cache) | Fresh start |
| `Ctrl + Alt + Delete` | Emergency stop playback | Kill all media |

---

## 10. IMPLEMENTATION NOTES

### A. Technical Architecture

#### Tauri Global Shortcuts (Rust)
```rust
// src-tauri/src/shortcuts.rs
use tauri::GlobalShortcutManager;

pub fn register_global_shortcuts(app: &tauri::App) {
    let mut shortcuts = app.global_shortcut_manager();
    
    // Media keys (work even when unfocused)
    shortcuts.register("MediaPlayPause", || {
        // Emit to frontend
        app.emit_all("media-play-pause", {}).unwrap();
    }).unwrap();
    
    shortcuts.register("MediaNextTrack", || {
        app.emit_all("media-next", {}).unwrap();
    }).unwrap();
}
```

#### Svelte Event Listeners
```javascript
// src/lib/shortcuts.ts
import { listen } from '@tauri-apps/api/event';

export function initShortcuts() {
    // Listen for Rust-emitted events
    listen('media-play-pause', () => {
        player.togglePlayback();
    });
    
    // Window-level key handlers
    window.addEventListener('keydown', handleKeyDown);
}

function handleKeyDown(e: KeyboardEvent) {
    // Ignore if typing in input
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') {
        return;
    }
    
    // Route to context-aware handler
    const section = getCurrentSection();
    switch(section) {
        case 'Discovery': handleDiscoveryKeys(e); break;
        case 'library': handleLibraryKeys(e); break;
        case 'now-playing': handlePlayerKeys(e); break;
    }
}
```

### B. Visual Feedback System

Every keyboard action must show **subtle confirmation**:

```css
/* Keyboard focus ring (Flux style) */
*:focus-visible {
    outline: 2px solid var(--secondary); /* Cyber Cyan */
    outline-offset: 2px;
    border-radius: 4px;
}

/* Action confirmation toast */
.kbd-toast {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: rgba(26, 26, 31, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid var(--secondary);
    padding: 12px 24px;
    border-radius: 8px;
    font-family: var(--font-heading);
    font-size: 0.8rem;
    animation: kbd-toast-fade 1s ease-out;
    pointer-events: none;
}

@keyframes kbd-toast-fade {
    0% { opacity: 0; transform: translate(-50%, -50%) scale(0.8); }
    20% { opacity: 1; transform: translate(-50%, -50%) scale(1); }
    80% { opacity: 1; }
    100% { opacity: 0; }
}
```

---

## 11. QUICK REFERENCE CARD

### Essential Shortcuts (Printable)

```
╔══════════════════════════════════════════════════════════╗
║               FLUX PLAYER - QUICK REFERENCE              ║
╠══════════════════════════════════════════════════════════╣
║ PLAYBACK                                                 ║
║ Space ............... Play/Pause                         ║
║ ← → ................. Seek ±5s                           ║
║ ↑ ↓ ................. Volume                             ║
║ M ................... Mute                               ║
║ F ................... Fullscreen                         ║
╠══════════════════════════════════════════════════════════╣
║ NAVIGATION                                               ║
║ Ctrl+D .............. Discovery                               ║
║ Ctrl+L .............. Library                            ║
║ Ctrl+P .............. Playlists                          ║
║ Ctrl+Q .............. Queue                              ║
║ Ctrl+F .............. Search                             ║
║ ESC ................. Exit/Close                         ║
╠══════════════════════════════════════════════════════════╣
║ QUICK ACTIONS                                            ║
║ Ctrl+N .............. New Playlist                       ║
║ Ctrl+O .............. Add Folder                         ║
║ Ctrl+/ .............. Show All Shortcuts                 ║
╚══════════════════════════════════════════════════════════╝
```

---

## 12. PLATFORM-SPECIFIC NOTES

### Windows
- `Ctrl` = Control key
- Media keys work via Windows Media API
- `Win + G` may conflict (Game Bar) - remind users to disable

### macOS
- `Ctrl` → `Cmd` for all shortcuts
- Media keys native support via MPNowPlayingInfoCenter
- `Cmd + Q` quits app (standard macOS)

### Linux
- Media keys depend on DE (GNOME/KDE/XFCE)
- Some distros may need manual keybind setup
- Wayland vs X11 behavior differences

---

## 13. FUTURE ENHANCEMENTS (V2/V3)

### Customizable Shortcuts
Allow users to rebind keys in Settings:

```
Settings → Appearance & UI → Keyboard Shortcuts
┌─────────────────────────────────────────┐
│ Action              Current    Custom   │
│ ───────────────────────────────────     │
│ Play/Pause          Space      [____]   │
│ Next Track          Shift+→    [____]   │
│ Volume Up           ↑          [____]   │
│                                          │
│ [Reset to Defaults]        [Save]       │
└─────────────────────────────────────────┘
```

### Macro Recording
Record sequences of actions:
- `Ctrl + Shift + M` → Start recording
- Perform actions
- `Ctrl + Shift + M` → Stop & save macro
- Assign to custom hotkey

### Voice Commands (V3)
Integrate with OS voice assistants:
- "Flux, play next"
- "Flux, add to favorites"
- "Flux, show me action movies"

---

## STATUS: READY FOR IMPLEMENTATION

This specification is complete and ready to be coded during Phase 1 (V1 MVP).

**Priority:** Medium-High (implement alongside UI components)

**Dependencies:**
- Tauri GlobalShortcutManager
- Svelte event system
- Focus trap utility (for modals)
- Toast notification component

**Estimated Complexity:** 🟡 Medium (3-5 days)

---

*Last Updated: March 22, 2026*  
*Document Version: 1.0*
