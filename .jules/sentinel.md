
## 2024-05-18 - [CRITICAL] Prevent Path Traversal in Image Caching
**Vulnerability:** The `cache_tmdb_image` command in `src-tauri/src/commands/library.rs` constructed file paths using unsanitized user inputs (`image_type` and `file_extension`), enabling path traversal attacks (e.g., passing `../../etc/passwd` via URL extension or `image_type` parameter).
**Learning:** Rust's `PathBuf::join` gracefully processes `..` segments when constructing paths. External input determining file paths or extensions must always be rigorously validated and sanitized to prevent malicious directory traversal.
**Prevention:** Strictly validate `image_type` against an allowlist (e.g., `"posters" | "backdrops" | "album-art"`). Sanitize extracted `file_extension` from external URLs by restricting it to alphanumeric characters and enforcing a length limit before appending it to paths.
