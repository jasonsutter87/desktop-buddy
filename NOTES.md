# Buddy — Desktop Pet App

## Where It Lives

```
C:\Users\jason\buddy\
├── app/                    ← The Tauri app (this is the project root)
│   ├── src/                ← Svelte frontend
│   ├── src-tauri/          ← Rust backend
│   ├── public/sprites/     ← Sprite PNGs used by the app
│   └── package.json
├── sprites/                ← Generated sprite master copies
└── sprite-gen/             ← Node scripts that generate sprites
```

## How to Run

```bash
cd C:\Users\jason\buddy\app
pnpm tauri dev
```

This starts both the Vite dev server (frontend) and compiles/runs the Rust backend.
Hot-reload works for frontend changes. Rust changes need a restart.

## How to Build a Release

```bash
cd C:\Users\jason\buddy\app
pnpm tauri build
```

Output goes to `src-tauri/target/release/buddy.exe`
Installer goes to `src-tauri/target/release/bundle/`

## How to Kill It

- Right-click tray icon → Quit
- Or: `taskkill /F /IM buddy.exe`

## How to Toggle Visibility

- Left-click the system tray icon (bottom-right, may be behind the `^` arrow)

---

## Key Files to Edit

### Tuning Pet Stats
**`app/src-tauri/src/pet.rs`**
- Line ~117: `tick_decay()` — hunger/happiness/energy decay rates
- `starvation_ticks >= 10` — ticks at 0 hunger before death (each tick = 30s)
- `feed()`, `pet_pet()`, `play()`, `clean()` — how much each action restores

### Tick Speed
**`app/src-tauri/src/lib.rs`**
- `Duration::from_secs(30)` — how often stats decay (30 = every 30 seconds)

### Species & Names
**`app/src-tauri/src/pet.rs`** (top of file)
- `SPECIES_LIST` — available species for egg hatching
- `NAME_LIST` — random names for new pets

### Tauri Commands (backend API)
**`app/src-tauri/src/commands.rs`**
- All the invoke handlers: `get_stats`, `feed_pet`, `pet_buddy`, `play_with_pet`, `clean_pet`, `start_egg`, `hatch_egg`

### Terminal Watcher Patterns
**`app/src-tauri/src/terminal_watcher.rs`**
- `classify_line()` — what terminal commands trigger reactions
- Watches PowerShell history at `%APPDATA%\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt`

### System Tray Menu
**`app/src-tauri/src/tray.rs`**
- Menu items and click handlers

### Window Config
**`app/src-tauri/tauri.conf.json`**
- Size: `width: 280, height: 400`
- Position: `x: 1200, y: 600`
- `alwaysOnTop`, `transparent`, `decorations`, etc.

---

## Frontend (Svelte)

### Main App Logic
**`app/src/App.svelte`**
- Animation state machine
- Mood → idle animation mapping
- Death → egg → hatch sequence
- Right-click menu, speech bubbles, starvation warning

### Tauri Bridge (all IPC calls)
**`app/src/lib/bridge.ts`**
- Every `invoke()` and `listen()` call in one file

### Sprite Engine
**`app/src/lib/sprite/engine.ts`** — canvas animation loop
**`app/src/lib/sprite/sheets.ts`** — sprite sheet definitions per species
**`app/src/lib/sprite/SpriteCanvas.svelte`** — the rendered canvas component

### UI Components
**`app/src/lib/ui/ContextMenu.svelte`** — right-click menu
**`app/src/lib/ui/SpeechBubble.svelte`** — floating text
**`app/src/lib/ui/StatsPanel.svelte`** — stat bars overlay

---

## Sprites

### Regenerating Sprites
```bash
cd C:\Users\jason\buddy\sprite-gen
node generate.mjs            # Base animations (idle, walk, eat, sleep, poop, pet, play, react)
node generate-moods.mjs      # Mood variants (happy, hungry, sad, dirty, dead)
node generate-egg-and-species.mjs  # Egg + duck/cat/frog
```

Then copy to app:
```bash
cp ../sprites/*.png ../app/public/sprites/
```

### Sprite Format
- 48x48 pixels per frame
- Horizontal strip PNGs (all frames side by side)
- Scaled to 192x192 in the app with `image-rendering: pixelated`
- Each character in the template maps to a color in the palette (`C` object)

### Adding a New Species
1. Add sprite frames in `sprite-gen/generate-egg-and-species.mjs`
2. Run the generator and copy PNGs
3. Add sheets to `app/src/lib/sprite/sheets.ts` in `SPECIES_SHEETS`
4. Add species name to `SPECIES_LIST` in `app/src-tauri/src/pet.rs`

---

## Current Stats (for tuning reference)

| Stat | Decay/tick | Full→0 time | Notes |
|------|-----------|-------------|-------|
| Hunger | -0.0116 | **3 days** | Death after 5 more min at 0 |
| Happiness | -0.0044 | ~8 days | |
| Energy | -0.0029 | ~12 days | Sleep animation at <15 |
| Cleanliness | -15 per poop | Random | Poops when hunger >30 |

| Action | Effect |
|--------|--------|
| Feed | +25 hunger, +5 happy |
| Pet | +15 happy, +5 energy |
| Play | +20 happy, -10 energy, -5 hunger |
| Clean | 100 cleanliness, 0 poops |

## Tech Stack
- **Tauri v2** (Rust backend + webview frontend)
- **Svelte 5** (frontend framework)
- **Vite** (bundler)
- **Sharp** (sprite generation, dev only)
- **notify** crate (terminal file watching)
- **tauri-plugin-autostart** (startup on boot)
- **tauri-plugin-store** (state persistence)
