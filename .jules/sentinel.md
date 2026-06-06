## 2025-06-06 - Path Traversal in Image Cache
**Vulnerability:** The `cache_tmdb_image` Tauri command constructed a file path directly using the user-provided `image_type` string without validation, allowing potential path traversal (e.g., `../../`). Additionally, file extensions were extracted from URLs without sanitization.
**Learning:** Always treat frontend-supplied strings used in file path construction as untrusted. Tauri's `path` API helps resolve base directories, but doesn't protect against `join()` traversal if the appended string contains `../`.
**Prevention:** Validate path segments against a strict allowlist (e.g., `match image_type { "posters" => ..., _ => ... }`) and enforce length/alphanumeric checks on dynamically extracted extensions.
