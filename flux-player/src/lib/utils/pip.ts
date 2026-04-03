import { getCurrentWindow, LogicalSize, LogicalPosition, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/window';
import { currentMonitor } from '@tauri-apps/api/window';

interface SavedWindowState {
  position: PhysicalPosition | LogicalPosition | null;
  size: PhysicalSize | LogicalSize | null;
  maximized: boolean;
}

let savedState: SavedWindowState | null = null;

/**
 * Enters Picture-in-Picture mode by making the window small,
 * removing decorations, keeping it always on top, and moving it
 * to the bottom-right corner.
 */
export async function enterPiP() {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
    console.warn('PiP mode requires a Tauri environment.');
    return;
  }

  const appWindow = getCurrentWindow();

  try {
    // 1. Save current window state
    const isMaximized = await appWindow.isMaximized();
    const position = await appWindow.innerPosition();
    const size = await appWindow.innerSize();

    savedState = {
      position,
      size,
      maximized: isMaximized
    };

    // 2. Unmaximize if needed before resizing
    if (isMaximized) {
      await appWindow.unmaximize();
    }

    // 3. Set window properties for PiP
    await appWindow.setAlwaysOnTop(true);
    await appWindow.setDecorations(false);

    // 4. Calculate bottom right corner and resize/reposition
    const monitor = await currentMonitor();

    // Default fallback values
    const pipWidth = 400;
    const pipHeight = 225;

    // Use LogicalSize for setting
    await appWindow.setSize(new LogicalSize(pipWidth, pipHeight));

    if (monitor) {
      // Calculate available work area in logical coordinates
      const scaleFactor = monitor.scaleFactor;

      const workAreaWidth = monitor.size.width / scaleFactor;
      const workAreaHeight = monitor.size.height / scaleFactor;

      const monitorX = monitor.position.x / scaleFactor;
      const monitorY = monitor.position.y / scaleFactor;

      // Bottom right corner with a small 20px padding
      const x = monitorX + workAreaWidth - pipWidth - 20;
      const y = monitorY + workAreaHeight - pipHeight - 20;

      await appWindow.setPosition(new LogicalPosition(x, y));
    }

  } catch (error) {
    console.error('Failed to enter PiP mode:', error);
  }
}

/**
 * Exits Picture-in-Picture mode and restores the window to its previous state.
 */
export async function exitPiP() {
  if (typeof window === 'undefined' || !('__TAURI_INTERNALS__' in window)) {
    return;
  }

  const appWindow = getCurrentWindow();

  try {
    await appWindow.setAlwaysOnTop(false);
    await appWindow.setDecorations(true);

    if (savedState) {
      if (savedState.maximized) {
        await appWindow.maximize();
      } else {
        if (savedState.size) {
          await appWindow.setSize(savedState.size);
        }
        if (savedState.position) {
          await appWindow.setPosition(savedState.position);
        }
      }
      savedState = null;
    } else {
      // Fallback if state was somehow lost
      await appWindow.center();
      await appWindow.setSize(new LogicalSize(1280, 720));
    }
  } catch (error) {
    console.error('Failed to exit PiP mode:', error);
  }
}
