<script lang="ts">
  import { onMount } from 'svelte';
  import SpriteCanvas from './lib/sprite/SpriteCanvas.svelte';
  import SpeechBubble from './lib/ui/SpeechBubble.svelte';
  import ContextMenu from './lib/ui/ContextMenu.svelte';
  import StatsPanel from './lib/ui/StatsPanel.svelte';
  import {
    getStats, feedPet, petBuddy, playWithPet, cleanPet,
    startEgg, hatchEgg,
    onStatsUpdated, onTerminalEvent, onTrayAction, onPetDied,
    type PetData, type TerminalEvent,
  } from './lib/bridge';

  let animation = $state('idle');
  let species = $state('capybara');
  let pet = $state<PetData | null>(null);
  let bubbleText = $state('');
  let showBubble = $state(false);
  let showMenu = $state(false);
  let showStats = $state(false);
  let menuX = $state(0);
  let menuY = $state(0);
  let pendingIdle = false;
  let offsetX = $state(0);
  let wanderTimer: ReturnType<typeof setTimeout> | null = null;

  function showSpeech(text: string, duration = 3000) {
    bubbleText = text;
    showBubble = true;
    setTimeout(() => { showBubble = false; }, duration);
  }

  function playAnim(name: string) {
    animation = name;
    pendingIdle = true;
  }

  function onAnimEnd() {
    if (!pet) return;

    // Egg just loops — hatch happens on click
    if (animation === 'egg') return;

    if (pendingIdle) {
      pendingIdle = false;
      animation = getMoodIdle();
    }
  }

  // Pick the right idle animation based on mood
  function getMoodIdle(): string {
    if (!pet) return 'idle';
    if (pet.life_state === 'dead') return 'dead';
    if (pet.life_state === 'egg') return 'egg';

    switch (pet.mood) {
      case 'happy': return 'mood-happy';
      case 'hungry': return 'mood-hungry';
      case 'sad': return 'mood-sad';
      case 'dirty': return 'mood-dirty';
      case 'tired': return 'sleep';
      default: return 'idle'; // content
    }
  }

  async function handleDeath() {
    if (!pet) return;
    animation = 'dead';
    species = pet.species;
    showSpeech(`${pet.name} has passed away...`, 5000);

    // After 5 seconds, transition to egg
    setTimeout(async () => {
      try {
        pet = await startEgg();
        animation = 'egg';
        showSpeech('An egg! Click to hatch it...', 6000);
      } catch (e) {
        console.error('Failed to start egg:', e);
      }
    }, 5000);
  }

  async function doHatch() {
    try {
      pet = await hatchEgg();
      species = pet.species;
      animation = getMoodIdle();
      showSpeech(`${pet.name} the ${pet.species} has hatched!`, 4000);
    } catch (e) {
      console.error('Failed to hatch:', e);
    }
  }

  async function handleAction(action: string) {
    showMenu = false;
    showStats = false;

    if (!pet) return;

    // Can't interact with dead/egg pets (except stats)
    if (pet.life_state !== 'alive' && action !== 'stats') {
      if (pet.life_state === 'dead') {
        showSpeech('...', 2000);
      }
      return;
    }

    try {
      switch (action) {
        case 'feed':
          pet = await feedPet();
          playAnim('eat');
          showSpeech('*munch munch*');
          break;
        case 'pet':
          pet = await petBuddy();
          playAnim('pet');
          showSpeech('*happy noises*');
          break;
        case 'play':
          pet = await playWithPet();
          playAnim('play');
          showSpeech('Wheee!');
          break;
        case 'clean':
          pet = await cleanPet();
          showSpeech('Squeaky clean!');
          break;
        case 'stats':
          showStats = !showStats;
          break;
      }
    } catch (e) {
      console.error('Action failed:', e);
    }
  }

  function handleRightClick(e: MouseEvent) {
    menuX = e.clientX;
    menuY = e.clientY;
    showMenu = true;
    showStats = false;
  }

  function handleTerminalEvent(event: TerminalEvent) {
    if (!pet || pet.life_state !== 'alive') return;

    // Pick animation based on event type
    switch (event.event_type) {
      case 'celebration':
        playAnim('play');
        break;
      case 'frustrated':
        playAnim('pet'); // sympathetic look
        break;
      case 'mentioned':
        playAnim('react');
        break;
      case 'greeting':
        playAnim('play');
        break;
      case 'sleepy':
        // Don't override sleep anim, just show bubble
        break;
      default:
        playAnim('react');
    }

    showSpeech(event.message, 3500);
  }

  // Reactively update animation when mood changes
  $effect(() => {
    if (!pet) return;
    if (pet.life_state === 'dead' && animation !== 'dead' && animation !== 'egg') {
      // Death is handled by the event, don't override
      return;
    }
    if (pet.life_state === 'alive' && !pendingIdle) {
      // Only switch idle variant if we're already in an idle-like state
      const idleAnims = ['idle', 'mood-happy', 'mood-hungry', 'mood-sad', 'mood-dirty', 'sleep'];
      if (idleAnims.includes(animation)) {
        animation = getMoodIdle();
      }
    }
  });

  onMount(async () => {
    try {
      pet = await getStats();
      species = pet.species;
      animation = getMoodIdle();
    } catch (e) {
      console.error('Failed to get stats:', e);
    }

    const unsubStats = await onStatsUpdated((data) => {
      pet = data;
      species = data.species;
    });

    const unsubTerminal = await onTerminalEvent(handleTerminalEvent);
    const unsubTray = await onTrayAction(handleAction);

    const unsubDied = await onPetDied(() => {
      handleDeath();
    });

    // Start idle wandering
    scheduleWander();

    return () => {
      unsubStats();
      unsubTerminal();
      unsubTray();
      unsubDied();
      if (wanderTimer) clearTimeout(wanderTimer);
    };
  });

  function scheduleWander() {
    // Random delay between 8-20 seconds
    const delay = 8000 + Math.random() * 12000;
    wanderTimer = setTimeout(() => {
      if (!pet || pet.life_state !== 'alive' || pendingIdle) {
        scheduleWander();
        return;
      }
      // Do a little walk
      const idleAnims = ['idle', 'mood-happy', 'mood-hungry', 'mood-sad', 'mood-dirty'];
      if (!idleAnims.includes(animation)) {
        scheduleWander();
        return;
      }
      // Pick a random direction, walk a little
      const direction = Math.random() > 0.5 ? 1 : -1;
      const distance = 10 + Math.floor(Math.random() * 20);
      animation = 'walk';
      pendingIdle = true;

      // Animate the offset over 1 second
      const steps = 10;
      const stepSize = (direction * distance) / steps;
      let step = 0;
      const walkInterval = setInterval(() => {
        offsetX = Math.max(-40, Math.min(40, offsetX + stepSize));
        step++;
        if (step >= steps) {
          clearInterval(walkInterval);
        }
      }, 100);

      // Return to idle after walk animation
      setTimeout(() => {
        animation = getMoodIdle();
        pendingIdle = false;
        scheduleWander();
      }, 1200);
    }, delay);
  }
