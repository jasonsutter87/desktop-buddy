use crate::pet::{self, PetData, PetState};

#[tauri::command]
pub fn get_stats(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let pet = state.0.lock().map_err(|e| e.to_string())?;
    Ok(pet.clone())
}

#[tauri::command]
pub fn feed_pet(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::feed(&mut pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn pet_buddy(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::pet_pet(&mut pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn play_with_pet(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::play(&mut pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn clean_pet(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::clean(&mut pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn start_egg(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::start_egg(&mut pet);
    Ok(pet.clone())
}

#[tauri::command]
pub fn hatch_egg(state: tauri::State<'_, PetState>) -> Result<PetData, String> {
    let mut pet = state.0.lock().map_err(|e| e.to_string())?;
    pet::hatch(&mut pet);
    Ok(pet.clone())
}
