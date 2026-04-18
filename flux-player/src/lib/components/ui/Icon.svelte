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
  let { name, size = 20, strokeWidth = 2, fill = "none", class: className = "", style = "" } = $props<{
    name: string;
    size?: number | string;
    strokeWidth?: number;
    fill?: string;
    class?: string;
    style?: string;
  }>();

  // Icon Path Dictionary (Using Lucide-inspired geometric paths)
  const icons: Record<string, string[]> = {
    // Navigation (Static Fallbacks)
    discovery: ['M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z', 'M9 12l2 2 4-4'],
    library: ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z'],
    playing: ['M12 2v20 M12 2a10 10 0 0 0-10 10 M12 22a10 10 0 0 0 10-10 M18 8l-8 4 8 4V8z'],
    playlists: ['M21 15V6', 'M18.5 18a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z', 'M12 12H3', 'M16 6H3', 'M12 18H3'], // New music note + lines list
    settings: ['M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z', 'M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z'],
    profile: ['M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z'],
    streaming: ['M4 11V4a2 2 0 0 1 2-2h10l4 4v5', 'M4 17h16', 'M7 22l4-4 4 4'],
    appearance: ['M12 2A10 10 0 0 0 2 12a10 10 0 0 0 10 10 10 10 0 0 0 10-10A10 10 0 0 0 12 2m0 16a3 3 0 1 1 3-3 3 3 0 0 1-3 3', 'M6.5 10c.8 0 1.5.7 1.5 1.5S7.3 13 6.5 13 5 12.3 5 11.5 5.7 10 6.5 10', 'M9.5 6c.8 0 1.5.7 1.5 1.5S10.3 9 9.5 9 8 8.3 8 7.5 8.7 6 9.5 6', 'M14.5 6c.8 0 1.5.7 1.5 1.5S15.3 9 14.5 9 13 8.3 13 7.5 13.7 6 14.5 6', 'M17.5 10c.8 0 1.5.7 1.5 1.5s-.7 1.5-1.5 1.5-1.5-.7-1.5-1.5.7-1.5 1.5-1.5'],    
    playback: ['M5 3l14 9-14 9V3z'],
    storage: ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z', 'M6 12h12', 'M6 16h12'],
    keyboard: ['M2 4h20v16H2V4z', 'M6 8h1M10 8h1M14 8h1M18 8h1M6 12h1M10 12h1M14 12h1M18 12h1M8 16h8'],
    rocket: ['M 15.20 13.73 c 1.02 -0.88 2.07 -1.88 3.17 -2.97 c 1.88 -1.88 3.13 -3.54 3.97 -4.97 c -2.07 -0.45 -3.70 -2.09 -4.15 -4.15 c -1.41 0.84 -3.07 2.11 -4.95 4.01 c -1.10 1.10 -2.09 2.15 -2.97 3.17 c -0.25 0.29 -0.63 0.45 -1.02 0.39 l -2.76 -0.31 c -0.35 -0.04 -0.70 0.08 -0.96 0.33 l -5.34 5.34 c -0.35 0.35 -0.14 0.94 0.35 1.00 l 3.54 0.41 c 0.35 0.04 0.70 -0.08 0.96 -0.33 l 1.27 -1.27 c 0.27 -0.27 0.72 -0.08 0.72 0.29 c -0.02 0.65 0.20 1.21 0.66 1.66 c 0.47 0.47 1.02 0.66 1.66 0.65 c 0.37 0.00 0.57 0.45 0.29 0.72 l -1.27 1.27 c -0.25 0.25 -0.37 0.61 -0.33 0.96 l 0.41 3.54 c 0.06 0.49 0.65 0.70 1.00 0.35 l 5.34 -5.34 c 0.25 -0.25 0.37 -0.61 0.33 -0.96 l -0.31 -2.76 C 14.77 14.38 14.91 13.99 15.20 13.73 z M 13.46 12.46 c -1.04 1.04 -2.29 1.43 -2.82 0.92 c -0.53 -0.53 -0.12 -1.78 0.92 -2.82 c 1.04 -1.04 2.29 -1.43 2.82 -0.92 S 14.48 11.45 13.46 12.46 z','M 20.03 0.71 c 0.12 1.74 1.51 3.13 3.23 3.27 c 1.02 -2.48 0.65 -3.87 0.65 -3.87 S 22.58 -0.37 20.03 0.71 z'],

    community: ['M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2','M9 7a4 4 0 1 0 0-8 4 4 0 0 0 0 8z','M23 21v-2a4 4 0 0 0-3-3.87', 'M16 3.13a4 4 0 0 1 0 7.75'],
    info: ['M12 16v-4','M12 8h.01','M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z'],
    performance: ['M13 2L3 14h9l-1 8 10-12h-9l1-8z'],
    tools: ['M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.77 3.77z'],
    
    // Actions
    refresh: ['M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8', 'M21 3v5h-5'], 
    'refresh-spark': ['M20 12C20 7.58 16.42 4 12 4C9.5 4 7.2 5.1 5.8 6.9L4 9', 'M4 4v5h5','M4 12c0 4.42 3.58 8 8 8c2.5 0 4.8-1.1 6.2-2.9L20 15', 'M20 20v-5h-5'],
    search: ['M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16z', 'M21 21l-4.35-4.35'],
    'new-playlist': ['M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v11z', 'M12 11v6', 'M9 14h6'],
    import: ['M12 3v12M8 11l4 4 4-4M20 17v2a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-2'],
    plus: ['M12 5v14M5 12h14'],
    edit: ['M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7M18.5 2.5a2.121 2.121 0 1 1 3 3L12 15l-4 1 1-4 9.5-9.5z'],
    movie: ['M2 3h20v18H2zM7 3v18M17 3v18M2 8h20M2 16h20'],
    music: ['M9 18V5l12-2v13M9 18c-1.5 0-3 1-3 2.5S7.5 23 9 23s3-1.5 3-3V5M21 16c-1.5 0-3 1-3 2.5s1.5 2.5 3 2.5s3-1.5 3-3V3'],
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
    delete: ['M3 6h18', 'M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2', 'M10 11v6', 'M14 11v6'],

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
    x: ['M18 6L6 18', 'M6 6l12 12'], // Alias for close
    bug: ['M8 2v4', 'M16 2v4', 'M3 12h18', 'M6 7.11V17a6 6 0 0 0 12 0V7.11', 'M4 5.5l2.5 2.5', 'M20 5.5l-2.5 2.5', 'M2 14l3-1.5', 'M22 14l-3-1.5', 'M4 19l2.5-2.5', 'M20 19l-2.5-2.5'],
    github: [
      'M9 19c-5 1.5-5-2.5-7-3', 
      'M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.4 5.4 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4'
    ],
    help: ['M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3', 'M12 17h.01', 'M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z'],
    feedback: ['M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z'],
    copy: ['M20 9h-9a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-9a2 2 0 0 0-2-2z', 'M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1'],
    error: ['M12 8v4M12 16h.01M22 12A10 10 0 1 1 2 12a10 10 0 0 1 20 0z'],
    send: ['M22 2L11 13', 'M22 2l-7 20-4-9-9-4z'],
    camera: ['M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z', 'M12 17a4 4 0 1 0 0-8 4 4 0 0 0 0 8z'],
    sparkle: ['M12 2l1.5 4.5L18 8l-4.5 1.5L12 14l-1.5-4.5L6 8l4.5-1.5L12 2z', 'M5 16l.75 2.25L8 19l-2.25.75L5 22l-.75-2.25L2 19l2.25-.75L5 16z'],
    database: [
      'M3 5v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2z',
      'M3 7h18', 'M3 11h18', 'M3 15h18'
    ],
    trash: ['M3 6h18', 'M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2', 'M10 11v6', 'M14 11v6'],

    'network-offline': []
  };

  let paths = $derived(icons[name] || []);
  let isFilled = $derived(name.startsWith('network-') || name.endsWith('-fill'));
