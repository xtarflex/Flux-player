
import { test, expect, describe, beforeEach } from "bun:test";
import { get, writable } from "./shims";
import { updateLocalProgress, mediaItems } from "./media";
import { settings } from "./settings";

describe("updateLocalProgress", () => {
  beforeEach(() => {
    mediaItems.set([
      {
        path: "test-video.mp4",
        duration: 100,
        last_position: 0,
        is_watched: false
      } as any
    ]);
    settings.set({ watchedThreshold: 90 } as any);
  });

  test("should update last_position for a valid item", () => {
    updateLocalProgress("test-video.mp4", 50);
    const items = get(mediaItems);
    expect(items[0].last_position).toBe(50);
    expect(items[0].is_watched).toBe(false);
  });

  test("should mark as watched when crossing threshold forward (89 -> 91 with 90 threshold)", () => {
    // Set initial position just below threshold
    mediaItems.update(items => {
      items[0].last_position = 89;
      return items;
    });

    updateLocalProgress("test-video.mp4", 91);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(true);
    expect(items[0].last_position).toBe(91);
  });

  test("should NOT mark as watched if already watched even if moving backward", () => {
    mediaItems.update(items => {
      items[0].last_position = 95;
      items[0].is_watched = true;
      return items;
    });

    updateLocalProgress("test-video.mp4", 50);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(true);
    expect(items[0].last_position).toBe(50);
  });

  test("should NOT mark as watched if moving backward across threshold (95 -> 85)", () => {
    mediaItems.update(items => {
      items[0].last_position = 95;
      items[0].is_watched = false; // Manually unwatched
      return items;
    });

    updateLocalProgress("test-video.mp4", 85);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(false);
  });

  test("should mark as watched if forcedWatched is true", () => {
    updateLocalProgress("test-video.mp4", 10, true);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(true);
  });

  test("should respect a different threshold (e.g. 50%)", () => {
    settings.set({ watchedThreshold: 50 } as any);

    updateLocalProgress("test-video.mp4", 51);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(true);
  });

  test("should handle item not found gracefully", () => {
    const initialItems = get(mediaItems);
    updateLocalProgress("non-existent.mp4", 50);
    expect(get(mediaItems)).toEqual(initialItems);
  });

  test("should handle item with null duration gracefully", () => {
    mediaItems.update(items => {
      items[0].duration = null;
      return items;
    });

    updateLocalProgress("test-video.mp4", 50);
    const items = get(mediaItems);
    expect(items[0].last_position).toBe(0); // Should not update if duration is null according to code
  });

  test("should mark as watched if threshold is 0 and any progress is made", () => {
    settings.set({ watchedThreshold: 0 } as any);
    updateLocalProgress("test-video.mp4", 1);
    const items = get(mediaItems);
    expect(items[0].is_watched).toBe(true);
  });

  test("should only mark as watched if threshold is crossed (not if already above)", () => {
      // Logic: !prevWatched && (forcedWatched || (prevPos < thresholdPos && position >= thresholdPos))
      mediaItems.update(items => {
          items[0].last_position = 92;
          items[0].is_watched = false;
          return items;
      });

      updateLocalProgress("test-video.mp4", 95);
      const items = get(mediaItems);
      expect(items[0].is_watched).toBe(false); // Because prevPos (92) was NOT < thresholdPos (90)
  });
});
