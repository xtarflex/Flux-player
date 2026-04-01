import { writable } from 'svelte/store';

export type Placement = 'top' | 'bottom' | 'left' | 'right';

export interface TooltipState {
  visible: boolean;
  content: string;
  shortcut?: string;
  secondaryContent?: string;
  secondaryShortcut?: string;
  x: number;
  y: number;
  placement: Placement;
}

export const tooltipState = writable<TooltipState>({
  visible: false,
  content: '',
  x: 0,
  y: 0,
  placement: 'bottom'
});
