# Flux Player: Implementation Roadmap

> **Version:** V1 → V2 → V3  
> **Last Updated:** March 2026  
> **Status:** Pre-Development (Documentation Complete)

---

## Overview

This roadmap breaks down Flux Player development into **3 major phases** with clear milestones, complexity ratings, and document references. Each task includes estimated complexity (🟢 Low, 🟡 Medium, 🔴 High) and links to the specific documentation containing implementation details.

---

## Phase 1: Foundation & Core UI (V1 MVP)
**Goal:** Functional desktop player with local file playback and essential UI components  
**Timeline:** 8-12 weeks  
**Complexity:** 🟡 Medium

### Milestone 1.1: Application Shell & Navigation
**Duration:** 2 weeks | **Complexity:** 🟢 Low

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Configure Tauri for frameless window | 🟢 Low | `workflow_reconstruct.md` (Target 1) |
| Build custom titlebar with drag region | 🟢 Low | `forensic_blueprint.md` (Section 10) |
| Implement squircle profile avatar | 🟢 Low | `skill_aesthetic_enforcer.md` (Section 4) |
| Set up CSS Grid macro layout | 🟢 Low | `FLUX_MASTER_GUIDE.md` (Section 3) |
| Create sidebar navigation | 🟢 Low | `DIRECTORY_STRUCTURE.md` |

### Milestone 1.2: Dynamic Island Foundation
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build F-L-U-X SVG morphing logo | 🟡 Medium | `FLUX_MASTER_GUIDE.md` (Section 2) |
| Implement Svelte spring physics | 🟡 Medium | `forensic_blueprint.md` (Section 2) |
| Create Idle/Hover/Status states | 🟡 Medium | `FLUX_MASTER_GUIDE.md` (Section 2) |
| Add adaptive border tinting | 🟡 Medium | `forensic_blueprint.md` (Section 1) |
| Implement offline red state | 🟢 Low | `deep_logic_supplement.md` (Section 2) |

### Milestone 1.3: Settings Hub
**Duration:** 2 weeks | **Complexity:** 🟢 Low

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build vertical tabs layout | 🟢 Low | `SETTINGS_SPEC.md` (Section 1) |
| My Profile tab (OS username, avatar) | 🟡 Medium | `SETTINGS_SPEC.md` (Section 1.A) |
| Storage & Library tab (Base Folders) | 🟢 Low | `SETTINGS_SPEC.md` (Section 1.B) |
| Playback & Performance tab | 🟢 Low | `SETTINGS_SPEC.md` (Section 1.C) |
| Appearance & UI tab | 🟢 Low | `SETTINGS_SPEC.md` (Section 1.D) |
| Streaming & Network tab (API delegation) | 🟡 Medium | `SETTINGS_SPEC.md` (Section 1.E) |

### Milestone 1.4: Library Brain & SQLite
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Design SQLite schema | 🟡 Medium | `workflow_reconstruct.md` (Target 3) |
| Build local file scanner (Rust) | 🟡 Medium | `FLUX_MASTER_GUIDE.md` (Section 4) |
| Implement 0ms local search | 🟢 Low | `forensic_blueprint.md` (Section 4) |
| Create filter & sort logic | 🟢 Low | `PROJECT_CHARTER.md` (Section 3) |
| Build Grid/List/Detail view modes | 🟡 Medium | `PROJECT_CHARTER.md` (Section 3) |

### Milestone 1.5: Detail Panel
**Duration:** 1 week | **Complexity:** 🟢 Low

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build backdrop + poster overlap layout | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 7) |
| Implement metadata display | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 7) |
| Add gradient play button | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 7) |
| Create metadata edit/refresh UI | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 7) |

