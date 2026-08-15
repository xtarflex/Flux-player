## 2026-08-15 - Secure Uninstaller Command Execution
**Vulnerability:** The 'open_uninstaller' function spawned a process based on an unparsed, unvalidated registry string, leading to potential command injection or execution of arbitrary files if the registry key was modified.
**Learning:** When spawning processes from external strings (like registry values), it's crucial to properly parse the string to isolate the executable from its arguments, and then validate the executable's path (ensure it's absolute, exists, and has a safe extension).
**Prevention:** Always parse, isolate, and strictly validate executables and their paths before passing them to std::process::Command::new.
