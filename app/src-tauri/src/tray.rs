use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let toggle = MenuItemBuilder::with_id("toggle", "Show/Hide").build(app)?;
    let feed = MenuItemBuilder::with_id("feed", "Feed Clamber").build(app)?;
    let pet = MenuItemBuilder::with_id("pet", "Pet Clamber").build(app)?;
    let play = MenuItemBuilder::with_id("play", "Play").build(app)?;
    let stats = MenuItemBuilder::with_id("stats", "View Stats").build(app)?;
    let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&toggle, &separator, &feed, &pet, &play, &stats, &separator, &quit])
        .build()?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .tooltip("Buddy - Clamber the Capybara")
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "toggle" => {
                toggle_window(app);
            }
            "feed" => {
                let _ = app.emit("tray-action", "feed");
            }
            "pet" => {
                let _ = app.emit("tray-action", "pet");
            }
            "play" => {
                let _ = app.emit("tray-action", "play");
            }
            "stats" => {
                let _ = app.emit("tray-action", "stats");
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            // Left-click the tray icon to toggle
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                toggle_window(tray.app_handle());
            }
        })
        .build(app)?;

    Ok(())
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("buddy") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}