### Milestone 1.6: Basic Playback
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Integrate Video.js player | 🟡 Medium | `PROJECT_CHARTER.md` (Section 4) |
| Implement local file playback | 🟡 Medium | `workflow_reconstruct.md` (Target 3) |
| Build Now Playing section | 🟡 Medium | `UX_JOURNEYS.md` (Scenario 5) |
| Add playback state memory (SQLite) | 🟡 Medium | `deep_logic_supplement.md` (Section 4) |
| Create "Resume from [MM:SS]" button | 🟢 Low | `deep_logic_supplement.md` (Section 4) |

### Milestone 1.7: Playback Footer
**Duration:** 1 week | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build footer layout (3 sections) | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 6) |
| Implement gradient progress scrubber | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 6) |
| Create VLC-style volume bars | 🟡 Medium | `forensic_blueprint_ADDITIONS.md` (Section 16) |
| Add center playback controls | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 6) |
| Implement context-aware thumbnail | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 6) |

**Phase 1 Total:** ~12 weeks

---

## Phase 2: Advanced Features & Streaming (V2 Power User)
**Goal:** Playlist system, queue management, subtitle engine, and streaming capabilities  
**Timeline:** 10-14 weeks  
**Complexity:** 🔴 High

### Milestone 2.1: Playlist System
**Duration:** 3 weeks | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Design playlist SQLite schema | 🟡 Medium | `PLAYLIST_ARCHITECTURE.md` (Data Structure) |
| Build batch select mode | 🟡 Medium | `UX_JOURNEYS_ADDITIONS.md` (Scenario 7) |
| Create playlist creation flows | 🟡 Medium | `PLAYLIST_ARCHITECTURE.md` (Creation Flows) |
| Build Playlist Screen UI | 🟡 Medium | `PLAYLIST_ARCHITECTURE.md` (Playlist Screen) |
| Implement Playlist Detail View | 🟢 Low | `PLAYLIST_ARCHITECTURE.md` (Detail View) |
| Add smart playlist criteria builder | 🔴 High | `PLAYLIST_ARCHITECTURE.md` (Smart Playlists) |
| Create favorites auto-playlist | 🟢 Low | `PLAYLIST_ARCHITECTURE.md` (Favorites) |
| Implement sidebar playlist display | 🟢 Low | `PLAYLIST_ARCHITECTURE.md` (Sidebar) |

### Milestone 2.2: Queue System
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build session-based queue (memory) | 🟢 Low | `deep_logic_supplement_ADDITIONS.md` (Section 6) |
| Create three-card queue preview | 🟡 Medium | `forensic_blueprint_ADDITIONS.md` (Section 15) |
| Build queue panel (expandable popup) | 🟢 Low | `PLAYLIST_ARCHITECTURE.md` (Queue UI) |
| Create Full Queue View screen | 🟢 Low | `PLAYLIST_ARCHITECTURE.md` (Queue UI) |
| Implement auto-advance logic | 🔴 High | `forensic_blueprint_ADDITIONS.md` (Section 14) |
| Add playback modes (shuffle, repeat) | 🟡 Medium | `PLAYLIST_ARCHITECTURE.md` (Playback Modes) |
| Create "Clear Queue" with warning | 🟢 Low | `UX_JOURNEYS_ADDITIONS.md` (Scenario 8) |

### Milestone 2.3: Subtitle Engine
**Duration:** 2 weeks | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build fuzzy matching algorithm | 🟡 Medium | `forensic_blueprint_ADDITIONS.md` (Section 17) |
| Implement OSHash computation (Rust) | 🟡 Medium | `forensic_blueprint.md` (Section 7) |
| Create Subtitle Binder Modal UI | 🟢 Low | `deep_logic_supplement_ADDITIONS.md` (Section 5) |
| Integrate OpenSubtitles API | 🟡 Medium | `forensic_blueprint_ADDITIONS.md` (Section 17) |
| Build auto-download & binding | 🟡 Medium | `forensic_blueprint_ADDITIONS.md` (Section 17) |
| Add context menu "Get Subtitles" | 🟢 Low | `deep_logic_supplement_ADDITIONS.md` (Section 5) |
| Implement subtitle sync adjustment | 🟢 Low | `UX_JOURNEYS_ADDITIONS.md` (Scenario 9) |

