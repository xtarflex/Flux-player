## 2026-05-11 - Prevent Path Traversal in Tauri Commands
**Vulnerability:** User-provided path segments (`image_type`) and file extensions in Tauri commands were used directly in `PathBuf::join`, allowing for potential path traversal.
**Learning:** In Tauri backends, always validate user inputs against explicit allowlists when constructing local paths, and rigorously sanitize variables derived from external inputs (like extensions from URLs).
**Prevention:** Implement strict string allowlists for path segments and use alphanumeric filters for extensions before constructing file paths.
