/**
 * @file media.ts
 * @description Svelte store for the media library. On first load, fetches all 
 * media items from the SQLite database via plugin-sql. After a library scan 
 * completes (triggered by the `flux-library-updated` event), it refreshes 
 * automatically.
 */

import { writable, get } from 'svelte/store';
import Database from '@tauri-apps/plugin-sql';

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
  // UI-only enriched fields (populated from TMDB later)
  rating?: number;
  genres?: string[];
  synopsis?: string;
  director?: string;
  starring?: string;
  subtitle?: string;
  series_tag?: string | null;
  is_watched?: boolean;
  /** Last playback position in seconds (0 = unwatched / from start) */
  last_position?: number;
  is_favorite?: boolean;
}

export type LibraryLoadState = 'idle' | 'loading' | 'done' | 'error';

export const mediaItems = writable<MediaItem[]>([]);
export const selectedMediaId = writable<string | null>(null);
export const libraryLoadState = writable<LibraryLoadState>('idle');
export const isScanning = writable<boolean>(false);

/**
 * Queries all rows from the `media` table in flux.db and updates the store.
 * Maps SQLite column names to the MediaItem interface.
 */
export async function loadLibraryFromDb(): Promise<void> {
  libraryLoadState.set('loading');
  try {
    const db = await Database.load('sqlite:flux.db');
    const rows = await db.select<any[]>(
      'SELECT path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, last_played, added_at, synopsis, rating, genres, director, starring, series_tag, is_watched, last_position, is_favorite FROM media ORDER BY added_at DESC'
    );

    const items: MediaItem[] = rows.map((row) => ({
      id: row.path,
      path: row.path,
      title: row.title,
      artist: row.artist ?? null,
      album: row.album ?? null,
      duration: row.duration ?? null,
      poster: row.poster_path ?? null,
      backdrop: row.backdrop_path ?? null,
      album_art: row.album_art_path ?? null,
      year: row.year ?? null,
      type: (row.media_type as 'video' | 'audio' | 'mixed') ?? 'video',
      last_played: row.last_played ?? 0,
      added_at: row.added_at ?? 0,
      synopsis: row.synopsis ?? null,
      rating: row.rating ?? null,
      genres: row.genres ? JSON.parse(row.genres) : [],
      director: row.director ?? null,
      starring: row.starring ?? null,
      series_tag: row.series_tag ?? null,
      is_watched: row.is_watched === 1 || row.is_watched === true,
      last_position: row.last_position ?? 0,
      is_favorite: row.is_favorite === 1 || row.is_favorite === true,
    }));

    mediaItems.set(items);
    libraryLoadState.set('done');
  } catch (e) {
    console.error('[MediaStore] Failed to load library from DB:', e);
    libraryLoadState.set('error');
  }
}

/**
 * Toggles favorite status for a media item.
 * Optimistically updates the store first, then persists to DB.
 */
export async function toggleFavorite(path: string) {
  const items = get(mediaItems);
  const index = items.findIndex(i => i.path === path);
  if (index === -1) return;

  const newState = !items[index].is_favorite;
  
  // 1. Optimistic Update
  items[index] = { ...items[index], is_favorite: newState };
  mediaItems.set([...items]);

  // 2. Persist
  try {
    const db = await Database.load('sqlite:flux.db');
    await db.execute('UPDATE media SET is_favorite = ? WHERE path = ?', [newState ? 1 : 0, path]);
    
    // Toast for feedback
    window.dispatchEvent(new CustomEvent('flux-toast', { 
      detail: { 
        label: newState ? 'Added to Favorites' : 'Removed from Favorites', 
        icon: 'star' 
      } 
    }));
  } catch (e) {
    console.error('[MediaStore] Failed to update favorite in DB:', e);
    // Back out optimistic update on fail
    items[index] = { ...items[index], is_favorite: !newState };
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