### Milestone 2.4: Streaming & Scrapers
**Duration:** 3 weeks | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build scraper cascade (VidSrc/VidLink/Consumet) | 🔴 High | `workflow_reconstruct.md` (Target 4) |
| Integrate yt-dlp sidecar | 🟡 Medium | `workflow_reconstruct.md` (Target 4) |
| Implement clipboard intercept | 🟡 Medium | `UX_JOURNEYS.md` (Scenario 4) |
| Create trailer lightbox | 🟡 Medium | `UX_JOURNEYS.md` (Scenario 3) |
| Build TMDB online search (Home) | 🟡 Medium | `UX_JOURNEYS.md` (Scenario 3) |
| Add off-canvas Detail Drawer (Home) | 🟡 Medium | `forensic_blueprint.md` (Section 5) |

### Milestone 2.5: API Delegation System
**Duration:** 1 week | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build 150-call tracker (SQLite) | 🟢 Low | `deep_logic_supplement.md` (Section 3) |
| Create API key rotation pool (Rust) | 🟡 Medium | `deep_logic_supplement.md` (Section 3) |
| Build "Unlock Infinite Discovery" modal | 🟢 Low | `UX_JOURNEYS.md` (Scenario 2) |
| Add TMDB cheat sheet copy-paste UI | 🟢 Low | `UX_JOURNEYS.md` (Scenario 2) |

### Milestone 2.6: Offline Image Caching
**Duration:** 1 week | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Implement Rust image downloader | 🟡 Medium | `workflow_reconstruct.md` (Target 3) |
| Build local asset serving | 🟡 Medium | `deep_logic_supplement.md` (Section 4) |
| Create cache directory structure | 🟢 Low | `deep_logic_supplement.md` (Section 4) |
| Add fallback logic (Flux logo) | 🟢 Low | `PROJECT_CHARTER.md` (Section 4) |

### Milestone 2.7: Home Screen & Discovery
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build TMDB curated rows | 🟡 Medium | `PROJECT_CHARTER.md` (Section 3) |
| Create personalized greeting | 🟢 Low | `UX_JOURNEYS.md` (Scenario 1) |
| Implement 500ms debounced search | 🟢 Low | `deep_logic_supplement.md` (Section 1) |
| Add horizontal scroll logic | 🟢 Low | `PROJECT_CHARTER.md` (Section 3) |

**Phase 2 Total:** ~14 weeks

---

## Phase 3: Premium Features & Audio Engine (V3 Ultimate)
**Goal:** Advanced audio features, visualizers, and network casting  
**Timeline:** 8-10 weeks  
**Complexity:** 🔴 High

### Milestone 3.1: PiP Modes
**Duration:** 2 weeks | **Complexity:** 🟡 Medium

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build in-app mini player | 🟡 Medium | `PROJECT_CHARTER.md` (Section 5) |
| Create draggable/resizable logic | 🟡 Medium | `forensic_blueprint.md` (Custom) |
| Implement OS-level PiP | 🔴 High | `PROJECT_CHARTER.md` (Section 5) |
| Add PiP button to playback controls | 🟢 Low | `FLUX_MASTER_GUIDE_ADDITIONS.md` (Section 6) |

### Milestone 3.2: Audio Forge
**Duration:** 3 weeks | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Build FFT BPM Analyzer | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V3) |
| Create RMS Smart Crossfader | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V3) |
| Implement weighted shuffle logic | 🟡 Medium | `PROJECT_CHARTER.md` (Section 4, V3) |
| Add audio visualizer engine | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V3) |

### Milestone 3.3: FFmpeg Sidecar
**Duration:** 1 week | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Integrate FFmpeg binary | 🟡 Medium | `workflow_reconstruct.md` (Target 4) |
| Build .mkv/.avi transcoder | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V2) |
| Create format detection logic | 🟡 Medium | `PROJECT_CHARTER.md` (Section 4, V2) |
| Add progress indicator | 🟢 Low | `deep_logic_supplement.md` (Section 1) |

