## 2024-05-17 - Prevent Path Traversal in cache_tmdb_image
**Vulnerability:** Unsanitized `image_type` and `file_extension` in `cache_tmdb_image` allowed path traversal.
**Learning:** Always validate user-provided path segments against an allowlist and sanitize file extensions before using them to construct file paths.
**Prevention:** Use an allowlist for path segments and validate extensions for alphanumeric characters and length limits.
