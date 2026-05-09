## 2024-05-09 - [Path Traversal in cache_tmdb_image]
**Vulnerability:** The `cache_tmdb_image` command constructed file paths using unvalidated user input (`image_type` and URL file extension), allowing potential path traversal vulnerabilities and arbitrary file extension writes.
**Learning:** File paths must never be constructed directly from user input without validation.
**Prevention:** Implement allowlists for known path segments and strictly sanitize file extensions by checking their length and character composition.
