## 2024-05-24 - Path Traversal in Image Caching
**Vulnerability:** The `image_type` parameter in `cache_tmdb_image` (a Tauri command called from the frontend) was directly appended to a cache directory path without validation, allowing potential path traversal. Furthermore, the file extension extracted from the URL was not sanitized.
**Learning:** Even internal API inputs from the frontend must be strictly validated in the Tauri backend, especially when constructing file paths.
**Prevention:** Always use an allowlist for path segments passed from the frontend (e.g., validate `image_type` against known safe values). Sanitize dynamically extracted components like file extensions (e.g., ensure they are purely alphanumeric and limited in length).
