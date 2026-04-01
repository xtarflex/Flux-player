import { convertFileSrc } from '@tauri-apps/api/core';

/**
 * @file media.ts
 * @description Utilities for handling media sources and path resolution.
 */

/**
 * Safely resolves a media source (path or URL) to a browser-loadable URL.
 * Handles absolute disk paths, relative web paths, and existing protocols.
 * 
 * @param src - The raw source from database or component (e.g. "C:\path", "/icon.png", "http://...")
 * @param fallback - Path to use if src is missing (default: "/flux2d.png")
 * @returns A string URL safe for <img> or <video> tags.
 */
export function resolveResource(src: string | null | undefined, fallback: string = '/flux2d.png'): string {
  if (!src) return fallback;

  /**
   * 1. Check if the input is a web asset or already resolved.
   */
  if (
    src.startsWith('http://') || 
    src.startsWith('https://') || 
    src.startsWith('data:') || 
    src.startsWith('asset:') ||
    src.startsWith('blob:')
  ) {
    return src;
  }

  // 2. Absolute filesystem paths (Windows or Unix)
  // Check for file:// protocol or absolute Windows drive letter (e.g. C:\)
  // or Unix absolute path (e.g. /home) that isn't a web asset.
  let cleanPath = src;
  let isLocalFile = false;

  if (cleanPath.startsWith('file:///')) {
    cleanPath = cleanPath.replace('file:///', '');
    isLocalFile = true;
  } else if (cleanPath.startsWith('file://')) {
    cleanPath = cleanPath.replace('file://', '');
    isLocalFile = true;
  }

  // Windows absolute path (e.g., C:\...) or Unix absolute path (e.g., /mnt/...)
  // We distinguish /assets or /flux2d.png from /home/user by checking if it exists 
  // in the web root, but in Tauri, usually anything starting with / that isn't
  // a known asset is a risk. For now, on Windows, if it starts with / and it's 
  // not a known small set of web assets, we could treat it as local.
  // HOWEVER, the safest way is to check for drive letters or the file:// flag we just set.
  
  const isWindowsAbsolute = /^[a-zA-Z]:[\\/]/.test(cleanPath);
  
  // On Windows, a path starting with / is almost certainly a web asset (e.g. /flux2d.png).
  // On Unix, a path starting with / is likely a local filesystem path.
  // We handle this by checking if we explicitly found a file:// protocol or a drive letter.
  if (isLocalFile || isWindowsAbsolute) {
    try {
      return convertFileSrc(cleanPath);
    } catch (e) {
      console.warn(`[resolveResource] Failed to convert path: ${src}`, e);
      return fallback;
    }
  }

  // 3. Fallback to original string (e.g. "/flux2d.png" relative assets)
  return src;
}
