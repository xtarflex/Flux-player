## 2025-02-23 - Registry Command Injection Vulnerability
**Vulnerability:** Execution of raw `UninstallString` from Windows Registry using `Command::new` without separating executable and arguments.
**Learning:** Using `Command::new` with a string containing arguments attempts to execute the entire string as a path, failing if arguments exist, but also potentially executing arbitrary binaries if paths are unquoted and contain spaces.
**Prevention:** Always parse registry command strings to isolate the absolute executable path from its arguments, validate the path exists and has a safe extension, and pass arguments via `.args()`.
## 2025-02-23 - String Slicing Panic Risk
**Vulnerability:** String slicing panic due to byte-length mismatch when using `to_lowercase()` on Unicode strings before slicing.
**Learning:** `String::to_lowercase()` can change byte lengths for Unicode paths. Using `.find()` on the lowercase version and applying that index to the original string causes an out-of-bounds byte slice panic.
**Prevention:** Always use `.to_ascii_lowercase()` when generating indices for slicing the original string to ensure 1:1 byte alignment, or perform case-insensitive regex/searches directly on the original string.
