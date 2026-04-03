/**
 * @file queue.ts
 * @description Session-based playback queue for Flux. 
 * Manages the list of upcoming tracks and provides navigation logic.
 */

import { writable, get } from 'svelte/store';
import { type MediaItem, mediaItems } from './media';
import { settings } from './settings';
import { activeMedia, playbackState } from './playback';
import { goto } from '$app/navigation';

export interface QueueState {
  items: MediaItem[];
  originalItems: MediaItem[]; // The pre-shuffled order
  index: number;
}

const initialState: QueueState = {
  items: [],
  originalItems: [],
  index: -1,
};

export const queue = writable<QueueState>(initialState);

// ── Core Actions ─────────────────────────────────────────────────────────────

/**
 * Replaces the entire queue with a new set of items.
 * @param items - The new track list.
 * @param startIndex - The index to start playback at.
 * @param seekTo - Optional position in seconds to start from.
 */
export function setQueue(items: MediaItem[], startIndex = 0, seekTo?: number) {
  const targetItem = items[startIndex];
  const isVideo = targetItem?.type === 'video' || targetItem?.type === 'mixed';
  
  queue.set({ items, originalItems: [...items], index: startIndex });
  if (targetItem) {
    activeMedia.set(targetItem);
    playbackState.update(s => ({ 
      ...s, 
      isPlaying: true,
      seekTo: seekTo ?? 0,
      isTheaterMode: isVideo,
      isMiniPlayer: false
    }));

    // Auto-navigate for Video
    if (isVideo) {
      goto('/playing');
    }
  }
}

/**
 * Adds an item to play immediately after the current track.
 */
export function addNext(item: MediaItem) {
  queue.update(state => {
    const newItems = [...state.items];
    newItems.splice(state.index + 1, 0, item);
    return { ...state, items: newItems };
  });
  
  window.dispatchEvent(new CustomEvent('flux-toast', { 
    detail: { label: 'Added to Play Next', icon: 'skip-next' } 
  }));
}

/**
 * Appends an item to the end of the queue.
 */
export function addToEnd(item: MediaItem) {
  queue.update(state => ({
    ...state,
    items: [...state.items, item]
  }));

  window.dispatchEvent(new CustomEvent('flux-toast', { 
    detail: { label: 'Added to Queue', icon: 'playlist-add' } 
  }));
}

/**
 * Advances to the next track in the queue.
 * Respects repeatMode: 0 (Off), 1 (Repeat All).
 * Note: repeatMode 2 (Repeat One) is handled by the engine looping.
 */
export function nextTrack() {
  const state = get(queue);
  const { repeatMode, shuffleState } = get(playbackState);
  
  if (state.items.length === 0) return;

  let nextIndex = state.index + 1;
  
  // Logic: End of Queue handling (Loop for ANY repeat mode)
  if (nextIndex >= state.items.length) {
    if (repeatMode === 1 || repeatMode === 2) {
      nextIndex = 0; // Loop to start
    } else {
      // End of queue reached (repeat off)
      activeMedia.set(null);
      playbackState.update(s => ({ ...s, isPlaying: false, progress: 0 }));
      return;
    }
  }

  queue.update(s => ({ ...s, index: nextIndex }));
  const nextMedia = state.items[nextIndex];
  if (nextMedia) {
    activeMedia.set(nextMedia);
    // Note: If repeatMode is 2 (Repeat One), the engine handles the loop, 
    // but if the user MANUALLY clicks "Next", we still advance.
    playbackState.update(s => ({ ...s, isPlaying: true, progress: 0 }));
  }
}

/**
 * Returns to the previous track or starts the current track over.
 */
