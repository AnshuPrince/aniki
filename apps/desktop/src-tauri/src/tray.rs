use std::sync::atomic::{AtomicBool, Ordering};

use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::TrayIconBuilder;
use tauri::AppHandle;

use crate::commands::{set_click_through_for_app, toggle_click_through_for_app};
use crate::overlay_windows;

pub static ALLOW_EXIT: AtomicBool = AtomicBool::new(false);

pub fn request_quit(app: &AppHandle) {
    ALLOW_EXIT.store(true, Ordering::SeqCst);
    app.exit(0);
}

pub fn install(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let show = MenuItemBuilder::with_id("show", "Show overlay").build(app)?;
    let click_through = MenuItemBuilder::with_id("click_through", "Toggle click-through").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit Aniki").build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&show)
        .item(&click_through)
        .separator()
        .item(&quit)
        .build()?;

    let mut tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Aniki")
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                let _ = set_click_through_for_app(app, false);
                let _ = overlay_windows::show_all_overlays(app);
            }
            "click_through" => {
                let _ = toggle_click_through_for_app(app);
            }
            "quit" => request_quit(app),
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }

    tray.build(app)?;
    tracing::info!("System tray installed (Show overlay / Quit)");
    Ok(())
}

pub fn should_prevent_exit() -> bool {
    !ALLOW_EXIT.load(Ordering::SeqCst)
}
