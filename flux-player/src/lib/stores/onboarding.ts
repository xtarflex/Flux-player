import { writable, get } from 'svelte/store';

/**
 * Onboarding Sections define the different parts of the tour.
 */
export type OnboardingSection = 'welcome' | 'global' | 'library' | 'player';

export interface OnboardingState {
  isActive: boolean;
  currentSection: OnboardingSection | null;
  currentStep: number;
  completedSections: OnboardingSection[];
}

const DEFAULT_STATE: OnboardingState = {
  isActive: false,
  currentSection: null,
  currentStep: 0,
  completedSections: [],
};

// ── Persistence Logic ────────────────────────────────────────────────────────

function loadOnboardingState(): OnboardingState {
  if (typeof window === 'undefined') return DEFAULT_STATE;
  const saved = localStorage.getItem('flux_onboarding');
  if (!saved) return DEFAULT_STATE;
  try {
    const parsed = JSON.parse(saved);
    return { ...DEFAULT_STATE, completedSections: parsed.completedSections || [] };
  } catch (e) {
    console.error('[Onboarding] Failed to parse saved state:', e);
    return DEFAULT_STATE;
  }
}

function saveOnboardingState(state: OnboardingState) {
  if (typeof window === 'undefined') return;
  // We only persist the completed sections
  localStorage.setItem('flux_onboarding', JSON.stringify({
    completedSections: state.completedSections
  }));
}

// ── The Store ────────────────────────────────────────────────────────────────

export const onboarding = writable<OnboardingState>(loadOnboardingState());

if (typeof window !== 'undefined') {
  onboarding.subscribe(val => saveOnboardingState(val));
}

// ── Actions ──────────────────────────────────────────────────────────────────

/**
 * Checks if a section needs to be started and begins it if so.
 * This should be called defensively on route load.
 */
export function triggerTour(section: OnboardingSection) {
  const state = get(onboarding);
  if (state.completedSections.includes(section) || state.isActive) {
    return; // Already done or another tour is active
  }
  
  onboarding.update(s => ({
    ...s,
    isActive: true,
    currentSection: section,
    currentStep: 0
  }));
}

/**
 * Advances to the next semantic step in the current section.
 */
export function nextTourStep() {
  onboarding.update(s => ({ ...s, currentStep: s.currentStep + 1 }));
}

/**
 * Completes the current section, hiding the overlay and saving progress.
 */
export function completeTourSection() {
  onboarding.update(s => {
    if (!s.currentSection) return s;
    const newCompleted = [...s.completedSections, s.currentSection];
    // deduplicate just in case
    const uniqueCompleted = Array.from(new Set(newCompleted));
    
    return {
      ...s,
      isActive: false,
      currentSection: null,
      currentStep: 0,
      completedSections: uniqueCompleted
    };
  });
}

/**
 * Forcibly skips the current sequence.
 */
export function skipTour() {
  completeTourSection();
}

/**
 * Development / Settings reset
 */
export function resetOnboarding() {
  onboarding.set(DEFAULT_STATE);
  saveOnboardingState(DEFAULT_STATE);
}
