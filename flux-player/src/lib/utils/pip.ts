import { getCurrentWindow, LogicalSize, LogicalPosition, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window';
import { currentMonitor } from '@tauri-apps/api/window';

interface SavedWindowState {
  position: PhysicalPosition | LogicalPosition | null;
  size: PhysicalSize | LogicalSize | null;
  maximized: boolean;
}

/**
 * @private
 * Local cache for the window state before entering PiP.
 * Note: Decoupled here but could be moved to a persistent store if multi-window or 
 * session-persistence (across app restarts) is required in the future.
 */
let savedState: SavedWindowState | null = null;

/**
 * Enters Picture-in-Picture mode by transforming the main window.
 * Saves current state, removes decorations, and pins to the bottom-right.
 * 
 * @returns {Promise<boolean>} Success status
 */
export async function enterPiP(): Promise<boolean> {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
    console.warn('[PiP] Environment not supported.');
    return false;
  }

  const appWindow = getCurrentWindow();

  try {
    // 1. Capture current state for restoration
    const isMaximized = await appWindow.isMaximized();
    const position = await appWindow.innerPosition();
    const size = await appWindow.innerSize();

    savedState = {
      position,
      size,
      maximized: isMaximized
    };

    // 2. Transition to Mini-State
    if (isMaximized) await appWindow.unmaximize();
    
    await appWindow.setAlwaysOnTop(true);
    await appWindow.setDecorations(false);

    // 3. Dimensions & Positioning
    const monitor = await currentMonitor();
    const pipWidth = 400; // Flux standard PiP width
    const pipHeight = 225; // 16:9 aspect ratio

    await appWindow.setSize(new LogicalSize(pipWidth, pipHeight));

    if (monitor) {
      const scaleFactor = monitor.scaleFactor;
      const workAreaWidth = monitor.size.width / scaleFactor;
      const workAreaHeight = monitor.size.height / scaleFactor;
      const monitorX = monitor.position.x / scaleFactor;
      const monitorY = monitor.position.y / scaleFactor;

      // Bottom right with secondary-layer padding (24px)
      const x = monitorX + workAreaWidth - pipWidth - 24;
      const y = monitorY + workAreaHeight - pipHeight - 24;

      await appWindow.setPosition(new LogicalPosition(x, y));
    } else {
      // Fallback: just center if monitor info is missing
      await appWindow.center();
    }

    return true;
  } catch (error) {
    console.error('[PiP] Failed to enter:', error);
    // Attempt to rollout if stuck in halfway state
    await exitPiP();
    return false;
  }
}

/**
 * Exits Picture-in-Picture mode and restores the window.
 * If saved state is missing, falls back to a safe default center.
 */
export async function exitPiP(): Promise<void> {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) return;

  const appWindow = getCurrentWindow();

  try {
    await appWindow.setAlwaysOnTop(false);
    // Flux is undecorated by default (custom titlebar), so we should NOT set decorations to true
    // as it creates a duplicate native titlebar above our own.
    await appWindow.setDecorations(false);

    if (savedState) {
      // 1. Always restore the unmaximized size/position first.
      // This ensures that if the user clicks "Restore" later, they get their old size 
      // back instead of the tiny PiP dimensions.
      if (savedState.size) await appWindow.setSize(savedState.size);
      if (savedState.position) await appWindow.setPosition(savedState.position);

      // 2. Then maximize if that was the previous state
      if (savedState.maximized) {
        await appWindow.maximize();
      }
      savedState = null; // Clear cache
    } else {
      // Emergency Restore: Standard 1280x720 centered
      await appWindow.setSize(new LogicalSize(1280, 720));
      await appWindow.center();
    }
  } catch (error) {
    console.error('[PiP] Failed to exit:', error);
  }
}
