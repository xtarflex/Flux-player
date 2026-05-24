## 2026-05-24 - [Path Traversal in cache_tmdb_image]
**Vulnerability:** Path traversal vulnerability due to unsanitized `image_type` user input used directly in file paths.
**Learning:** Constructing file paths with unsanitized parameters allows arbitrary file writes. Also, unsanitized file extensions from URLs can lead to dangerous files being saved.
**Prevention:** Use an allowlist for path segments and validate/sanitize file extensions (e.g., alphanumeric and length check).
