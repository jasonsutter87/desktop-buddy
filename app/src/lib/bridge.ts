import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';

export interface Stats {
  hunger: number;
  happiness: number;
  energy: number;
  cleanliness: number;
}

export interface PetData {
  name: string;
  species: string;
  stats: Stats;
  mood: string;
  life_state: string;
  poop_count: number;
  last_fed: number;
  last_played: number;
  created_at: number;
  total_interactions: number;
  starvation_ticks: number;
  generation: number;
  next_species: string | null;
  next_name: string | null;
}

export interface TerminalEvent {
  event_type: string;
  message: string;
}

export async function getStats(): Promise<PetData> {
  return invoke<PetData>('get_stats');
}

export async function feedPet(): Promise<PetData> {
  return invoke<PetData>('feed_pet');
}

export async function petBuddy(): Promise<PetData> {
  return invoke<PetData>('pet_buddy');
}

export async function playWithPet(): Promise<PetData> {
  return invoke<PetData>('play_with_pet');
}

export async function cleanPet(): Promise<PetData> {
  return invoke<PetData>('clean_pet');
}

export async function startEgg(): Promise<PetData> {
  return invoke<PetData>('start_egg');
}

export async function hatchEgg(): Promise<PetData> {
  return invoke<PetData>('hatch_egg');
}

export function onStatsUpdated(cb: (data: PetData) => void) {
  return listen<PetData>('stats-updated', (e) => cb(e.payload));
}

export function onTerminalEvent(cb: (data: TerminalEvent) => void) {
  return listen<TerminalEvent>('terminal-event', (e) => cb(e.payload));
}

export function onTrayAction(cb: (action: string) => void) {
  return listen<string>('tray-action', (e) => cb(e.payload));
}

export function onPetDied(cb: (data: PetData) => void) {
  return listen<PetData>('pet-died', (e) => cb(e.payload));
}

export function startDragging() {
  getCurrentWindow().startDragging();
}
