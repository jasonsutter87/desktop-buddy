<script lang="ts">
  import type { PetData } from '../bridge';

  interface Props {
    pet: PetData;
    onClose: () => void;
  }

  let { pet, onClose }: Props = $props();

  const statDefs = [
    { key: 'hunger', label: 'Hunger', color: '#e67e22' },
    { key: 'happiness', label: 'Happy', color: '#f1c40f' },
    { key: 'energy', label: 'Energy', color: '#2ecc71' },
    { key: 'cleanliness', label: 'Clean', color: '#3498db' },
  ] as const;
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="panel" role="dialog" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.key === 'Escape' && onClose()}>
  <div class="header">
    <span class="name">{pet.name}</span>
    <span class="species">{pet.species}</span>
    <button class="close" onclick={onClose}>x</button>
  </div>
  <div class="mood">Mood: {pet.mood}</div>
  <div class="stats">
    {#each statDefs as stat}
      <div class="stat-row">
        <span class="stat-label">{stat.label}</span>
        <div class="bar-bg">
          <div
            class="bar-fill"
            style="width: {pet.stats[stat.key]}%; background: {stat.color};"
          ></div>
        </div>
        <span class="stat-val">{Math.round(pet.stats[stat.key])}</span>
      </div>
    {/each}
  </div>
  {#if pet.poop_count > 0}
    <div class="poop-alert">Poops: {pet.poop_count} (right-click → Clean!)</div>
  {/if}
  <div class="footer">
    Gen {pet.generation} · {pet.total_interactions} interactions
    {#if pet.life_state === 'dead'} · DEAD{/if}
  </div>
</div>

<style>
  .panel {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    background: #1a1a2e;
    border: 2px solid #444;
    border-radius: 8px;
    padding: 10px;
    color: #e0e0e0;
    font-size: 12px;
    font-family: 'Segoe UI', system-ui, sans-serif;
    z-index: 50;
  }
  .header {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 6px;
  }
  .name {
    font-weight: bold;
    font-size: 14px;
  }
  .species {
    color: #888;
    font-size: 11px;
  }
  .close {
    margin-left: auto;
    background: none;
    border: 1px solid #555;
    color: #aaa;
    border-radius: 3px;
    cursor: pointer;
    font-size: 11px;
    padding: 0 4px;
  }
  .close:hover {
    background: #333;
  }
  .mood {
    color: #aaa;
    margin-bottom: 8px;
    text-transform: capitalize;
  }
  .stat-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 4px;
  }
  .stat-label {
    width: 50px;
    font-size: 11px;
    color: #aaa;
  }
  .bar-bg {
    flex: 1;
    height: 8px;
    background: #333;
    border-radius: 4px;
    overflow: hidden;
  }
  .bar-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.5s ease;
  }
  .stat-val {
    width: 24px;
    text-align: right;
    font-size: 11px;
    color: #aaa;
  }
  .poop-alert {
    margin-top: 6px;
    color: #e67e22;
    font-size: 11px;
  }
  .footer {
    margin-top: 6px;
    color: #666;
    font-size: 10px;
  }
</style>
