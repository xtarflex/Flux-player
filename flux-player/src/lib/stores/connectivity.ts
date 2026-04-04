/**
 * @file connectivity.ts
 * @description Centralized network connectivity store for Flux Player.
 * Tracks online/offline status, intentional offline mode, and connection quality.
 */

import { derived, writable } from 'svelte/store';
import { settings } from './settings';

export type ConnectivityStatus = 'online' | 'weak' | 'error' | 'offline';

const isBrowser = typeof window !== 'undefined';

/**
 * Internal store for the browser's raw online status.
 */
function createOnlineStore() {
  const online = writable(isBrowser ? window.navigator.onLine : true);

  if (isBrowser) {
    window.addEventListener('online', () => online.set(true));
    window.addEventListener('offline', () => online.set(false));
  }

  return online;
}

export const onlineStatus = createOnlineStore();

/**
 * Derived connectivity status based on environment and user settings.
 */
export const connectivity = derived<[typeof onlineStatus, typeof settings], ConnectivityStatus>(
  [onlineStatus, settings],
  ([$online, $settings]) => {
    // 1. Intentional Offline Mode (highest priority)
    if ($settings.offlineMode) return 'offline';

    // 2. Technical Connection Loss
    if (!$online) return 'error';
    
    // 3. Weak Connection Heuristic
    if (isBrowser && (navigator as any).connection) {
      const conn = (navigator as any).connection;
      // 'slow-2g' and '2g' are considered weak connections for media streaming
      if (conn.effectiveType === '2g' || conn.effectiveType === 'slow-2g') {
        return 'weak';
      }
      
      // Alternative: check downlink speed if available
      if (conn.downlink && conn.downlink < 1.5) {
        return 'weak';
      }
    }
    
    return 'online';
  }
);

/**
 * Returns the human-readable label and associated icon name for a status.
 */
export function getConnectivityDetails(status: ConnectivityStatus) {
  switch (status) {
    case 'online':
      return { label: 'STABLE', icon: 'network-online', color: 'var(--secondary)' };
    case 'weak':
      return { label: 'WEAK', icon: 'network-weak', color: '#ffcc00' };
    case 'error':
      return { label: 'NO CONNECTION', icon: 'network-error', color: '#ff0000' };
    case 'offline':
      return { label: 'OFFLINE MODE', icon: 'network-offline', color: 'var(--text-muted)' };
  }
}
