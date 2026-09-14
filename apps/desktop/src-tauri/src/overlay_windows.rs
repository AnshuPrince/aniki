use tauri::{AppHandle, Manager, WebviewWindow};

use crate::macos_overlay;

pub fn configure_overlay_window(window: &WebviewWindow) -> Result<(), String> {
    window.set_always_on_top(true).map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    {
        window
            .set_visible_on_all_workspaces(true)
            .map_err(|e| e.to_string())?;
        macos_overlay::configure_overlay_panel(window)?;
    }
    crate::commands::apply_stealth(window)?;
    Ok(())
}

pub fn ensure_overlay_window(app: &AppHandle) -> Result<(), String> {
    let main = app
        .get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())?;
    configure_overlay_window(&main)?;
    tracing::info!("Overlay panel configured");
    Ok(())
}

pub fn hide_all_overlays(app: &AppHandle) -> Result<(), String> {
    macos_overlay::hide_overlay_panel(app)
}

pub fn show_all_overlays(app: &AppHandle) -> Result<(), String> {
    let collapse = app.state::<crate::collapse::CollapseState>();
    if crate::collapse::is_collapsed(&collapse)? {
        crate::collapse::expand_overlay(app, &collapse)?;
    }
    let _ = crate::commands::set_click_through_for_app(app, false);
    macos_overlay::show_overlay_panel(app)
}

pub fn any_overlay_visible(app: &AppHandle) -> bool {
    macos_overlay::overlay_panel_visible(app)
}
