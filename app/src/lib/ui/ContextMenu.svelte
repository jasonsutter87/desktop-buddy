<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    x: number;
    y: number;
    onAction: (action: string) => void;
    onClose: () => void;
  }

  let { x, y, onAction, onClose }: Props = $props();

  let menuEl: HTMLDivElement;
  let adjustedX = $state(0);
  let adjustedY = $state(0);
  let ready = $state(false);

  const items = [
    { id: 'feed', label: 'Feed', icon: '🍎' },
    { id: 'pet', label: 'Pet', icon: '✋' },
    { id: 'play', label: 'Play', icon: '⚾' },
    { id: 'clean', label: 'Clean', icon: '🧹' },
    { id: 'stats', label: 'Stats', icon: '📊' },
  ];

  function handleClick(id: string) {
    onAction(id);
  }

  function handleOutsideClick(e: MouseEvent) {
    onClose();
  }

  onMount(() => {
    // Position menu so it doesn't overflow the window
    const rect = menuEl.getBoundingClientRect();
    const vw = window.innerWidth;
    const vh = window.innerHeight;
    adjustedX = x + rect.width > vw ? vw - rect.width - 4 : x;
    adjustedY = y + rect.height > vh ? y - rect.height : y;
    if (adjustedY < 0) adjustedY = 4;
    ready = true;

    const timer = setTimeout(() => {
      window.addEventListener('click', handleOutsideClick, { once: true });
    }, 50);
    return () => {
      clearTimeout(timer);
      window.removeEventListener('click', handleOutsideClick);
    };
  });
</script>

<div
  class="menu"
  class:ready
  bind:this={menuEl}
  style="left: {adjustedX}px; top: {adjustedY}px;"
>
  {#each items as item}
    <button class="menu-item" onclick={() => handleClick(item.id)}>
      <span class="icon">{item.icon}</span>
      {item.label}
    </button>
  {/each}
</div>

<style>
  .menu {
    position: fixed;
    opacity: 0;
    background: #1a1a2e;
    border: 2px solid #333;
    border-radius: 6px;
    padding: 4px 0;
    min-width: 120px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    z-index: 100;
  }
  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 12px;
    border: none;
    background: none;
    color: #e0e0e0;
    font-size: 13px;
    font-family: 'Segoe UI', system-ui, sans-serif;
    cursor: pointer;
    text-align: left;
  }
  .menu-item:hover {
    background: #2a2a4e;
  }
  .icon {
    font-size: 14px;
  }
  .ready {
    opacity: 1;
  }
</style>
