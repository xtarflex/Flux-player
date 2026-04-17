# Prompt for Second Opinion on Tauri v2 Production Path Issues

You can use the following prompt to ask another AI or an expert for a second opinion on the production path and asset loading issues we've been encountering.

---

**Task: Technical Audit of Tauri v2 Production Path Resolution**

I am building a media player (Flux) using **Tauri v2 + SvelteKit**. We are encountering "file not found" and "permission denied" errors specifically in the **production NSIS build**, while everything works perfectly in development (`npm run tauri dev`).

**The Problem:**
Media assets (videos/posters) stored on various local drives (C:, D:, etc.) fail to load in the production build. We suspect the Tauri sandbox or capability system is blocking access.

**Current Configuration:**
1. **Frontend**: Uses `convertFileSrc(path)` to generate `asset://` URLs.
2. **Cargo.toml**: `tauri` has `protocol-asset` enabled.
3. **tauri.conf.json**:
   - `security.assetProtocol.enable` is `true`.
   - `security.assetProtocol.scope` includes `["C:/**", "D:/**", ... ]`.
   - `csp` includes `img-src asset: https://asset.localhost; media-src asset: https://asset.localhost;`.
4. **Capabilities (default.json)**:
   - `fs:allow-read` and `fs:allow-exists` are set for the relevant drives.
   - Using standard `core:default` and `core:path:default`.
5. **Platform**: Windows 10/11 (NSIS Installer).

**Specific Symptoms:**
- Posters/Videos show as broken links or trigger console errors about failing to load the resource.
- The paths are absolute Windows paths (e.g., `C:\Users\...\video.mp4`) that are passed through `convertFileSrc`.

**Questions for Audit:**
1. In Tauri v2, does `convertFileSrc` require any specific permission in the `capabilities.json` file beyond what is in `tauri.conf.json`?
2. Is there a known interaction between `tauri-plugin-fs` scopes and the built-in `asset` protocol scopes? 
3. Does the NSIS installer's "controlled folder access" or installation location affect how the asset protocol handles absolute paths on other drives?
4. What is the most robust way to debug path resolution on a production build without access to a debugger?

---
