mod collapse;
mod commands;
mod macos_overlay;
mod ocr;
mod overlay_windows;
mod stt_client;
mod tray;

use aniki_stealth_window::StealthWindowHandle;
use commands::{toggle_click_through_for_app, AppAudioState};
use tauri::{Emitter, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_global_shortcut::{Code, ShortcutState};

pub(crate) struct TauriStealthWindow(pub WebviewWindow);

impl StealthWindowHandle for TauriStealthWindow {
    fn set_content_protected(&self, enabled: bool) -> Result<(), aniki_stealth_window::StealthError> {
        self.0
            .set_content_protected(enabled)
            .map_err(|e| aniki_stealth_window::StealthError::Platform(e.to_string()))
    }

    fn set_skip_taskbar(&self, skip: bool) -> Result<(), aniki_stealth_window::StealthError> {
        self.0
            .set_skip_taskbar(skip)
            .map_err(|e| aniki_stealth_window::StealthError::Platform(e.to_string()))
    }

    #[cfg(target_os = "windows")]
    fn native_window_handle(&self) -> Result<isize, aniki_stealth_window::StealthError> {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let handle = self
            .0
            .window_handle()
            .map_err(|e| aniki_stealth_window::StealthError::Platform(e.to_string()))?;
        match handle.as_raw() {
            RawWindowHandle::Win32(h) => Ok(h.hwnd.get() as isize),
            _ => Err(aniki_stealth_window::StealthError::Platform(
                "expected Win32 handle".into(),
            )),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["CmdOrCtrl+Shift+H", "CmdOrCtrl+Shift+C"])
                .expect("valid shortcut")
                .with_handler(|app, shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return;
                    }

                    match shortcut.key {
                        Code::KeyH => {
                            let state = app.state::<collapse::CollapseState>();
                            let _ = collapse::toggle_collapsed(app, &state);
                        }
                        Code::KeyC => {
                            let _ = toggle_click_through_for_app(app);
                        }
                        _ => {}
                    }
                })
                .build(),
        );

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(AppAudioState::default())
        .manage(collapse::CollapseState::default())
        .invoke_handler(tauri::generate_handler![
            commands::start_audio_session,
            commands::stop_audio_session,
            commands::get_audio_status,
            commands::refresh_stt_jwt,
            commands::hide_overlay,
            commands::panic_hide,
            commands::show_overlay,
            commands::toggle_click_through,
            commands::get_click_through,
            commands::get_stealth_status,
            commands::capture_screen_ocr,
            collapse::collapse_overlay_cmd,
            collapse::expand_overlay_cmd,
            collapse::toggle_collapsed_cmd,
            collapse::is_collapsed_cmd,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            if let Err(e) = tray::install(app) {
                tracing::warn!(error = %e, "Failed to install system tray");
            }

            if let Some(window) = app.get_webview_window("main") {
                let app_handle = app.handle().clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = overlay_windows::hide_all_overlays(&app_handle);
                    }
                });
            }

            if let Err(e) = overlay_windows::ensure_overlay_window(app.handle()) {
                tracing::warn!(error = %e, "Failed to configure overlay panel");
            }

            // If window-state restored pebble dimensions, sync collapse flag.
            if let Some(window) = app.get_webview_window("main") {
                if let (Ok(size), Ok(scale)) = (window.inner_size(), window.scale_factor()) {
                    let w = size.width as f64 / scale;
                    let h = size.height as f64 / scale;
                    if w <= collapse::PEBBLE_SIZE + 2.0 && h <= collapse::PEBBLE_SIZE + 2.0 {
                        let state = app.state::<collapse::CollapseState>();
                        if let Ok(mut collapsed) = state.collapsed.lock() {
                            *collapsed = true;
                        }
                        let _ = app.handle().emit("overlay-collapsed", true);
                    }
                }
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            match event {
                RunEvent::ExitRequested { api, .. } => {
                    if tray::should_prevent_exit() {
                        api.prevent_exit();
                    }
                }
                RunEvent::WindowEvent { label, event, .. } => {
                    if label == "main" {
                        if let WindowEvent::CloseRequested { api, .. } = event {
                            api.prevent_close();
                            let _ = overlay_windows::hide_all_overlays(app_handle);
                        }
                    }
                }
                _ => {}
            }
        });
}
