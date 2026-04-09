use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const SPECIES_LIST: &[&str] = &["capybara", "duck", "cat", "frog"];
const NAME_LIST: &[&str] = &[
    "Clamber", "Quackers", "Whiskers", "Ribbit",
    "Nugget", "Mochi", "Pudding", "Biscuit",
    "Pebble", "Sprout", "Wobble", "Crumb",
    "Pickles", "Muffin", "Truffle", "Noodle",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hunger: f32,
    pub happiness: f32,
    pub energy: f32,
    pub cleanliness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Happy,
    Content,
    Hungry,
    Tired,
    Sad,
    Dirty,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LifeState {
    Alive,
    Dead,
    Egg,
    Hatching,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetData {
    pub name: String,
    pub species: String,
    pub stats: Stats,
    pub mood: Mood,
    pub life_state: LifeState,
    pub poop_count: u32,
    pub last_fed: u64,
    pub last_played: u64,
    pub created_at: u64,
    pub total_interactions: u64,
    pub starvation_ticks: u32,
    pub generation: u32,
    pub next_species: Option<String>,
    pub next_name: Option<String>,
}

pub struct PetState(pub Mutex<PetData>);

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn derive_mood(stats: &Stats) -> Mood {
    if stats.hunger < 20.0 {
        Mood::Hungry
    } else if stats.energy < 20.0 {
        Mood::Tired
    } else if stats.cleanliness < 20.0 {
        Mood::Dirty
    } else if stats.happiness < 30.0 {
        Mood::Sad
    } else if stats.happiness > 70.0 && stats.hunger > 50.0 {
        Mood::Happy
    } else {
        Mood::Content
    }
}

fn pick_random_species(exclude: &str) -> String {
    let ts = now_secs();
    let candidates: Vec<&&str> = SPECIES_LIST.iter().filter(|s| **s != exclude).collect();
    let idx = (ts as usize) % candidates.len();
    candidates[idx].to_string()
}

fn pick_random_name() -> String {
    let ts = now_secs();
    let idx = ((ts / 7) as usize) % NAME_LIST.len();
    NAME_LIST[idx].to_string()
}

impl PetState {
    pub fn default_clamber() -> Self {
        let ts = now_secs();
        PetState(Mutex::new(PetData {
            name: "Clamber".into(),
            species: "capybara".into(),
            stats: Stats {
                hunger: 80.0,
                happiness: 80.0,
                energy: 80.0,
                cleanliness: 90.0,
            },
            mood: Mood::Happy,
            life_state: LifeState::Alive,
            poop_count: 0,
            last_fed: ts,
            last_played: ts,
            created_at: ts,
            total_interactions: 0,
            starvation_ticks: 0,
            generation: 1,
            next_species: None,
            next_name: None,
        }))
    }
}

/// Decay stats over time. Called every tick (30s).
/// Returns true if the pet just died this tick.
pub fn tick_decay(pet: &mut PetData) -> bool {
    if pet.life_state != LifeState::Alive {
        return false;
    }

    // 3 days from full hunger to 0
    pet.stats.hunger = (pet.stats.hunger - 0.0116).max(0.0);
    pet.stats.happiness = (pet.stats.happiness - 0.0044).max(0.0);
    pet.stats.energy = (pet.stats.energy - 0.0029).max(0.0);

    // Random poop chance when hunger > 30
    if pet.stats.hunger > 30.0 {
        let pseudo_rand = (now_secs() % 7) == 0;
        if pseudo_rand {
            pet.poop_count += 1;
            pet.stats.cleanliness = (pet.stats.cleanliness - 15.0).max(0.0);
        }
    }

    // Starvation death: hunger at 0 for 10 ticks (5 minutes)
    if pet.stats.hunger <= 0.0 {
        pet.starvation_ticks += 1;
        if pet.starvation_ticks >= 10 {
            pet.life_state = LifeState::Dead;
            pet.mood = Mood::Dead;
            // Pre-pick the next species for the egg
            pet.next_species = Some(pick_random_species(&pet.species));
            pet.next_name = Some(pick_random_name());
            return true;
        }
    } else {
        pet.starvation_ticks = 0;
    }

    pet.mood = derive_mood(&pet.stats);
    false
}

/// Start the egg phase after death
pub fn start_egg(pet: &mut PetData) {
    pet.life_state = LifeState::Egg;
}

/// Hatch the egg into a new companion
pub fn hatch(pet: &mut PetData) {
    let ts = now_secs();
    let new_species = pet.next_species.take().unwrap_or_else(|| "capybara".into());
    let new_name = pet.next_name.take().unwrap_or_else(|| "Buddy".into());
    let gen = pet.generation + 1;

    *pet = PetData {
        name: new_name,
        species: new_species,
        stats: Stats {
            hunger: 80.0,
            happiness: 80.0,
            energy: 80.0,
            cleanliness: 100.0,
        },
        mood: Mood::Happy,
        life_state: LifeState::Alive,
        poop_count: 0,
        last_fed: ts,
        last_played: ts,
        created_at: ts,
        total_interactions: 0,
        starvation_ticks: 0,
        generation: gen,
        next_species: None,
        next_name: None,
    };
}

pub fn feed(pet: &mut PetData) {
    if pet.life_state != LifeState::Alive { return; }
    pet.stats.hunger = (pet.stats.hunger + 25.0).min(100.0);
    pet.stats.happiness = (pet.stats.happiness + 5.0).min(100.0);
    pet.last_fed = now_secs();
    pet.total_interactions += 1;
    pet.starvation_ticks = 0;
    pet.mood = derive_mood(&pet.stats);
}

pub fn pet_pet(pet: &mut PetData) {
    if pet.life_state != LifeState::Alive { return; }
    pet.stats.happiness = (pet.stats.happiness + 15.0).min(100.0);
    pet.stats.energy = (pet.stats.energy + 5.0).min(100.0);
    pet.total_interactions += 1;
    pet.mood = derive_mood(&pet.stats);
}

pub fn play(pet: &mut PetData) {
    if pet.life_state != LifeState::Alive { return; }
    pet.stats.happiness = (pet.stats.happiness + 20.0).min(100.0);
    pet.stats.energy = (pet.stats.energy - 10.0).max(0.0);
    pet.stats.hunger = (pet.stats.hunger - 5.0).max(0.0);
    pet.last_played = now_secs();
    pet.total_interactions += 1;
    pet.mood = derive_mood(&pet.stats);
}

pub fn clean(pet: &mut PetData) {
    if pet.life_state != LifeState::Alive { return; }
    pet.stats.cleanliness = 100.0;
    pet.poop_count = 0;
    pet.total_interactions += 1;
    pet.mood = derive_mood(&pet.stats);
}
