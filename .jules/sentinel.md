## 2026-05-14 - Prevent Path Traversal in cache_tmdb_image
**Vulnerability:** The `cache_tmdb_image` function in `commands/library.rs` accepted an unvalidated `image_type` and unsanitized file extension from the URL, allowing arbitrary path traversal (e.g., using `../`) and arbitrary file creation.
**Learning:** Even internal API inputs must be strictly validated against allowlists, and parsed components from external URLs (like extensions) should be rigorously sanitized before being used in file system operations.
**Prevention:** Implement strict allowlisting for path segments derived from input and enforce strict alphanumeric, length-limited checks on file extensions.
