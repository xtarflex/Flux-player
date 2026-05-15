## 2024-05-15 - Prevent Path Traversal and Arbitrary File Extension Injection
**Vulnerability:** The `cache_tmdb_image` command in `flux-player/src-tauri/src/commands/library.rs` previously accepted arbitrary `image_type` and parsed file extensions from arbitrary URLs without validation, potentially leading to path traversal (`../../`) and writing files with arbitrary extensions.
**Learning:** Without sanitization of URL parameters and validation of string variables used for file paths, attackers can create or overwrite arbitrary files in the cache directories.
**Prevention:** Use an allowlist for path variables (e.g. `image_type` to "posters", "backdrops", "album-art") and sanitize external extensions by enforcing alphanumeric characters and a reasonable string length constraint.
