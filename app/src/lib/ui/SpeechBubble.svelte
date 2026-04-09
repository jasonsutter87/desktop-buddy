<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    text: string;
    duration?: number;
    onDone?: () => void;
  }

  let { text, duration = 3000, onDone }: Props = $props();
  let visible = $state(true);
  let fading = $state(false);

  onMount(() => {
    const fadeTimer = setTimeout(() => { fading = true; }, duration - 500);
    const hideTimer = setTimeout(() => {
      visible = false;
      onDone?.();
    }, duration);
    return () => {
      clearTimeout(fadeTimer);
      clearTimeout(hideTimer);
    };
  });
</script>

{#if visible}
  <div class="bubble" class:fading>
    {text}
  </div>
{/if}

<style>
  .bubble {
    position: absolute;
    top: -8px;
    left: 50%;
    transform: translateX(-50%);
    background: #1a1a2e;
    color: #e0e0e0;
    border: 2px solid #444;
    border-radius: 8px;
    padding: 4px 10px;
    font-size: 13px;
    font-family: 'Segoe UI', system-ui, sans-serif;
    white-space: nowrap;
    pointer-events: none;
    opacity: 1;
    transition: opacity 0.5s ease-out;
    z-index: 10;
  }
  .bubble::after {
    content: '';
    position: absolute;
    bottom: -6px;
    left: 50%;
    transform: translateX(-50%);
    width: 0;
    height: 0;
    border-left: 6px solid transparent;
    border-right: 6px solid transparent;
    border-top: 6px solid #444;
  }
  .fading {
    opacity: 0;
  }
</style>
