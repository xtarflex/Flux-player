/**
 * @file draggable.ts
 * @description Svelte action for handling dragging interactions.
 */

interface DraggableOptions {
  onDragStart?: (e: MouseEvent) => void;
  onDragMove?: (pos: { x: number; y: number }) => void;
  onDragEnd?: () => void;
  initialPos: { x: number; y: number };
}

/**
 * Custom action to make an element draggable.
 * 
 * @param node - The DOM element.
 * @param options - Drag callbacks and initial position.
 */
export function draggable(node: HTMLElement, options: DraggableOptions) {
  let isDragging = false;
  let dragOffset = { x: 0, y: 0 };
  let currentPos = { ...options.initialPos };

  function handleMouseDown(e: MouseEvent) {
    if (e.button !== 0) return; // Left click only
    isDragging = true;
    dragOffset = {
      x: e.clientX - currentPos.x,
      y: e.clientY - currentPos.y
    };
    
    options.onDragStart?.(e);
    
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    currentPos = {
      x: e.clientX - dragOffset.x,
      y: e.clientY - dragOffset.y
    };
    options.onDragMove?.(currentPos);
  }

  function handleMouseUp() {
    if (!isDragging) return;
    isDragging = false;
    options.onDragEnd?.();
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }

  node.addEventListener('mousedown', handleMouseDown);

  return {
    destroy() {
      node.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    },
    update(newOptions: DraggableOptions) {
      // In case we want to update initialPos or callbacks dynamically
      options = newOptions;
    }
  };
}
