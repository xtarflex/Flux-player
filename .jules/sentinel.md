
## 2024-05-20 - Fix Path Traversal in cache_tmdb_image Tauri Command
**Vulnerability:** Path traversal in a Tauri backend command allowing unverified file type strings (`image_type`) and arbitrarily long file extensions to inject malicious directory paths.
**Learning:** Even internal backend endpoints taking input directly from client (frontend) calls via IPC require stringent path sanitization and strict allowlisting. Standard path joining (`join`) on `PathBuf` can be overridden by absolute paths passed directly through `image_type`.
**Prevention:** Strictly allowlist directory parameters sent by IPC. Limit extensions to alphanumeric strings and strip out query parameters before usage.
