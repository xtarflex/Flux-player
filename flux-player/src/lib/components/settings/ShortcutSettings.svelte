<script lang="ts">
  import Icon from "../ui/Icon.svelte";

  // Mock shortcut data based on HOTKEYS.md
  let sections = $state([
    {
      title: "Navigation",
      items: [
        { action: "Go to Library", key: "CTRL + L", editable: true },
        { action: "Go to Discovery", key: "CTRL + D", editable: true },
        { action: "Go to Playlists", key: "CTRL + P", editable: true },
        { action: "Open Settings", key: "CTRL + ,", editable: true },
        { action: "Search Focus", key: "CTRL + F", editable: false },
      ]
    },
    {
      title: "Playback",
      items: [
        { action: "Play / Pause", key: "SPACE", editable: true },
        { action: "Rewind 10s", key: "J", editable: true },
        { action: "Forward 10s", key: "L", editable: true },
        { action: "Next Track", key: "SHIFT + →", editable: false },
        { action: "Previous Track", key: "SHIFT + ←", editable: false },
        { action: "Mute", key: "M", editable: false },
      ]
    },
    {
      title: "Library Mode",
      items: [
        { action: "Switch View Mode", key: "V", editable: true },
        { action: "Filter Menu", key: "F", editable: true },
        { action: "Sort Menu", key: "S", editable: true },
        { action: "Select All", key: "CTRL + A", editable: false },
        { action: "Invert Selection", key: "CTRL + I", editable: false },
      ]
    }
  ]);

  let editingId = $state<string | null>(null);
  let currentKeyCombo = $state<string>("");

  function startEdit(sectionTitle: string, action: string, currentKey: string) {
    editingId = `${sectionTitle}-${action}`;
    currentKeyCombo = currentKey;
  }

  function stopEdit(sectionIndex: number, itemIndex: number) {
    if (editingId && currentKeyCombo) {
      // Save the intercepted keys
      sections[sectionIndex].items[itemIndex].key = currentKeyCombo;
    }
    editingId = null;
    currentKeyCombo = "";
  }

  function handleKeydown(e: KeyboardEvent, sectionIndex: number, itemIndex: number) {
    if (!editingId) return;

    e.preventDefault();

    // Prevent saving just modifier keys
    if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) {
      return;
    }

    if (e.key === 'Escape') {
      editingId = null;
      return;
    }

    if (e.key === 'Enter') {
      stopEdit(sectionIndex, itemIndex);
      return;
    }

    let keys = [];
    if (e.ctrlKey || e.metaKey) keys.push('CTRL');
    if (e.shiftKey) keys.push('SHIFT');
    if (e.altKey) keys.push('ALT');

    let mainKey = e.key;
    if (mainKey === ' ') mainKey = 'SPACE';
    else if (mainKey.length === 1) mainKey = mainKey.toUpperCase();
    else if (mainKey.startsWith('Arrow')) mainKey = mainKey.replace('Arrow', ''); // e.g., ArrowRight -> Right
    // map arrows to symbols
    if (mainKey === 'Up') mainKey = '↑';
    if (mainKey === 'Down') mainKey = '↓';
    if (mainKey === 'Left') mainKey = '←';
    if (mainKey === 'Right') mainKey = '→';

    keys.push(mainKey);
    currentKeyCombo = keys.join(' + ');

    // Automatically stop editing and save once a full combo is pressed
    stopEdit(sectionIndex, itemIndex);
  }
</script>

<div class="shortcut-settings">
  <header class="section-header">
    <div class="header-content">
      <Icon name="settings" size={24} class="header-icon" />
      <div>
        <h2>Hotkeys & Control</h2>
        <p>Manage app-wide keyboard shortcuts and interaction patterns.</p>
      </div>
    </div>
    <button class="reset-btn">Reset to Defaults</button>
  </header>

  <div class="shortcuts-container">
    {#each sections as section, sIdx}
      <section class="shortcut-group">
        <h3>{section.title}</h3>
        <div class="shortcut-list">
          {#each section.items as item, iIdx}
            <div class="shortcut-row" class:is-editing={editingId === `${section.title}-${item.action}`}>
              <span class="action-name">{item.action}</span>
              <div class="key-container">
                {#if editingId === `${section.title}-${item.action}`}
                  <input 
                    type="text" 
                    class="key-input" 
                    value={currentKeyCombo}
                    onkeydown={(e) => handleKeydown(e, sIdx, iIdx)}
                    onblur={() => { editingId = null; }}
                    autoFocus
                    readonly
                    placeholder="Press keys..."
                  />
                {:else}
                  <button 
                    class="key-chip" 
                    class:editable={item.editable}
                    onclick={() => item.editable && startEdit(section.title, item.action, item.key)}
                  >
                    {item.key}
                  </button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </section>
    {/each}
  </div>
</div>

<style>
  .shortcut-settings {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    animation: fadeIn 0.4s ease-out;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding-bottom: 2rem;
    border-bottom: 1px solid var(--glass-border-low);
  }

  .header-content {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  :global(.header-icon) {
    color: var(--secondary);
  }

  h2 {
    font-family: var(--font-heading);
    font-size: 1.5rem;
    margin: 0;
    letter-spacing: 0.05em;
  }

  p {
    color: var(--text-muted);
    margin: 0.5rem 0 0;
    font-size: 0.9rem;
  }

  .reset-btn {
    background: transparent;
    border: 1px solid var(--glass-border-mid);
    color: var(--text-muted);
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .reset-btn:hover {
    border-color: var(--secondary);
    color: var(--secondary);
  }

  .shortcuts-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 2.5rem;
  }

  .shortcut-group h3 {
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.15em;
    color: var(--secondary);
    margin-bottom: 1.5rem;
    opacity: 0.8;
  }

  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .shortcut-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.8rem 1rem;
    background: var(--glass-bg-low);
    border-radius: 12px;
    border: 1px solid transparent;
    transition: all 0.2s ease;
  }

  .shortcut-row:hover {
    background: var(--glass-bg-mid);
    border-color: var(--glass-border-low);
  }

  .shortcut-row.is-editing {
    border-color: var(--secondary);
    background: rgba(0, 255, 255, 0.05);
  }

  .action-name {
    font-size: 0.9rem;
    color: var(--text-main);
  }

  .key-chip {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--glass-border-mid);
    color: var(--secondary);
    font-family: monospace;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    font-size: 0.85rem;
    min-width: 80px;
    text-align: center;
  }

  .key-chip.editable {
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .key-chip.editable:hover {
    border-color: var(--secondary);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.2);
  }

  .key-input {
    background: var(--bg-base);
    border: 1px solid var(--secondary);
    color: var(--text-main);
    font-family: monospace;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    font-size: 0.85rem;
    width: 140px;
    outline: none;
    text-align: center;
  }

  .key-input:focus {
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.2);
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
