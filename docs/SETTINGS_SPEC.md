# Flux Player: Settings Hub Specification

This document outlines the architecture, default values, and customization limits of the Flux Player Settings Hub.

## 1. Hub Structure

The Settings Hub is organized into five primary functional vertical tabs to maintain a clean, high-performance UI.

### A. My Profile (Identity)

* **Display Name:**

  * *Default:* The current Windows OS username (extracted via Tauri OS API).

  * *Customization:* Fully editable text field. Used for dynamic UI greetings on the Home Screen.

* **Profile Avatar:**

  * *Default:* A sleek, premium vector placeholder (`flux.logo` or initials).

  * *Customization:* Users can upload an avatar. An in-app Image Editor allows cropping and panning before masking it into a CSS squircle.

* **Local Statistics:**

  * *Features:* Displays offline insights (Total Watch Time, Top Genres). Non-editable.

### B. Storage & Library (The Brain)

* **Base Folders Management:**

  * *Default:* The system's default "Videos" and "Music" folders.

  * *Customization:* Full CRUD capability for paths and a manual refresh button.

* **Auto-Indexing:**

  * *Default:* ON (Scans every 30 minutes).

  * *Customization:* Toggle ON/OFF; slider for scan frequency.

* **Windows Indexing Integration:**

  * *Default:* OFF (Standard recursive scan).

  * *Customization:* Toggle to "Experimental Native Search".

### C. Playback & Performance (The Muscle)

* **Hardware Acceleration (GPU):**

  * *Default:* ON (Auto-detected).

  * *Customization:* Toggle ON/OFF.

* **FFmpeg Threading:**

  * *Default:* "Auto".

  * *Customization:* Selectable (1 to Max Cores).

* **"Watched" Threshold:**

  * *Default:* 90%.

  * *Customization:* Slider (70% to 100%).

* **Subtitle Preferences:**

  * **Preferred Language:** Inherited from OS.

  * **Subtitle Fuzzy Matching (Local):** ON by default.

  * **Auto-Fetch Subtitles (Online):** ON by default.

  * **OpenSubtitles Account (For Local Downloads):** Optional Login for higher limits.

### D. Appearance & UI (The Face)

* **UI Theme:**

  * *Default:* "Cyber Dark" (Base: `#0a0a0c`, Accent: `#8a2be2`).

  * *Customization:* Selection between "Cyber Dark," "Neon High Contrast," and "Adaptive Tint".

### E. Streaming & Network (The Link)

* **External API Keys (TMDB):**

  * *Default:* The embedded Flux Shared Key Pool (Limited to 150 calls via 3 rotated keys).

  * *Customization:* Dedicated input field for **TMDB API Read Access Token (v4)**.

  * *Helper UI:* Includes the "Cheat Sheet" text for easy registration:

    * App Name: `Flux Player Local`

    * App URL: `https://github.com/flux-player`

    * Summary: `A local desktop media player using the TMDB API to fetch posters and metadata for my personal file collection.`

* **Streaming Quality:**

  * *Default:* "Best Available."

  * *Customization:* Manual caps (720p, 1080p, 4K).

## 2. Immutable vs. Mutable Features

| Feature Category | Customizable? | Reason for Lock | 
 | ----- | ----- | ----- | 
| **Typography** | NO | `Syncopate` and `Outfit` are core to the Flux branding. | 
| **Macro Layout** | NO | The CSS Grid is optimized for performance. | 
| **Animation Timing** | NO | Physics are tuned for "zero-gravity" feel. | 
| **Base Palette** | YES | Users can switch accent colors or use Adaptive Tinting. | 
| **User Identity** | YES | Name and Avatar personalize the local experience. | 
| **File Paths** | YES | Users must have total control over media locations. | 
| **Performance** | YES | Essential for running Flux on varying hardware. | 

## 3. Configuration Persistence

All settings are stored locally via `Tauri Store` in `%APPDATA%/flux-player/`. No cloud sync is utilized.