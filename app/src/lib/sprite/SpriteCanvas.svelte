<script lang="ts">
  import { onMount } from 'svelte';
  import { getSheet, EGG_SHEET, SPRITE_SIZE, type SheetDef } from './sheets';
  import { loadImage, createAnimState, renderFrame, type AnimationState } from './engine';
  import { startDragging } from '../bridge';

  interface Props {
    species?: string;
    animation: string;
    size?: number;
    onAnimationEnd?: () => void;
    onRightClick?: (e: MouseEvent) => void;
    onClick?: () => void;
  }

  let { species = 'capybara', animation = 'idle', size = 192, onAnimationEnd, onRightClick, onClick }: Props = $props();

  let canvas: HTMLCanvasElement;
  let animState: AnimationState | null = $state(null);
  let rafId: number = 0;
  let currentKey = '';

  function getSheetForAnim(sp: string, anim: string): SheetDef {
    if (anim === 'egg') return EGG_SHEET;
    return getSheet(sp, anim);
  }

  async function switchAnimation(sp: string, anim: string) {
    const key = `${sp}:${anim}`;
    if (key === currentKey) return;
    const sheet = getSheetForAnim(sp, anim);
    const img = await loadImage(sheet.src);
    animState = createAnimState(sheet, img);
    currentKey = key;
  }

  function tick(timestamp: number) {
    if (!canvas || !animState) {
      rafId = requestAnimationFrame(tick);
      return;
    }
    const ctx = canvas.getContext('2d')!;
    const newState = renderFrame(ctx, animState, size, SPRITE_SIZE, timestamp);

    if (newState.done && !animState.done) {
      onAnimationEnd?.();
    }

    animState = newState;
    rafId = requestAnimationFrame(tick);
  }

  function handleMouseDown(e: MouseEvent) {
    if (e.button === 0) {
      if (onClick) {
        onClick();
      } else {
        startDragging();
      }
    }
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    onRightClick?.(e);
  }

  $effect(() => {
    switchAnimation(species, animation);
  });

  onMount(() => {
    switchAnimation(species, animation);
    rafId = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafId);
  });
</script>

<canvas
  bind:this={canvas}
  width={size}
  height={size}
  style="image-rendering: pixelated; cursor: grab; width: {size}px; height: {size}px;"
  onmousedown={handleMouseDown}
  oncontextmenu={handleContextMenu}
></canvas>
