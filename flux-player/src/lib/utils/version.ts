/**
 * @file version.ts
 * @description Lightweight utility for checking for beta updates via GitHub.
 */

import { hasUpdateAvailable } from '../stores/ui';
import { get } from 'svelte/store';

const GITHUB_PACKAGE_URL = 'https://raw.githubusercontent.com/xtarflex/flux-player/main/flux-player/package.json';

/**
 * Fetches the latest package.json from GitHub and compares versions.
 */
export async function checkForUpdates(): Promise<void> {
  try {
    const response = await fetch(GITHUB_PACKAGE_URL, { cache: 'no-cache' });
    if (!response.ok) return;

    const remotePackage = await response.json();
    const remoteVersion = remotePackage.version; // e.g. "0.2.0-beta.2"
    const localVersion = import.meta.env.VITE_APP_VERSION || '0.0.1';

    if (isNewer(remoteVersion, localVersion)) {
      console.log(`[VersionCheck] New version found: ${remoteVersion} (Local: ${localVersion})`);
      hasUpdateAvailable.set(true);
    } else {
      console.log(`[VersionCheck] On latest version: ${localVersion}`);
      hasUpdateAvailable.set(false);
    }
  } catch (e) {
    console.warn('[VersionCheck] Failed to check for updates:', e);
  }
}

/**
 * Simple semver comparison for Beta tags.
 * Handles "0.2.0-beta.1" < "0.2.0-beta.2"
 */
function isNewer(remote: string, local: string): boolean {
  if (remote === local) return false;

  const parse = (v: string) => {
    const [base, beta] = v.split('-');
    const nums = base.split('.').map(Number);
    const betaNum = beta ? parseInt(beta.replace('beta.', '')) : 999; // Non-beta is newer than beta
    return [...nums, betaNum];
  };

  const r = parse(remote);
  const l = parse(local);

  for (let i = 0; i < r.length; i++) {
    if (r[i] > l[i]) return true;
    if (r[i] < l[i]) return false;
  }

  return false;
}
