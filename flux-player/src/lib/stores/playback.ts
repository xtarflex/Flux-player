/**
 * @file playback.ts
 * @description Global playback state store for Flux Player.
 * Manages active media, playback state, theater mode, mini-player, and seek targets.
 * The PlayerEngine.svelte component subscribes to this store and drives Video.js accordingly.
 */

import { writable, get } from 'svelte/store';
import { goto } from '$app/navigation';
import type { MediaItem } from './media';

// ── Re-export MediaItem so consumers can import from one place ──────────────
export type { MediaItem };

// ── Interfaces ──────────────────────────────────────────────────────────────

export interface PlaybackState {
  /** Whether audio/video is currently playing */
  isPlaying: boolean;
  /** Progress ratio 0–1 (used by scrubber; actual time comes from Video.js) */
  progress: number;
  /** Volume level 0–1 */
  volume: number;
  isMuted: boolean;
  /** Playback speed multiplier (0.5 – 2.0) */
  speed: number;
  isPiP: boolean;
  isFullscreen: boolean;
  /** Theater mode: sidebar/footer become floating glass overlays */
  isTheaterMode: boolean;
  /** Mini-player: video shrinks to 320×180 overlay so the user can browse */
  isMiniPlayer: boolean;
  /**
   * Seek-to target in seconds. Set before routing to /playing.
   * PlayerEngine reads this once on load then nulls it out.
   */
  seekTo: number | null;
  /**
   * manual seek request by percentage (0 to 1) from the UI scrubber.
   */
  seekProgressRequest: number | null;
  /**
   * 0: off, 1: all, 2: one
   */
  repeatMode: number;
  /**
   * Whether sequence should play randomly
   */
  shuffleState: boolean;
  /**
   * Whether the user is currently idle (mouse hasn't moved)
   */
  isIdle: boolean;
  /**
   * Internal implementation signals
   */
  fullscreenRequest: boolean | null;
  pipRequest: boolean | null;
}

// ── Stores ───────────────────────────────────────────────────────────────────

/** The media item currently loaded into the player engine */
export const activeMedia = writable<MediaItem | null>(null);

export const playbackState = writable<PlaybackState>({
  isPlaying: false,
  progress: 0,
  volume: 0.7,
  isMuted: false,
  speed: 1,
  isPiP: false,
  isFullscreen: false,
  isTheaterMode: false,
  isMiniPlayer: false,
  seekTo: null,
  seekProgressRequest: null,
  repeatMode: 0,
  shuffleState: false,
  isIdle: false,
  fullscreenRequest: null,
  pipRequest: null,
});

/**
 * Holds a reference to the live Video.js player instance.
 * Set by PlayerEngine.svelte on mount; used by MiniPlayer.svelte to share the instance.
 */
export const playerEngineRef = writable<any | null>(null);

// ── Actions ──────────────────────────────────────────────────────────────────

/**
 * Sets the active media without starting playback (hydrates the footer).
 * @param item - The MediaItem to preview.
 */
export function setMedia(item: MediaItem) {
  activeMedia.set(item);
  playbackState.update(s => ({ ...s, isPlaying: false, progress: 0, seekTo: null }));
}

/**
 * Initiates full playback of a media item.
 * — Audio: plays in background, user stays on current route (UX Journey §5).
 * — Video: routes to /playing in theater mode.
 *
 * @param item - The MediaItem to play.
 * @param startSeconds - Optional resume position in seconds (default: 0).
 */
export function playMediaFromItem(item: MediaItem, startSeconds: number = 0) {
  const isVideo = item.type === 'video' || item.type === 'mixed';

  activeMedia.set(item);
  playbackState.update(s => ({
    ...s,
    isPlaying: true,
    progress: 0,
    seekTo: startSeconds > 0 ? startSeconds : null,
    isTheaterMode: isVideo,
    isMiniPlayer: false,
  }));

  // Audio: DO NOT navigate — user stays in the Library.
  // The persistent AudioEngine in +layout.svelte handles playback.
  if (isVideo) {
    goto('/playing');
  }
}

/**
 * Toggles play/pause. No-ops if there is no active media.
 */
export function togglePlayback() {
  playbackState.update(s => {
    if (!get(activeMedia)) return s;
    return { ...s, isPlaying: !s.isPlaying };
  });
}

/**
 * Activates the Mini-Player, shrinking the video into the overlay.
 * Called by the layout's beforeNavigate interceptor.
 */
export function activateMiniPlayer() {
  playbackState.update(s => ({
    ...s,
    isMiniPlayer: true,
    isTheaterMode: false,
  }));
}

/**
 * Deactivates the Mini-Player and restores theater mode for video.
 * Called when the user clicks the mini-player to return to /playing.
 */
export function deactivateMiniPlayer() {
  const media = get(activeMedia);
  const isVideo = media?.type === 'video' || media?.type === 'mixed';
  playbackState.update(s => ({
    ...s,
    isMiniPlayer: false,
    isTheaterMode: isVideo,
  }));
}
