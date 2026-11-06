// @ts-nocheck
import { expect, test, describe, beforeEach } from "bun:test";
import { get } from "svelte/store";
import { playbackState, activeMedia, togglePlayback } from "./playback";
import type { MediaItem } from "./media";

const mockMedia: MediaItem = {
  id: "test-id",
  path: "/path/to/media",
  title: "Test Media",
  artist: "Test Artist",
  album: "Test Album",
  duration: 100,
  year: 2024,
  type: "video",
  last_played: 0,
  added_at: Date.now(),
};

describe("playback store: togglePlayback", () => {
  beforeEach(() => {
    // Reset stores
    activeMedia.set(null);
    playbackState.update(s => ({ ...s, isPlaying: false }));
  });

  test("should toggle isPlaying from false to true when activeMedia is set", () => {
    activeMedia.set(mockMedia);
    expect(get(playbackState).isPlaying).toBe(false);

    togglePlayback();

    expect(get(playbackState).isPlaying).toBe(true);
  });

  test("should toggle isPlaying from true to false when activeMedia is set", () => {
    activeMedia.set(mockMedia);
    playbackState.update(s => ({ ...s, isPlaying: true }));
    expect(get(playbackState).isPlaying).toBe(true);

    togglePlayback();

    expect(get(playbackState).isPlaying).toBe(false);
  });

  test("should NOT toggle isPlaying when activeMedia is NOT set", () => {
    activeMedia.set(null);
    expect(get(playbackState).isPlaying).toBe(false);

    togglePlayback();

    expect(get(playbackState).isPlaying).toBe(false);
  });
});