### Milestone 3.4: Network Casting
**Duration:** 2 weeks | **Complexity:** 🔴 High

| Task | Complexity | Reference Document |
|------|-----------|-------------------|
| Implement Chromecast support | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V3) |
| Add DLNA discovery | 🔴 High | `PROJECT_CHARTER.md` (Section 4, V3) |
| Create device selection UI | 🟡 Medium | `PROJECT_CHARTER.md` (Section 4, V3) |
| Build remote playback controls | 🟡 Medium | `PROJECT_CHARTER.md` (Section 4, V3) |

**Phase 3 Total:** ~8 weeks

---

## Complexity Legend

| Symbol | Level | Description |
|--------|-------|-------------|
| 🟢 | Low | Standard implementation, well-documented patterns |
| 🟡 | Medium | Requires system integration, moderate complexity |
| 🔴 | High | Complex algorithm, external API, or performance-critical |

---

## Document Reference Index

### Core Architecture
- `PROJECT_CHARTER.md` — Roadmap, scope, version goals
- `DIRECTORY_STRUCTURE.md` — File organization, folder structure
- `AGENTIAL_DEVELOPMENT.md` — AI collaboration guidelines

### Visual & UX Specifications
- `FLUX_MASTER_GUIDE.md` — Brand DNA, aesthetics, physics
- `FLUX_MASTER_GUIDE_ADDITIONS.md` — Footer & Detail Panel specs
- `skill_aesthetic_enforcer.md` — Typography, colors, Dynamic Island rules
- `UX_JOURNEYS.md` — User flows (Scenarios 1-6)
- `UX_JOURNEYS_ADDITIONS.md` — User flows (Scenarios 7-9)

### Technical Implementation
- `forensic_blueprint.md` — Technical "magic" explanations (Sections 1-13)
- `forensic_blueprint_ADDITIONS.md` — Advanced implementations (Sections 14-17)
- `deep_logic_supplement.md` — Hidden technical details (Sections 1-5)
- `deep_logic_supplement_ADDITIONS.md` — Playlist & Queue logic (Section 6)

### Feature-Specific Docs
- `PLAYLIST_ARCHITECTURE.md` — Complete playlist/queue system
- `SETTINGS_SPEC.md` — Settings Hub specifications
- `SETTINGS_SPEC_ADDITIONS.md` — New playback settings

### Workflows
- `workflow_reconstruct.md` — V1 reconstruction sequence
- `workflow_active_coding.md` — Standard coding procedure (5-step loop)

---

## Success Metrics

### Phase 1 (V1 MVP)
- ✅ Plays local video/audio files
- ✅ 0ms local search functional
- ✅ Dynamic Island morphs correctly
- ✅ Settings Hub fully operational
- ✅ Playback state saved (resume watching)

### Phase 2 (V2 Power User)
- ✅ Playlists create, edit, play seamlessly
- ✅ Queue auto-advances without gaps
- ✅ Subtitles auto-bind with >90% accuracy
- ✅ Streams launch successfully (>95% success rate)
- ✅ Offline image cache populates

### Phase 3 (V3 Ultimate)
- ✅ PiP modes work across all OS
- ✅ Audio crossfade smooth (no audible gaps)
- ✅ Chromecast/DLNA discovery <5 seconds
- ✅ Visualizer renders at 60fps

---

## Notes

- **Parallel Development:** Milestones within phases can be developed concurrently where dependencies allow
- **Testing Checkpoints:** Run manual verification after each milestone (see `workflow_active_coding.md` Step 4)
- **Documentation Updates:** Log all architectural decisions in `DEV.log` (see `agent.md` Core Directive 3)
- **Git Strategy:** Small, atomic commits after each task completion

---

**Next Action:** Begin Phase 1, Milestone 1.1 (Application Shell)
