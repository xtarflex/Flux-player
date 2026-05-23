
## 2024-05-23 - [Fix Path Traversal in Image Caching]
**Vulnerability:** Path traversal vulnerability due to using unvalidated `image_type` in the file path construction in `cache_tmdb_image` command.
**Learning:** External inputs controlling directory structure must be strictly allowlisted to prevent writing files to arbitrary directories, avoiding path traversal attacks.
**Prevention:** Implement strict allowlisting on all inputs that directly manipulate file paths and sanitize file extensions to expected types (e.g., alphanumeric and length limited).
