# Flux Player - Mini TODO (Technical Debt & Roadmap)

This list tracks granular technical tasks and "Premium UX" improvements following the Beta 1 launch.

## Active / Upcoming
- [ ] **Real-time Audio Sync**: Implement Rust-based `IMMNotificationClient` listener for instant UI updates on device change.
- [ ] **HUD Focus Hardening**: Allow HUD to show when main window is "Not Focused" (not just minimized).
- [ ] **Maintenance Hub**: Implementation of Factory Reset and GUI-based Uninstaller.
- [ ] **Beta Update Badge**: Lightweight startup check for new releases via GitHub.

## In Progress
- [/] **Infrastructure Pass**: Updating `DEV.log`, `TODO.md`, and uninstaller scripts.

## Completed (Recently)
- [x] **HUD Z-Order Hardening**: Moved window pinning to Rust for OS-level priority.
- [x] **Cinematic Filter Sync**: Smooth `out:fade` transitions for the "Watched" filter.
- [x] **Registry & Invoke**: Standardized `show_hud` command registration.
