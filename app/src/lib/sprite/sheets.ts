export interface SheetDef {
  src: string;
  frames: number;
  fps: number;
  loop: boolean;
}

// Per-species sprite sheets. Each species needs at minimum an 'idle' sheet.
// Capybara has the full set; other species fall back to idle for missing anims.
const SPECIES_SHEETS: Record<string, Record<string, SheetDef>> = {
  capybara: {
    idle:         { src: '/sprites/clamber-idle.png',        frames: 4, fps: 3,  loop: true },
    walk:         { src: '/sprites/clamber-walk.png',        frames: 6, fps: 8,  loop: true },
    eat:          { src: '/sprites/clamber-eat.png',         frames: 6, fps: 6,  loop: false },
    sleep:        { src: '/sprites/clamber-sleep.png',       frames: 4, fps: 2,  loop: true },
    poop:         { src: '/sprites/clamber-poop.png',        frames: 4, fps: 4,  loop: false },
    pet:          { src: '/sprites/clamber-pet.png',         frames: 4, fps: 6,  loop: false },
    play:         { src: '/sprites/clamber-play.png',        frames: 6, fps: 8,  loop: false },
    react:        { src: '/sprites/clamber-react.png',       frames: 4, fps: 6,  loop: false },
    'mood-happy': { src: '/sprites/clamber-mood-happy.png',  frames: 4, fps: 3,  loop: true },
    'mood-hungry':{ src: '/sprites/clamber-mood-hungry.png', frames: 4, fps: 3,  loop: true },
    'mood-sad':   { src: '/sprites/clamber-mood-sad.png',    frames: 4, fps: 3,  loop: true },
    'mood-dirty': { src: '/sprites/clamber-mood-dirty.png',  frames: 4, fps: 3,  loop: true },
    dead:         { src: '/sprites/clamber-dead.png',        frames: 4, fps: 2,  loop: true },
  },
  duck: {
    idle:         { src: '/sprites/duck-idle.png',           frames: 4, fps: 3,  loop: true },
  },
  cat: {
    idle:         { src: '/sprites/cat-idle.png',            frames: 4, fps: 3,  loop: true },
  },
  frog: {
    idle:         { src: '/sprites/frog-idle.png',           frames: 4, fps: 3,  loop: true },
  },
};

// Special sheets (not species-specific)
export const EGG_SHEET: SheetDef = {
  src: '/sprites/egg.png', frames: 7, fps: 3, loop: true,
};

export function getSheet(species: string, animation: string): SheetDef {
  const speciesSheets = SPECIES_SHEETS[species] || SPECIES_SHEETS.capybara;
  return speciesSheets[animation] || speciesSheets.idle;
}

export const SPRITE_SIZE = 48;
