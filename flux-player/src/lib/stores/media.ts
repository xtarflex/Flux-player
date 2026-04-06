/**
 * @file media.ts
 * @description Svelte store for the media library. On first load, fetches all 
 * media items from the SQLite database via plugin-sql. After a library scan 
 * completes (triggered by the `flux-library-updated` event), it refreshes 
 * automatically.
 */

import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface MediaItem {
  id: string;      // maps to `path` (PRIMARY KEY)
  path: string;
  title: string;
  artist: string | null;
  album: string | null;
  duration: number | null;  // seconds
  poster?: string | null;   // poster_path (asset:// or null)
  backdrop?: string | null; // backdrop_path
  album_art?: string | null;// album_art_path
  year: number | null;
  type: 'video' | 'audio' | 'mixed';
  last_played: number;
  added_at: number;
  // UI-only enriched fields
  rating?: number;
  genres?: string[];
  synopsis?: string;
  director?: string;
  starring?: string;
  subtitle?: string;
  series_tag?: string | null;
  is_watched?: boolean;
  last_position?: number;
  is_favorite?: boolean;
}

export type LibraryLoadState = 'idle' | 'loading' | 'done' | 'error';

export const mediaItems = writable<MediaItem[]>([]);
export const selectedMediaId = writable<string | null>(null);
export const libraryLoadState = writable<LibraryLoadState>('idle');
export const isScanning = writable<boolean>(false);

/**
 * Fetches all media items from the database via the Rust command.
 */
export async function loadLibraryFromDb(): Promise<void> {
  libraryLoadState.set('loading');
  try {
    const rows = await invoke<any[]>('get_all_media');

    const items: MediaItem[] = rows.map((row) => ({
      id: row.path,
      path: row.path,
      title: row.title,
      artist: row.artist,
      album: row.album,
      duration: row.duration,
      poster: row.poster_path,
      backdrop: row.backdrop_path,
      album_art: row.album_art_path,
      year: row.year,
      type: row.media_type as 'video' | 'audio' | 'mixed',
      last_played: row.last_played,
      added_at: row.added_at,
      synopsis: row.synopsis,
      rating: row.rating,
      genres: row.genres,
      director: row.director,
      starring: row.starring,
      series_tag: row.series_tag,
      is_watched: row.is_watched,
      last_position: row.last_position,
      is_favorite: row.is_favorite,
    }));

    mediaItems.set(items);
    libraryLoadState.set('done');
  } catch (e) {
    console.error('[MediaStore] Failed to load library from Rust backend:', e);
    libraryLoadState.set('error');
  }
}

/**
 * Toggles favorite status for a media item via the Rust command.
 * Optimistically updates the store first.
 */
export async function toggleFavorite(path: string) {
  const items = get(mediaItems);
  const index = items.findIndex(i => i.path === path);
  if (index === -1) return;

  const currentFavorite = items[index].is_favorite;
  
  // 1. Optimistic Update
  items[index] = { ...items[index], is_favorite: !currentFavorite };
  mediaItems.set([...items]);

  // 2. Persist
  try {
    const isFavoriteNow = await invoke<boolean>('toggle_favorite_status', { path });
    
    // Ensure store is in sync with server truth
    items[index].is_favorite = isFavoriteNow;
    mediaItems.set([...items]);

    // Toast for feedback
    window.dispatchEvent(new CustomEvent('flux-toast', { 
      detail: { 
        label: isFavoriteNow ? 'Added to Favorites' : 'Removed from Favorites', 
        icon: 'star' 
      } 
    }));
  } catch (e) {
    console.error('[MediaStore] Failed to update favorite in DB:', e);
    // Back out optimistic update on fail
    items[index].is_favorite = currentFavorite;
    mediaItems.set([...items]);
  }
}

/**
 * Updates the playback position locally in the store.
 * Used for optimistic UI updates (e.g., progress bars) in the library.
 */
export function updateLocalProgress(path: string, position: number) {
  mediaItems.update(items => {
    const item = items.find(i => i.path === path);
    if (item) {
      item.last_position = position;
    }
    return [...items];
  });
}
