## 2024-05-18 - [Fix path traversal in image cache]
**Vulnerability:** The `cache_tmdb_image` command constructed file paths using unsanitized user inputs (`image_type` and `file_extension` from `url`), which could lead to path traversal vulnerabilities and allow writing arbitrary file types.
**Learning:** Always validate user-provided path segments against an allowlist and strictly sanitize file extensions before using them to construct file paths.
**Prevention:** Use an explicit allowlist for predefined directory segments (e.g., `["posters", "backdrops", "album-art"]`) and sanitize file extensions by checking for alphanumeric characters and length limits.
