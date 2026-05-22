
## 2024-05-18 - Path Traversal in Image Cache
**Vulnerability:** The `cache_tmdb_image` Tauri command was taking a user-supplied string `image_type` and appending it directly to the path string `app_dir.join("cache").join("images").join(image_type)`. It also took file extensions unverified from a URL. This could allow for writing arbitrary files or path traversal if a user sent a path like `../../`.
**Learning:** File paths built via `.join()` on unfiltered string parameters in Rust backend commands are vulnerable. The URL file extension parsing was also unsanitized, allowing for arbitrarily long or invalid paths to be supplied.
**Prevention:** Always validate directory path segments against an allowlist and sanitize extracted strings (like file extensions) by enforcing acceptable length and character ranges (e.g., alphanumeric).
