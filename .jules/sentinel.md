
## 2024-05-27 - Path Traversal via Cache Image Parameters
**Vulnerability:** Path traversal vulnerability existed in the `cache_tmdb_image` command where user-supplied `image_type` and unchecked URL extensions could allow arbitrary directory traversal and creation due to unsafe `PathBuf::join()` and raw URL string extraction.
**Learning:** `PathBuf::join()` replaces the preceding path entirely if the appended segment is an absolute path. Further, file extensions extracted from URLs often contain query strings (`.jpg?token=...`) which must be explicitly split and sanitized to avoid bypassing path validators.
**Prevention:** Always strict-allowlist user-provided segments intended for filesystem construction (e.g. valid cache directories). Always `.split('?').next()` when parsing extensions from URLs, followed by explicit ASCII alphanumeric filtering and length-limiting to prevent injection.
