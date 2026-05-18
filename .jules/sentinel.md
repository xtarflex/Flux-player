## 2024-05-18 - Path Traversal in Image Caching
**Vulnerability:** The `image_type` parameter was directly appended to cache directories in `cache_tmdb_image`, leading to a path traversal vulnerability. File extensions were also unsanitized.
**Learning:** This architectural gap lacked boundary checking on filesystem interaction, enabling arbitrary folder creation when saving metadata images.
**Prevention:** Always validate user-provided paths (like `image_type`) against a strict allowlist and sanitize file extensions before appending them to a filesystem path.