</script>

<div class="pet-container" style="transform: translateX(calc(-50% + {offsetX}px));">
  {#if showBubble}
    <SpeechBubble text={bubbleText} onDone={() => showBubble = false} />
  {/if}

  <SpriteCanvas
    {species}
    {animation}
    size={192}
    onAnimationEnd={onAnimEnd}
    onRightClick={handleRightClick}
    onClick={pet?.life_state === 'egg' ? doHatch : undefined}
  />

  {#if pet && pet.life_state === 'alive' && pet.poop_count > 0}
    <div class="poop-indicator">
      {#each Array(Math.min(pet.poop_count, 3)) as _}
        <span class="poop">💩</span>
      {/each}
    </div>
  {/if}

  {#if pet && pet.life_state === 'alive' && pet.starvation_ticks > 0}
    <div class="warning">
      ⚠ Starving! Feed me!
    </div>
  {/if}
</div>

{#if showMenu}
  <ContextMenu x={menuX} y={menuY} onAction={handleAction} onClose={() => showMenu = false} />
{/if}

{#if showStats && pet}
  <StatsPanel {pet} onClose={() => showStats = false} />
{/if}

<style>
  .pet-container {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .poop-indicator {
    position: absolute;
    bottom: -10px;
    right: -20px;
    display: flex;
    gap: 2px;
    font-size: 16px;
  }

  .poop {
    animation: poop-bounce 0.6s ease-in-out infinite alternate;
  }
  .poop:nth-child(2) { animation-delay: 0.2s; }
  .poop:nth-child(3) { animation-delay: 0.4s; }

  @keyframes poop-bounce {
    from { transform: translateY(0); }
    to { transform: translateY(-3px); }
  }

  .warning {
    position: absolute;
    bottom: -20px;
    color: #e74c3c;
    font-size: 11px;
    font-family: 'Segoe UI', system-ui, sans-serif;
    font-weight: bold;
    white-space: nowrap;
    animation: pulse 1s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.4; }
  }
</style>
