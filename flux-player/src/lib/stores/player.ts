/**
 * @file player.ts
 * @description Svelte store for current media playback state.
 */
import { writable } from 'svelte/store';
import type { MediaItem } from './media';

export interface PlayerState {
  activeItem: MediaItem | null;
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  isBuffering: boolean;
  volume: number;
  isMuted: boolean;
  isFullscreen: boolean;
}

const initialState: PlayerState = {
  activeItem: null,
  isPlaying: false,
  currentTime: 0,
  duration: 0,
  isBuffering: false,
  volume: 1,
  isMuted: false,
  isFullscreen: false
};

export const playerStore = writable<PlayerState>(initialState);

/**
 * Commands to manipulate the player state
 */
export const playerActions = {
  play: (item: MediaItem) => {
    playerStore.update(s => ({ 
      ...s, 
      activeItem: item, 
      isPlaying: true, 
      currentTime: item.last_position || 0 
    }));
  },
  
  pause: () => {
    playerStore.update(s => ({ ...s, isPlaying: false }));
  },
  
  toggle: () => {
    playerStore.update(s => ({ ...s, isPlaying: !s.isPlaying }));
  },
  
  stop: () => {
    playerStore.set(initialState);
  },
  
  updateTime: (seconds: number) => {
    playerStore.update(s => ({ ...s, currentTime: seconds }));
  },
  
  setDuration: (seconds: number) => {
    playerStore.update(s => ({ ...s, duration: seconds }));
  },
  
  setVolume: (level: number) => {
    playerStore.update(s => ({ ...s, volume: level }));
  }
};
