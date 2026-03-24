<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { MenuItem } from './context-menu';

  let { x, y, items, onclose } = $props<{
    x: number;
    y: number;
    items: MenuItem[];
    onclose: () => void;
  }>();

  let menuElement = $state<HTMLElement>();
  let activeSubmenu = $state<string | null>(null);

  // Measure and position logic
  let offsetX = $state(0);
  let offsetY = $state(0);
  let flipSubmenus = $state(false);
  let isPositioned = $state(false);

  $effect(() => {
    if (menuElement) {
      const rect = menuElement.getBoundingClientRect();
      const padding = 16;
      const expectedSubmenuWidth = 200; 
      
      // Horizontal overflow check for main menu
      if (x + rect.width > window.innerWidth - padding) {
        offsetX = -rect.width;
        flipSubmenus = true;
      } else {
        offsetX = 0;
        if (x + rect.width + expectedSubmenuWidth > window.innerWidth - padding) {
          flipSubmenus = true;
        } else {
          flipSubmenus = false;
        }
      }
      
      // Vertical overflow check
      if (y + rect.height > window.innerHeight - padding) {
        offsetY = -rect.height;
      } else {
        offsetY = 0;
      }

      // Signal that we are ready to paint
      isPositioned = true;
    }
  });

  function handleOutsideClick(e: MouseEvent) {
    if (menuElement && !menuElement.contains(e.target as Node)) {
      onclose();
    }
  }

  onMount(() => {
    // delay adding listener to avoid instant close from trigger click
    setTimeout(() => {
      window.addEventListener('mousedown', handleOutsideClick);
      window.addEventListener('contextmenu', handleContextOutside);
    }, 10);
  });

  function handleContextOutside(e: MouseEvent) {
    if (menuElement && !menuElement.contains(e.target as Node)) {
      onclose();
    }
  }

  onDestroy(() => {
    window.removeEventListener('mousedown', handleOutsideClick);
    window.removeEventListener('contextmenu', handleContextOutside);
  });
</script>

<div 
  class="context-menu glass" 
  style="left: {x}px; top: {y}px; transform: translate({offsetX}px, {offsetY}px); visibility: {isPositioned ? 'visible' : 'hidden'}"
  bind:this={menuElement}
  oncontextmenu={(e) => e.preventDefault()}
  role="menu"
  tabindex="-1"
>
  {#each items as item}
    {#if item.separator}
      <div class="separator" role="separator"></div>
    {:else if item.children}
      <!-- Use div for items with submenus to avoid nested buttons -->
      <div 
        class="menu-item has-submenu"
        onmouseenter={() => activeSubmenu = item.label || null}
        role="menuitem"
        tabindex="0"
      >
        <span class="label">{item.label}</span>
        <svg class="chevron" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 18l6-6-6-6" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        
        {#if activeSubmenu === item.label}
          <div class="submenu glass" class:flip-left={flipSubmenus} role="menu">
            {#each item.children as child}
              <button 
                class="menu-item"
                class:danger={child.danger}
                onclick={() => {
                  child.action?.();
                  onclose();
                }}
                role="menuitem"
              >
                <span class="label">{child.label}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {:else}
      <button 
        class="menu-item"
        class:danger={item.danger}
        onmouseenter={() => activeSubmenu = null}
        onclick={() => {
          item.action?.();
          onclose();
        }}
        role="menuitem"
      >
        <span class="label">{item.label}</span>
      </button>
    {/if}
  {/each}
</div>

<style>
  .context-menu {
    position: fixed;
    z-index: 99999;
    min-width: 200px;
    padding: 6px;
    border-radius: 12px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    gap: 2px;
    animation: fadeIn 0.1s cubic-bezier(0.2, 0, 0, 1);
    backdrop-filter: blur(20px);
    background: rgba(10, 10, 12, 0.75);
    border: 1px solid var(--glass-border-mid);
    color: var(--text-main);
  }

  .menu-item {
    background: transparent;
    border: none;
    color: var(--text-muted);
    padding: 10px 12px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: all 0.2s ease;
    position: relative;
    width: 100%;
    text-align: left;
    font-family: var(--font-body);
  }

  .menu-item:hover {
    background: var(--glass-bg-low);
    color: var(--text-main);
  }

  .menu-item.danger {
    color: #ff4444;
  }

  .menu-item.danger:hover {
    background: rgba(255, 68, 68, 0.1);
  }

  .separator {
    height: 1px;
    background: var(--glass-border-low);
    margin: 4px 8px;
  }

  .chevron {
    width: 14px;
    height: 14px;
    opacity: 0.5;
  }

  .submenu {
    position: absolute;
    top: -6px;
    left: calc(100% + 4px);
    min-width: 180px;
    padding: 6px;
    border-radius: 12px;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    gap: 2px;
    background: rgba(10, 10, 12, 0.9);
    border: 1px solid var(--glass-border-mid);
    backdrop-filter: blur(20px);
  }

  .submenu.flip-left {
    left: auto;
    right: calc(100% + 4px);
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
</style>
