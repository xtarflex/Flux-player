import { writable } from 'svelte/store';
import type { MenuItem } from '../components/ui/context-menu';

export interface ActiveMenu {
  x: number;
  y: number;
  items: MenuItem[];
}

export const activeMenu = writable<ActiveMenu | null>(null);
export const isSidebarCollapsed = writable<boolean>(false);

/**
 * Opens a global context menu at the specified coordinates.
 * Renders at the root layout to avoid stacking context issues.
 */
export function openMenu(x: number, y: number, items: MenuItem[]) {
  activeMenu.set({ x, y, items });
}

/**
 * Closes the currently active global context menu.
 */
export function closeMenu() {
  activeMenu.set(null);
}
