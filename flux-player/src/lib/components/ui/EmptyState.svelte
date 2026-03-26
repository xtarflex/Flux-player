<script lang="ts">
  /**
   * EmptyState.svelte
   *
   * A unified component for displaying empty states (e.g., 0 items in a playlist,
   * empty library, or "Coming Soon" features).
   *
   * It features a clean, geometric, abstract SVG illustration by default that
   * adheres to the Cyber Dark theme (crisp borders, primary/secondary colors).
   *
   * Note: The `<slot name="illustration">` allows you to easily swap this
   * geometric SVG for a more complex unDraw-style illustration later.
   */
  let { title, description, actionLabel = "", onAction = () => {} } = $props<{
    title: string;
    description: string;
    actionLabel?: string;
    onAction?: () => void;
  }>();
</script>

<div class="empty-state">
  <div class="illustration-container">
    <!-- Default Geometric Illustration -->
    <slot name="illustration">
      <svg class="geometric-illustration" viewBox="0 0 120 120" fill="none" xmlns="http://www.w3.org/2000/svg">
        <!-- Floating Elements Background -->
        <circle cx="20" cy="20" r="4" fill="var(--secondary)" opacity="0.2" />
        <circle cx="100" cy="100" r="6" fill="var(--primary)" opacity="0.2" />

        <!-- Base Slate / Folder -->
        <path class="slate-base" d="M30 45 L90 45 L90 95 L30 95 Z" stroke="var(--primary)" stroke-width="1.5" fill="rgba(138, 43, 226, 0.05)" />
        <path class="slate-tab" d="M30 45 L30 35 L50 35 L60 45 Z" stroke="var(--primary)" stroke-width="1.5" fill="rgba(138, 43, 226, 0.05)" />

        <!-- Media Nodes / Data Points -->
        <circle cx="45" cy="70" r="10" stroke="var(--secondary)" stroke-width="1.5" class="data-node" />
        <path d="M42 66 L52 70 L42 74 Z" fill="var(--secondary)" class="data-play" />

        <path class="data-line" d="M65 65 L80 65" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4" />
        <path class="data-line delay-1" d="M65 75 L75 75" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" opacity="0.4" />
      </svg>
    </slot>
  </div>

  <h2 class="empty-title">{title}</h2>
  <p class="empty-description">{description}</p>

  {#if actionLabel}
    <button class="empty-action-btn" onclick={onAction}>
      {actionLabel}
    </button>
  {/if}
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 64px 24px;
    height: 100%;
    min-height: 400px;
    animation: fadeIn 0.5s ease;
  }

  .illustration-container {
    width: 240px;
    height: 240px;
    margin-bottom: 32px;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .geometric-illustration {
    width: 100%;
    height: 100%;
  }

  /* Abstract Drawing Animations */
  .slate-base, .slate-tab {
    stroke-dasharray: 240;
    stroke-dashoffset: 240;
    animation: draw 2s ease forwards;
  }

  .slate-tab {
    animation-delay: 0.5s;
  }

  .data-node {
    transform-origin: 45px 70px;
    animation: pulse 4s ease-in-out infinite alternate;
  }

  .data-play {
    transform-origin: 47px 70px;
    animation: pulse 4s ease-in-out infinite alternate 1s;
  }

  .data-line {
    stroke-dasharray: 20;
    stroke-dashoffset: 20;
    animation: draw 1.5s ease forwards 1s;
  }

  .data-line.delay-1 {
    animation-delay: 1.2s;
  }

  .empty-title {
    font-family: var(--font-heading);
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-main);
    letter-spacing: 0.05em;
    margin-bottom: 12px;
  }

  .empty-description {
    font-family: var(--font-body);
    font-size: 1rem;
    color: var(--text-muted);
    max-width: 400px;
    line-height: 1.6;
    margin-bottom: 32px;
  }

  .empty-action-btn {
    background: var(--glass-bg-low);
    border: 1px solid var(--primary);
    color: var(--primary);
    padding: 12px 32px;
    border-radius: 8px;
    font-family: var(--font-heading);
    font-size: 0.9rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
  }

  .empty-action-btn:hover {
    background: var(--primary);
    color: white;
    box-shadow: 0 4px 20px rgba(138, 43, 226, 0.4);
    transform: translateY(-2px);
  }

  @keyframes draw {
    to { stroke-dashoffset: 0; }
  }

  @keyframes pulse {
    0% { transform: scale(1); opacity: 0.8; }
    100% { transform: scale(1.05); opacity: 1; }
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
