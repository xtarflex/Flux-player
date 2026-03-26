<script lang="ts">
  /**
   * Icon.svelte
   * Centralized SVG Icon component for Flux.
   * Provides consistent weight, sizing, and brand-tailored styling across the app.
   *
   * Usage:
   * Use this component for all static, simple icons to ensure consistent aesthetics.
   * For complex animated navigation icons, use the `Animated*.svelte` components
   * in `src/lib/components/ui/animated-icons/`.
   */
  let { name, size = 20, strokeWidth = 2, fill = "none", class: className = "" } = $props<{
    name: string;
    size?: number | string;
    strokeWidth?: number;
    fill?: string;
    class?: string;
  }>();

  // Icon Path Dictionary (Using Lucide-inspired geometric paths)
  const icons: Record<string, string[]> = {
    // Navigation (Static Fallbacks)
    discovery: ['M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z', 'M9 12l2 2 4-4'],
    library: ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z'],
    playing: ['M12 2v20 M12 2a10 10 0 0 0-10 10 M12 22a10 10 0 0 0 10-10 M18 8l-8 4 8 4V8z'],
    playlists: ['M21 15V6', 'M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z', 'M12 12H3', 'M16 6H3', 'M12 18H3'], // New music note + lines list
    settings: ['M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z', 'M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z'],

    // Actions
    refresh: ['M21 2v6h-6', 'M3 12a9 9 0 0 1 15-6.7L21 8', 'M3 22v-6h6', 'M15 16.7A9 9 0 0 1 3 12', 'M15 16.7L3 16'], // Better refresh icon matching Titlebar
    search: ['M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16z', 'M21 21l-4.35-4.35'],
    'new-playlist': ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v11z', 'M12 11v6', 'M9 14h6'],
    import: ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v11z', 'M12 11v6', 'M9 14h6'],
    save: ['M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z', 'M17 21v-8H7v8', 'M7 3v5h8'],
    screenshot: ['M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z', 'M12 17a4 4 0 1 0 0-8 4 4 0 0 0 0 8z'],
    star: ['M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z'],
    check: ['M20 6L9 17l-5-5'],
    menu: ['M3 12h18M3 6h18M3 18h18'], // Added from Sidebar

    // View Toggles
    'list-view': ['M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01'], 
    'grid-view': ['M7 3H3v4h4V3zM14 3h-4v4h4V3zM21 3h-4v4h4V3zM7 10H3v4h4v-4zM14 10h-4v4h4v-4zM21 10h-4v4h4v-4zM7 17H3v4h4v-4zM14 17h-4v4h4v-4zM21 17h-4v4h4v-4z'],
    'zoom-in': ['M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16z', 'M21 21l-4.35-4.35', 'M11 8v6', 'M8 11h6'],
    'zoom-out': ['M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16z', 'M21 21l-4.35-4.35', 'M8 11h6'],

    // Playback

    // Playback Specific
    'play-fill': ['M5 6v12', 'M19 7.5v9c0 1.5-1.5 2.5-2.5 1.7l-6.5-4.5c-1.2-.8-1.2-2.6 0-3.4l6.5-4.5c1-.8 2.5.2 2.5 1.7z'],
    'pause-fill': ['M6 4h4v16H6z', 'M14 4h4v16H14z'],
    'skip-prev-fill': ['M5 6v12', 'M19 7.5v9c0 1.5-1.5 2.5-2.5 1.7l-6.5-4.5c-1.2-.8-1.2-2.6 0-3.4l6.5-4.5c1-.8 2.5.2 2.5 1.7z'],
    'skip-next-fill': ['M5 7.5v9c0 1.5 1.5 2.5 2.5 1.7l6.5-4.5c1.2-.8 1.2-2.6 0-3.4l-6.5-4.5c-1-.8-2.5.2-2.5 1.7z', 'M19 6v12'],
    'shuffle-2': ['M2 18h1.4c1.3 0 2.5-.6 3.3-1.7l4.1-6.1C11.6 9 12.8 8.4 14.1 8.4H22', 'M18 5l4 3.4-4 3.4', 'M2 8.4h1.4c1.3 0 2.5.6 3.3 1.7l1.2 1.8', 'M11 14.3l1.2 1.8c.8 1.1 2 1.7 3.3 1.7H22', 'M18 14.6l4 3.4-4 3.4'],
    'repeat-1': ['M17 2l4 4-4 4', 'M3 11v-1a4 4 0 0 1 4-4h14', 'M7 22l-4-4 4-4', 'M21 13v1a4 4 0 0 1-4 4H3', 'M11 10h1v4'],
    'repeat': ['M17 2l4 4-4 4', 'M3 11v-1a4 4 0 0 1 4-4h14', 'M7 22l-4-4 4-4', 'M21 13v1a4 4 0 0 1-4 4H3'],
    play: ['M5 3l14 9-14 9V3z'],
    pause: ['M6 4h4v16H6z', 'M14 4h4v16H14z'],
    'volume-up': ['M11 5L6 9H2v6h4l5 4V5z', 'M15.54 8.46a5 5 0 0 1 0 7.07', 'M19.07 4.93a10 10 0 0 1 0 14.14'],
    'volume-down': ['M11 5L6 9H2v6h4l5 4V5z', 'M15.54 8.46a5 5 0 0 1 0 7.07'],
    'volume-mute': ['M11 5L6 9H2v6h4l5 4V5z', 'M23 9l-6 6', 'M17 9l6 6'],
    'seek-forward': ['M13 19l6-7-6-7', 'M5 19l6-7-5-7'],
    'seek-backward': ['M11 19l-6-7 6-7', 'M19 19l-6-7 6-7'],
    'skip-next': ['M5 4l10 8-10 8V4z', 'M19 5v14'],
    'skip-previous': ['M19 20L9 12l10-8v16z', 'M5 19V5'],
    fullscreen: ['M15 3h6v6', 'M9 21H3v-6', 'M21 3l-7 7', 'M3 21l7-7'],




    // UI Elements
    subtitles: ['M2 6h20v12H2z', 'M9 10.5A1.5 1.5 0 0 0 6 12a1.5 1.5 0 0 0 3 1.5', 'M18 10.5A1.5 1.5 0 0 0 15 12a1.5 1.5 0 0 0 3 1.5'],
    pip: ['M2 4h20v16H2z', 'M11 11h9v7h-9z'],
    visualizer: ['M12 2v20', 'M17 5v14', 'M7 8v8', 'M22 10v4', 'M2 11v2'],
    'exit-fullscreen': ['M8 3v3a2 2 0 0 1-2 2H3', 'M21 8h-3a2 2 0 0 1-2-2V3', 'M3 16h3a2 2 0 0 1 2 2v3', 'M16 21v-3a2 2 0 0 1 2-2h3'],

    'chevron-down': ['M6 9l6 6 6-6'],
    'chevron-up': ['M18 15l-6-6-6 6'],
    'chevron-right': ['M9 18l6-6-6-6'],
    'chevron-left': ['M15 18l-6-6 6-6'],
    close: ['M18 6L6 18', 'M6 6l12 12'],
    more: ['M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0', 'M19 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0', 'M5 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0'],
    x: ['M18 6L6 18', 'M6 6l12 12'] // Alias for close
  };

  let paths = $derived(icons[name] || []);
</script>

<svg 
  viewBox="0 0 24 24" 
  width={size} 
  height={size} 
  fill="none" 
  stroke="currentColor" 
  stroke-width={strokeWidth} 
  stroke-linecap="round" 
  stroke-linejoin="round"
  class={className}
>
  {#each paths as d}
    <path {d} />
  {/each}
</svg>
