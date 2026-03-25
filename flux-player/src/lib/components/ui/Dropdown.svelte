<script lang="ts">
  /**
   * @typedef {Object} Props
   * @property {string[]} options - The list of options to display
   * @property {string} value - The currently selected value (bindable)
   * @property {string} [label] - Optional label for the dropdown
   * @property {boolean} [showCheckmark] - Whether to show the checkmark on selection
   */
  let { 
    options, 
    value = $bindable(), 
    label = "", 
    showCheckmark = true,
    isOpen = $bindable(false)
  } = $props<{
    options: string[];
    value: string;
    label?: string;
    showCheckmark?: boolean;
    isOpen?: boolean;
  }>();

  let highlightedIndex = $state(-1);

  // Sync highlight with current value when opened
  $effect(() => {
    if (isOpen) {
      highlightedIndex = options.indexOf(value);
    }
  });

  let container = $state<HTMLElement>();

  function selectOption(option: string) {
    value = option;
    isOpen = false;
  }

  // Handle clicking outside to close
  function handleGlobalClick(event: MouseEvent) {
    if (isOpen && container) {
      const target = event.target as HTMLElement;
      if (!container.contains(target)) {
        isOpen = false;
      }
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (!isOpen) return;

    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        highlightedIndex = (highlightedIndex + 1) % options.length;
        break;
      case 'ArrowUp':
        e.preventDefault();
        highlightedIndex = (highlightedIndex - 1 + options.length) % options.length;
        break;
      case 'Enter':
        e.preventDefault();
        if (highlightedIndex >= 0) selectOption(options[highlightedIndex]);
        break;
      case 'Escape':
        e.preventDefault();
        isOpen = false;
        break;
    }
  }

  import { onMount, onDestroy } from 'svelte';
  onMount(() => {
    window.addEventListener('click', handleGlobalClick);
    window.addEventListener('keydown', handleKeydown);
  });
  onDestroy(() => {
    window.removeEventListener('click', handleGlobalClick);
    window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="dropdown-container" bind:this={container}>
  {#if label}
    <span class="dropdown-label">{label}</span>
  {/if}
  
  <button 
    class="dropdown-trigger" 
    class:active={isOpen}
    onclick={() => isOpen = !isOpen}
    type="button"
  >
    <span class="selected-text">{value}</span>
    <svg 
      class="chevron" 
      class:rotated={isOpen}
      viewBox="0 0 24 24" 
      fill="none" 
      stroke="currentColor" 
      stroke-width="2"
    >
      <path d="M6 9l6 6 6-6" stroke-linecap="round" stroke-linejoin="round" />
    </svg>
  </button>

  {#if isOpen}
    <div class="dropdown-menu glass">
      {#each options as option, i}
        <button 
          class="dropdown-item" 
          class:selected={option === value}
          class:highlighted={i === highlightedIndex}
          onclick={() => selectOption(option)}
          onmouseenter={() => highlightedIndex = i}
        >
          {option}
          {#if showCheckmark && option === value}
            <span class="check">✓</span>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown-container {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 180px;
  }

  .dropdown-label {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  .dropdown-trigger {
    background: var(--glass-bg-low);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
    padding: 0.75rem 1rem;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    font-family: var(--font-body);
    font-size: 0.95rem;
    width: 100%;
    text-align: left;
  }

  .dropdown-trigger:hover {
    background: var(--glass-bg-mid);
    border-color: var(--secondary-muted);
  }

  .dropdown-trigger.active {
    border-color: var(--secondary);
    box-shadow: 0 0 15px rgba(0, 255, 255, 0.1);
  }

  .chevron {
    width: 18px;
    height: 18px;
    color: var(--secondary);
    transition: transform 0.3s ease;
    opacity: 0.8;
  }

  .chevron.rotated {
    transform: rotate(180deg);
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    width: 100%;
    z-index: 1000;
    border-radius: 12px;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4);
    animation: slideDown 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .dropdown-item {
    background: transparent;
    border: none;
    color: var(--text-muted);
    padding: 0.75rem 1rem;
    border-radius: 8px;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 0.95rem;
    font-family: var(--font-body);
  }

  .dropdown-item:hover, .dropdown-item.highlighted {
    background: var(--glass-bg-low);
    color: var(--text-main);
  }

  .dropdown-item.selected {
    color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
    font-weight: 600;
  }

  .check {
    font-weight: bold;
    font-size: 0.85rem;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
