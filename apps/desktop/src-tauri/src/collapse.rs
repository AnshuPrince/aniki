use std::sync::Mutex;

use tauri::{AppHandle, Emitter, LogicalSize, Manager};

pub const PEBBLE_SIZE: f64 = 38.0;
pub const MIN_EXPANDED_WIDTH: f64 = 280.0;
pub const MIN_EXPANDED_HEIGHT: f64 = 280.0;
pub const DEFAULT_WIDTH: f64 = 380.0;
pub const DEFAULT_HEIGHT: f64 = 640.0;

pub struct CollapseState {
    pub collapsed: Mutex<bool>,
    pub saved_width: Mutex<f64>,
    pub saved_height: Mutex<f64>,
}

impl Default for CollapseState {
    fn default() -> Self {
        Self {
            collapsed: Mutex::new(false),
            saved_width: Mutex::new(DEFAULT_WIDTH),
            saved_height: Mutex::new(DEFAULT_HEIGHT),
        }
    }
}

fn main_window(app: &AppHandle) -> Result<tauri::WebviewWindow, String> {
    app.get_webview_window("main").ok_or_else(|| "main window not found".to_string())
}

pub fn collapse_overlay(app: &AppHandle, state: &CollapseState) -> Result<(), String> {
    let window = main_window(app)?;
    let mut collapsed = state.collapsed.lock().map_err(|e| e.to_string())?;
    if *collapsed {
        return Ok(());
    }

    let size = window.inner_size().map_err(|e| e.to_string())?;
    let scale = window.scale_factor().map_err(|e| e.to_string())?;
    let width = size.width as f64 / scale;
    let height = size.height as f64 / scale;

    if width > PEBBLE_SIZE + 10.0 && height > PEBBLE_SIZE + 10.0 {
        *state.saved_width.lock().map_err(|e| e.to_string())? =
            width.max(MIN_EXPANDED_WIDTH);
        *state.saved_height.lock().map_err(|e| e.to_string())? =
            height.max(MIN_EXPANDED_HEIGHT);
    }

    window
        .set_size(LogicalSize::new(PEBBLE_SIZE, PEBBLE_SIZE))
        .map_err(|e| e.to_string())?;

    *collapsed = true;
    let _ = app.emit("overlay-collapsed", true);
    tracing::info!("Overlay collapsed to pebble");
    Ok(())
}

pub fn expand_overlay(app: &AppHandle, state: &CollapseState) -> Result<(), String> {
    use crate::commands::set_click_through_for_app;

    let window = main_window(app)?;
    let mut collapsed = state.collapsed.lock().map_err(|e| e.to_string())?;
    if !*collapsed {
        return Ok(());
    }

    set_click_through_for_app(app, false)?;

    let width = *state.saved_width.lock().map_err(|e| e.to_string())?;
    let height = *state.saved_height.lock().map_err(|e| e.to_string())?;

    window
        .set_size(LogicalSize::new(width, height))
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        use tauri_nspanel::ManagerExt as _;
        if let Ok(panel) = app.get_webview_panel("main") {
            panel.show();
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        window.show().map_err(|e| e.to_string())?;
    }

    *collapsed = false;
    let _ = app.emit("overlay-collapsed", false);
    tracing::info!(width, height, "Overlay expanded from pebble");
    Ok(())
}

pub fn toggle_collapsed(app: &AppHandle, state: &CollapseState) -> Result<bool, String> {
    let is_collapsed = *state.collapsed.lock().map_err(|e| e.to_string())?;
    if is_collapsed {
        expand_overlay(app, state)?;
        Ok(false)
    } else {
        collapse_overlay(app, state)?;
        Ok(true)
    }
}

pub fn is_collapsed(state: &CollapseState) -> Result<bool, String> {
    Ok(*state.collapsed.lock().map_err(|e| e.to_string())?)
}

#[tauri::command]
pub fn collapse_overlay_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, CollapseState>,
) -> Result<(), String> {
    collapse_overlay(&app, &state)
}

#[tauri::command]
pub fn expand_overlay_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, CollapseState>,
) -> Result<(), String> {
    expand_overlay(&app, &state)
}

#[tauri::command]
pub fn toggle_collapsed_cmd(
    app: tauri::AppHandle,
    state: tauri::State<'_, CollapseState>,
) -> Result<bool, String> {
    toggle_collapsed(&app, &state)
}

#[tauri::command]
pub fn is_collapsed_cmd(state: tauri::State<'_, CollapseState>) -> Result<bool, String> {
    is_collapsed(&state)
}
