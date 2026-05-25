
## 2024-05-24 - Tooltip Visibility Through Splash Overlay
**Learning:** When writing Playwright verification tests for UI components that render early (like the footer), the `.splash-overlay` might block pointer events. Hover actions will fail with a timeout unless you explicitly wait for the overlay to be hidden/detached or use `force=True`.
**Action:** Always include a `page.locator(".splash-overlay").first.wait_for(state="detached")` or similar wait condition when writing automated interaction scripts for Flux Player.
