import { invoke } from '@tauri-apps/api/core';
import { resolveResource } from '$lib/utils/media';
import type { MediaItem } from '$lib/stores/playback';

/**
 * @file player.ts
 * @description Extraction of core playback logic and utilities.
 */

/**
 * Extracts the dominant color from an image and updates the adaptive tint.
 * Averages 1000 sampled pixels from a 64x64 canvas.
 * 
 * @param src - The resolved asset URL.
 */
export function extractDominantColor(src: string): void {
  const img = new Image();
  img.crossOrigin = 'anonymous';
  img.onload = () => {
    const canvas = document.createElement('canvas');
    canvas.width = 64;
    canvas.height = 64;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.drawImage(img, 0, 0, 64, 64);
    const data = ctx.getImageData(0, 0, 64, 64).data;
    let r = 0, g = 0, b = 0, count = 0;
    const total = data.length / 4;
    const step = Math.max(1, Math.floor(total / 1000));
    for (let i = 0; i < total; i += step) {
      r += data[i * 4];
      g += data[i * 4 + 1];
      b += data[i * 4 + 2];
      count++;
    }
    const tint = `rgb(${Math.round(r / count)}, ${Math.round(g / count)}, ${Math.round(b / count)})`;
    document.documentElement.style.setProperty('--island-adaptive-tint', tint);
  };
  img.src = src;
}

/**
 * Persists playback progress to the backend.
 * 
 * @param path - File path as primary key.
 * @param currentTime - Current position in seconds.
 * @param duration - Total length in seconds.
 */
export async function savePlaybackProgress(path: string, currentTime: number, duration: number): Promise<void> {
  try {
    console.log(`[PlayerUtils] Saving: ${path} @ ${Math.floor(currentTime)}s`);
    await invoke('save_playback_progress', {
      path,
      seconds: Math.floor(currentTime),
      duration: Math.floor(duration),
    });
    // Update local store so library UI progress bars stay in sync
    const { updateLocalProgress } = await import('$lib/stores/media');
    updateLocalProgress(path, Math.floor(currentTime));
  } catch (e) {
    console.warn('[PlayerUtils] Failed to save progress:', e);
  }
}

/**
 * Configures a Video.js player with a MediaItem.
 * 
 * @param p - Video.js player instance.
 * @param item - The media item to load.
 */
export function loadItemIntoPlayer(p: any, item: MediaItem): void {
  const src = resolveResource(item.path);
  if (p.src() !== src) {
    p.src({ src, type: 'video/mp4' });
    const artSrc = item.album_art || item.poster;
    if (artSrc) {
      const resolvedArt = resolveResource(artSrc);
      p.poster(resolvedArt);
      extractDominantColor(resolvedArt);
    }
  }
  p.load();
}