</script>

<svg 
  viewBox="0 0 24 24" 
  width={size} 
  height={size} 
  fill={name.startsWith('network-') ? "none" : (isFilled ? "currentColor" : fill)} 
  stroke={name.startsWith('network-') ? "none" : "currentColor"} 
  stroke-width={strokeWidth} 
  stroke-linecap="round" 
  stroke-linejoin="round"
  class={className}
  {style}
>
  {#if name === 'network-online'}
    <rect x="4" y="14" width="3" height="6" rx="1.5" fill="#00ff00"/>
    <rect x="10.5" y="10" width="3" height="10" rx="1.5" fill="#00ff00"/>
    <rect x="17" y="6" width="3" height="14" rx="1.5" fill="#00ff00"/>
  {:else if name === 'network-weak'}
    <rect x="4" y="14" width="3" height="6" rx="1.5" fill="#ffa500"/>
    <rect x="10.5" y="10" width="3" height="10" rx="1.5" fill="#ffa500"/>
    <rect x="17" y="6" width="3" height="14" rx="1.5" fill="var(--text-muted)"opacity="0.5"/>
  {:else if name === 'network-error'}
    <rect x="4" y="14" width="3" height="6" rx="1.5" fill="#ff0000"/>
    <rect x="10.5" y="10" width="3" height="10" rx="1.5" fill="var(--text-muted)"opacity="0.5"/>
    <rect x="17" y="6" width="3" height="14" rx="1.5" fill="var(--text-muted)"opacity="0.5"/>
    <path d="M20 4L23 7M23 4L20 7" stroke="#ff0000" stroke-width="2" stroke-linecap="round"/>
  {:else if name === 'network-offline'}
    <path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" stroke="var(--text-muted)" fill="none"/>
    <path d="M3 3L21 21" stroke="var(--text-muted)" stroke-opacity="0.8" stroke-width="2.5"/>
  {:else}
    {#each paths as d}
      <path {d} />
    {/each}
  {/if}
</svg>
