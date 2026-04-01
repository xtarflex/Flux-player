import { tooltipState, type Placement } from '../stores/tooltip';

interface TooltipParams {
  content: string;
  shortcut?: string;
  secondaryContent?: string;
  secondaryShortcut?: string;
  placement?: Placement;
}

export function tooltip(node: HTMLElement, params: string | TooltipParams) {
  let content = '';
  let shortcut = '';
  let secondaryContent = '';
  let secondaryShortcut = '';
  let placement: Placement = 'bottom';

  function parseParams(p: string | TooltipParams) {
    if (typeof p === 'string') {
      content = p;
      shortcut = '';
      secondaryContent = '';
      secondaryShortcut = '';
      placement = 'bottom';
    } else {
      content = p.content;
      shortcut = p.shortcut || '';
      secondaryContent = p.secondaryContent || '';
      secondaryShortcut = p.secondaryShortcut || '';
      placement = p.placement || 'bottom';
    }
  }

  parseParams(params);

  let isCurrentlyHovered = false;

  function handleMouseEnter() {
    isCurrentlyHovered = true;
    const rect = node.getBoundingClientRect();
    
    // Calculate base position (centered below/above the element by default depending on placement)
    let x = rect.left + rect.width / 2;
    let y = rect.bottom;
    
    if (placement === 'top') {
      y = rect.top;
    } else if (placement === 'left') {
      x = rect.left;
      y = rect.top + rect.height / 2;
    } else if (placement === 'right') {
      x = rect.right;
      y = rect.top + rect.height / 2;
    }

    // Use setTimeout to avoid 'state_unsafe_mutation' in Svelte 5 if this 
    // fires during a reaction or layout shift.
    setTimeout(() => {
      tooltipState.set({
        visible: true,
        content,
        shortcut,
        secondaryContent,
        secondaryShortcut,
        placement,
        x,
        y
      });
    }, 0);
  }

  function handleMouseLeave() {
    isCurrentlyHovered = false;
    // Use setTimeout to avoid 'state_unsafe_mutation' in Svelte 5 if this 
    // fires during a DOM teardown or reactive transition.
    setTimeout(() => {
      tooltipState.update(s => ({ ...s, visible: false }));
    }, 0);
  }

  node.addEventListener('mouseenter', handleMouseEnter);
  node.addEventListener('mouseleave', handleMouseLeave);
  node.addEventListener('focus', handleMouseEnter);
  node.addEventListener('blur', handleMouseLeave);

  return {
    update(newParams: string | TooltipParams) {
      parseParams(newParams);
      // Only refresh the tooltip if THIS node is the one being hovered
      if (isCurrentlyHovered) {
        handleMouseEnter(); 
      }
    },
    destroy() {
      node.removeEventListener('mouseenter', handleMouseEnter);
      node.removeEventListener('mouseleave', handleMouseLeave);
      node.removeEventListener('focus', handleMouseEnter);
      node.removeEventListener('blur', handleMouseLeave);
      
      // Only hide the global tooltip if it was being shown for THIS node
      if (isCurrentlyHovered) {
        setTimeout(() => {
          tooltipState.update(s => ({ ...s, visible: false }));
        }, 0);
      }
    }
  };
}
