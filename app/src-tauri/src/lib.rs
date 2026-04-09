mod commands;
mod pet;
mod terminal_watcher;
mod tray;

use pet::PetState;
use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--hidden"]),
        ))
        .manage(PetState::default_clamber())
        .invoke_handler(tauri::generate_handler![
            commands::get_stats,
            commands::feed_pet,
            commands::pet_buddy,
            commands::play_with_pet,
            commands::clean_pet,
            commands::start_egg,
            commands::hatch_egg,
        ])
        .setup(|app| {
            // Create system tray
            tray::create_tray(app)?;

            // Start terminal watcher
            terminal_watcher::start_watcher(app.handle().clone());

            // Start stat decay loop
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_time()
                    .build()
                    .unwrap();
                rt.block_on(async {
                    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
                    loop {
                        interval.tick().await;
                        let (snapshot, just_died) = {
                            let state = handle.state::<PetState>();
                            let mut pet = match state.0.lock() {
                                Ok(p) => p,
                                Err(_) => continue,
                            };
                            let died = pet::tick_decay(&mut pet);
                            (pet.clone(), died)
                        };
                        let _ = handle.emit("stats-updated", &snapshot);
                        if just_died {
                            let _ = handle.emit("pet-died", &snapshot);
                        }
                    }
                });
            });

            // Show window after setup (avoids white flash)
            if let Some(window) = app.get_webview_window("buddy") {
                let _ = window.show();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running buddy");
}