export function prevTrack() {
  const state = get(queue);
  const playState = get(playbackState);
  const currentMedia = get(activeMedia);
  
  // If we've played more than 3 seconds, just restart the current track
  const currentPos = (currentMedia?.duration || 0) * playState.progress;
  if (currentPos > 3) {
    playbackState.update(s => ({ ...s, seekProgressRequest: 0 }));
    return;
  }

  let prevIndex = state.index - 1;
  
  if (prevIndex < 0) {
    if (get(playbackState).repeatMode > 0) {
      prevIndex = state.items.length - 1; // Loop to end
    } else {
      // Start of queue reached, just restart first track
      playbackState.update(s => ({ ...s, seekProgressRequest: 0 }));
      return;
    }
  }

  queue.update(s => ({ ...s, index: prevIndex }));
  const prevMedia = state.items[prevIndex];
  if (prevMedia) {
    activeMedia.set(prevMedia);
    playbackState.update(s => ({ ...s, isPlaying: true, progress: 0 }));
  }
}

/**
 * Toggles shuffle mode and reorders the queue.
 */
export function toggleShuffle(enabled: boolean) {
  queue.update(state => {
    if (state.items.length <= 1) return state;

    if (enabled) {
      // SHUFFLE: Keep current item, randomize the rest
      const current = state.items[state.index];
      const others = state.items.filter((_, i) => i !== state.index);
      
      // Fisher-Yates Shuffle
      for (let i = others.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [others[i], others[j]] = [others[j], others[i]];
      }

      return {
        ...state,
        items: [current, ...others],
        index: 0
      };
    } else {
      // UNSHUFFLE: Restore original order
      const current = state.items[state.index];
      const newIndex = state.originalItems.findIndex(i => i.path === current.path);
      return {
        ...state,
        items: [...state.originalItems],
        index: newIndex
      };
    }
  });
}

/**
 * Clears the session queue.
 */
export function clearQueue() {
  queue.set(initialState);
}

// ── Smart Logic ─────────────────────────────────────────────────────────────

/**
 * Implements the "Smart" auto-queue logic.
 * If the item is part of a series or album, it builds a queue from related items.
 * @param targetItem - The item to play.
 * @param seekTo - Optional resume position.
 * @param contextList - Optional list of items to use as the "Always" context (e.g. filtered library).
 */
export function playWithAutoQueue(targetItem: MediaItem, seekTo?: number, contextList?: MediaItem[]) {
  const mode = get(settings).autoQueueMode;
  const allMedia = get(mediaItems);

  if (mode === 'Never') {
    setQueue([targetItem], 0, seekTo);
    return;
  }

  if (mode === 'Always') {
    // Queue everything in context (or all library if no context provided)
    const baseList = contextList || allMedia;
    const sorted = [...baseList].sort((a, b) => a.path.localeCompare(b.path, undefined, { numeric: true }));
    const idx = sorted.findIndex(i => i.path === targetItem.path);
    
    // If item not in context, fallback to single item or Smart? 
    // Usually it should be in context, but safety first:
    if (idx === -1) {
      setQueue([targetItem], 0, seekTo);
    } else {
      setQueue(sorted, idx, seekTo);
    }
    return;
  }

  // mode === 'Smart'
  let smartList: MediaItem[] = [targetItem];
  let startIndex = 0;

  if (targetItem.series_tag) {
    // It's a series - identify the "Show Name" by stripping the tag from the title
    // Example: "The Simpsons - S01E01" -> "The Simpsons"
    const showName = targetItem.title.replace(` - ${targetItem.series_tag}`, '').trim();
    
    smartList = allMedia
      .filter(i => {
        if (!i.series_tag) return false;
        // Match items that either share the same series_tag (redundant but safe) 
        // OR share the same show name prefix in the title.
        return i.title.startsWith(showName);
      })
      .sort((a, b) => a.path.localeCompare(b.path, undefined, { numeric: true }));
    
    startIndex = smartList.findIndex(i => i.path === targetItem.path);
  } else if (targetItem.album) {
    // It's an album - find all items with the same album
    smartList = allMedia
      .filter(i => i.album === targetItem.album)
      .sort((a, b) => a.path.localeCompare(b.path, undefined, { numeric: true }));
    startIndex = smartList.findIndex(i => i.path === targetItem.path);
  }

  setQueue(smartList, startIndex, seekTo);
}
