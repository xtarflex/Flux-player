<script lang="ts">
  /**
   * Toggle.svelte
   * Reusable switch component for Flux settings.
   */
  let { checked = $bindable(false), label = "", onchange = (val: boolean) => {} } = $props<{
    checked?: boolean;
    label?: string;
    onchange?: (val: boolean) => void;
  }>();

  function toggle() {
    checked = !checked;
    onchange(checked);
  }
</script>

<label class="switch">
  <input 
    type="checkbox" 
    bind:checked={checked} 
    onchange={() => onchange(checked)}
  />
  <span class="slider round"></span>
</label>

<style>
  .switch {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 24px;
    flex-shrink: 0;
  }

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--glass-border-high);
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 24px;
    border: 1px solid var(--glass-border-mid);
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 2px;
    bottom: 2px;
    background-color: #fff;
    transition: .3s cubic-bezier(0.4, 0, 0.2, 1);
    border-radius: 50%;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
  }

  input:checked + .slider {
    background-color: var(--secondary);
    border-color: var(--secondary);
  }

  input:checked + .slider:before {
    transform: translateX(24px);
  }
</style>
