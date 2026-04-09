export interface SheetDef {
  src: string;
  frames: number;
  fps: number;
  loop: boolean;
}

function s(species: string, anim: string, frames: number, fps: number, loop: boolean): SheetDef {
  return { src: `/sprites/${species}-${anim}.png`, frames, fps, loop };
}

// Full animation set shared by all species
function fullSet(sp: string): Record<string, SheetDef> {
  return {
    idle:          s(sp, 'idle',        4, 3,  true),
    walk:          s(sp, 'walk',        6, 8,  true),
    eat:           s(sp, 'eat',         6, 6,  false),
    sleep:         s(sp, 'sleep',       4, 2,  true),
    poop:          s(sp, 'poop',        4, 4,  false),
    pet:           s(sp, 'pet',         4, 6,  false),
    play:          s(sp, 'play',        6, 8,  false),
    react:         s(sp, 'react',       4, 6,  false),
    'mood-happy':  s(sp, 'mood-happy',  4, 3,  true),
    'mood-hungry': s(sp, 'mood-hungry', 4, 3,  true),
    'mood-sad':    s(sp, 'mood-sad',    4, 3,  true),
    'mood-dirty':  s(sp, 'mood-dirty',  4, 3,  true),
    dead:          s(sp, 'dead',        4, 2,  true),
  };
}

const SPECIES_SHEETS: Record<string, Record<string, SheetDef>> = {
  capybara: {
    ...fullSet('clamber'),
    // capybara uses 'clamber-' prefix for legacy sprite names
  },
  duck: fullSet('duck'),
  cat:  fullSet('cat'),
  frog: fullSet('frog'),
};

export const EGG_SHEET: SheetDef = {
  src: '/sprites/egg.png', frames: 7, fps: 3, loop: true,
};

export function getSheet(species: string, animation: string): SheetDef {
  const speciesSheets = SPECIES_SHEETS[species] || SPECIES_SHEETS.capybara;
  return speciesSheets[animation] || speciesSheets.idle;
}

export const SPRITE_SIZE = 48;
