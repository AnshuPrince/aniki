//! Convert the overlay to an NSPanel so it can float above fullscreen app Spaces.
//! A plain NSWindow cannot join other apps' fullscreen desktops on macOS.

#[cfg(target_os = "macos")]
pub fn configure_overlay_panel(window: &tauri::WebviewWindow) -> Result<(), String> {
    use tauri_nspanel::{
        cocoa::appkit::NSWindowCollectionBehavior,
        WebviewWindowExt as _,
    };

    // NSWindowStyleMaskNonActivatingPanel — receive events without stealing focus.
    #[allow(non_upper_case_globals)]
    const NS_NONACTIVATING_PANEL: i32 = 1 << 7;

    let panel = window.to_panel().map_err(|e| e.to_string())?;

    panel.set_level(25); // NSMainMenuWindowLevel (24) + 1
    panel.set_style_mask(NS_NONACTIVATING_PANEL);
    panel.set_floating_panel(true);
    panel.set_hides_on_deactivate(false);
    panel.set_works_when_modal(true);
    panel.set_becomes_key_only_if_needed(true);
    panel.set_collection_behaviour(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
    );

    tracing::info!("macOS overlay converted to NSPanel (all Spaces + fullscreen apps)");
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn show_overlay_panel(app: &tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    use tauri_nspanel::ManagerExt as _;

    let panel = app
        .get_webview_panel("main")
        .map_err(|_| "overlay panel not found".to_string())?;

    if let Some(window) = app.get_webview_window("main") {
        window.show().ok();
        window.set_always_on_top(true).ok();
    }

    panel.show();
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn hide_overlay_panel(app: &tauri::AppHandle) -> Result<(), String> {
    use tauri::Manager;
    use tauri_nspanel::ManagerExt as _;

    if let Ok(panel) = app.get_webview_panel("main") {
        panel.order_out(None);
    }

    if let Some(window) = app.get_webview_window("main") {
        window.hide().ok();
    }

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn overlay_panel_visible(app: &tauri::AppHandle) -> bool {
    use tauri_nspanel::ManagerExt as _;

    app.get_webview_panel("main")
        .map(|p| p.is_visible())
        .unwrap_or(false)
}

#[cfg(not(target_os = "macos"))]
pub fn configure_overlay_panel(_window: &tauri::WebviewWindow) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn show_overlay_panel(app: &tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().ok();
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn hide_overlay_panel(app: &tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn overlay_panel_visible(app: &tauri::AppHandle) -> bool {
    app.get_webview_window("main")
        .and_then(|w| w.is_visible().ok())
        .unwrap_or(false)
}
