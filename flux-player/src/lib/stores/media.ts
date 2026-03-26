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
  last_position: number;    // seconds
  is_watched: boolean;
  // UI-only enriched fields (populated from TMDB later)
  rating?: number;
  genres?: string[];
  synopsis?: string;
  director?: string;
  starring?: string;
  subtitle?: string;
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
      'SELECT path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, last_played, added_at, last_position, is_watched FROM media ORDER BY added_at DESC'
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
      last_position: row.last_position ?? 0,
      is_watched: !!row.is_watched,
    }));

    mediaItems.set(items);
    libraryLoadState.set('done');
  } catch (e) {
    console.error('[MediaStore] Failed to load library from DB:', e);
    libraryLoadState.set('error');
  }
}

/**
 * Updates the playback progress for a specific media item.
 */
export async function updatePlaybackProgress(path: string, seconds: number, isWatched: boolean): Promise<void> {
  try {
    const db = await Database.load('sqlite:flux.db');
    await db.execute(
      'UPDATE media SET last_position = ?, is_watched = ?, last_played = ? WHERE path = ?',
      [seconds, isWatched ? 1 : 0, Math.floor(Date.now() / 1000), path]
    );

    // Update local store to avoid a full re-query
    mediaItems.update(items => items.map(item => {
      if (item.path === path) {
        return { ...item, last_position: seconds, is_watched: isWatched, last_played: Math.floor(Date.now() / 1000) };
      }
      return item;
    }));
  } catch (e) {
    console.error('[MediaStore] Failed to update playback progress:', e);
  }
}
