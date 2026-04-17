/**
 * @file settings.ts
 * @description Persistent global settings store for Flux Player.
 * Uses localStorage for simple frontend persistence of UI/UX preferences.
 */

import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export type AutoQueueMode = 'Never' | 'Smart' | 'Always';
export type TransitionBehavior = 'Return to Library' | 'Stay in Now Playing';

export interface FluxSettings {
  autoQueueMode: AutoQueueMode;
  allowMixedQueue: boolean;
  transitionBehavior: TransitionBehavior;
  volume: number;
  isMuted: boolean;
  playbackSpeed: number;
  hwAcceleration: boolean;
  ffmpegThreading: string;
  watchedThreshold: number;
  subtitleLanguage: string;
  subtitleFuzzy: boolean;
  subtitleAutoFetch: boolean;
  osUsername: string;
  offlineMode: boolean; // Intentional offline state
  videoAudioTransition: boolean; 
  theme: string;
  language: string;
}

const DEFAULT_SETTINGS: FluxSettings = {
  autoQueueMode: 'Smart',
  allowMixedQueue: true,
  transitionBehavior: 'Return to Library',
  volume: 0.7,
  isMuted: false,
  playbackSpeed: 1.0,
  hwAcceleration: true,
  ffmpegThreading: 'Auto',
  watchedThreshold: 90,
  subtitleLanguage: 'System Default',
  subtitleFuzzy: true,
  subtitleAutoFetch: true,
  osUsername: 'User',
  offlineMode: false,
  videoAudioTransition: true,
  theme: 'Cyber Dark',
  language: 'System Default',
};

// ── Persistence Logic ────────────────────────────────────────────────────────

function loadSettings(): FluxSettings {
  if (typeof window === 'undefined') return DEFAULT_SETTINGS;
  const saved = localStorage.getItem('flux_settings');
  if (!saved) return DEFAULT_SETTINGS;
  try {
    return { ...DEFAULT_SETTINGS, ...JSON.parse(saved) };
  } catch (e) {
    console.error('[Settings] Failed to parse saved settings:', e);
    return DEFAULT_SETTINGS;
  }
}

function saveSettings(settings: FluxSettings) {
  if (typeof window === 'undefined') return;
  localStorage.setItem('flux_settings', JSON.stringify(settings));
}

/**
 * Syncs specific settings to the backend SQLite database for consistency 
 * with core engine logic (e.g. watchedThreshold).
 */
async function syncToBackend(key: string, value: any) {
  try {
    // Only sync keys that the backend actually uses
    const syncableKeys = ['watchedThreshold', 'autoIndexing', 'hwAcceleration', 'ffmpegThreading', 'autoQueueMode'];
    if (syncableKeys.includes(key)) {
      await invoke('save_setting', { key, value: String(value) });
    }
  } catch (e) {
    console.warn(`[SettingsSync] Failed to sync ${key}:`, e);
  }
}

// ── The Store ────────────────────────────────────────────────────────────────

export const settings = writable<FluxSettings>(loadSettings());

// Subscribe to changes and save to both localStorage AND backend SQLite
if (typeof window !== 'undefined') {
  let lastVal: FluxSettings | null = loadSettings();
  
  settings.subscribe(val => {
    saveSettings(val);
    
    // Perform differential sync to avoid redundant backend calls
    for (const k in val) {
      const key = k as keyof FluxSettings;
      if (lastVal && lastVal[key] !== val[key]) {
        syncToBackend(key, val[key]);
      }
    }
    lastVal = { ...val };
  });
}

/**
 * Updates a specific setting key.
 * @param key - The setting key to update.
 * @param value - The new value.
 */
export function updateSetting<K extends keyof FluxSettings>(key: K, value: FluxSettings[K]) {
  settings.update(s => ({ ...s, [key]: value }));
}
