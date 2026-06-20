## 2025-05-10 - Path Traversal Prevention in Image Caching
**Vulnerability:** The `cache_tmdb_image` command in `src-tauri/src/commands/library.rs` accepted an arbitrary `image_type` string to construct file paths, allowing potential path traversal (e.g., `../../../`) to write outside the intended cache directory. It also didn't validate the `file_extension` from the URL, posing a risk of malicious file drops.
**Learning:** Rust `PathBuf::join` doesn't sanitize strings implicitly. Trusting parameter-provided path segments from client commands is unsafe.
**Prevention:** Implement strict allowlists for path segment strings (`["posters", "backdrops", "album-art"]`) and sanitize file extensions by validating character types (alphanumeric only) and enforcing length bounds.
