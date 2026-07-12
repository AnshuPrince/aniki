use std::sync::Mutex;

use aniki_audio_core::{DualCapture, PipelineConfig};
use aniki_stealth_window::{StealthConfig, StealthWindow};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State, WebviewWindow};

use crate::overlay_windows;

use crate::stt_client::SttSessionHandle;

pub struct AppAudioState {
    pub capture: Mutex<Option<DualCapture>>,
    pub stt: Mutex<Option<SttSessionHandle>>,
    pub stealth: Mutex<StealthWindow>,
    pub click_through: Mutex<bool>,
    pub dev_stt_mode: Mutex<bool>,
}

impl Default for AppAudioState {
    fn default() -> Self {
        Self {
            capture: Mutex::new(None),
            stt: Mutex::new(None),
            stealth: Mutex::new(StealthWindow::new(StealthConfig::default())),
            click_through: Mutex::new(false),
            dev_stt_mode: Mutex::new(false),
        }
    }
}

#[derive(Serialize)]
pub struct StealthStatusResponse {
    pub os_version: String,
    pub capture_exclusion_supported: bool,
    pub capture_exclusion_degraded: bool,
    pub degradation_reason: Option<String>,
}

#[derive(Serialize)]
pub struct AudioStatusResponse {
    pub mic_active: bool,
    pub mic_level: f32,
    pub dev_stt_mode: bool,
    pub live_stt: bool,
}

#[tauri::command]
pub async fn start_audio_session(
    app: tauri::AppHandle,
    state: State<'_, AppAudioState>,
    stt_jwt: String,
    stt_endpoint: String,
    transcription_config: Option<serde_json::Value>,
) -> Result<AudioStatusResponse, String> {
    tracing::info!(endpoint = %stt_endpoint, "Starting audio session");

    let dev_mode = stt_jwt.starts_with("dev-stt-jwt");
    *state.dev_stt_mode.lock().map_err(|e| e.to_string())? = dev_mode;

    let mut capture = DualCapture::new(PipelineConfig::default());
    capture.start().map_err(|e| {
        tracing::error!(error = %e, "Failed to start microphone capture");
        format!("Microphone error: {e}. Check System Settings → Privacy → Microphone and allow Aniki.")
    })?;

    let config = transcription_config.unwrap_or_else(|| {
        serde_json::json!({
            "language": "en",
            "enable_partials": true,
            "max_delay": 0.7,
            "diarization": "channel",
            "channel_diarization_labels": ["interviewer", "candidate"]
        })
    });

    let handle = crate::stt_client::start_stt_session(
        app.clone(),
        stt_endpoint,
        stt_jwt,
        config,
        capture,
    )
    .await?;

    *state.stt.lock().map_err(|e| e.to_string())? = Some(handle);

    Ok(AudioStatusResponse {
        mic_active: true,
        mic_level: 0.0,
        dev_stt_mode: dev_mode,
        live_stt: !dev_mode,
    })
}

#[tauri::command]
pub async fn get_audio_status(state: State<'_, AppAudioState>) -> Result<AudioStatusResponse, String> {
    let dev_mode = *state.dev_stt_mode.lock().map_err(|e| e.to_string())?;
    let mut capture_guard = state.capture.lock().map_err(|e| e.to_string())?;
    let (mic_active, mic_level) = if let Some(capture) = capture_guard.as_mut() {
        (capture.is_running(), capture.mic_level())
    } else {
        (false, 0.0)
    };

    Ok(AudioStatusResponse {
        mic_active,
        mic_level,
        dev_stt_mode: dev_mode,
        live_stt: !dev_mode,
    })
}

#[tauri::command]
pub async fn stop_audio_session(state: State<'_, AppAudioState>) -> Result<(), String> {
    if let Some(handle) = state.stt.lock().map_err(|e| e.to_string())?.take() {
        handle.stop();
    }
    if let Some(mut capture) = state.capture.lock().map_err(|e| e.to_string())?.take() {
        capture.stop();
    }
    *state.dev_stt_mode.lock().map_err(|e| e.to_string())? = false;
    Ok(())
}

#[tauri::command]
pub async fn refresh_stt_jwt(
    state: State<'_, AppAudioState>,
    jwt: String,
) -> Result<(), String> {
    let handle = state
        .stt
        .lock()
        .map_err(|e| e.to_string())?
        .clone();
    if let Some(handle) = handle {
        handle.update_jwt(jwt).await;
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_overlay(app: tauri::AppHandle, state: State<'_, AppAudioState>) -> Result<(), String> {
    let mut stealth = state.stealth.lock().map_err(|e| e.to_string())?;
    stealth.panic_hide().map_err(|e| e.to_string())?;
    overlay_windows::hide_all_overlays(&app)
}

#[tauri::command]
pub async fn panic_hide(app: tauri::AppHandle, state: State<'_, AppAudioState>) -> Result<(), String> {
    hide_overlay(app, state).await
}

#[tauri::command]
pub async fn show_overlay(app: tauri::AppHandle) -> Result<(), String> {
    set_click_through_for_app(&app, false)?;
    overlay_windows::show_all_overlays(&app)
}

pub fn set_click_through_for_app(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let state = app.state::<AppAudioState>();
    *state.click_through.lock().map_err(|e| e.to_string())? = enabled;

    if let Some(window) = app.get_webview_window("main") {
        window
            .set_ignore_cursor_events(enabled)
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        use tauri_nspanel::ManagerExt as _;
        if let Ok(panel) = app.get_webview_panel("main") {
            panel.set_ignore_mouse_events(enabled);
            if !enabled {
                panel.show();
            }
        }
    }

    let _ = app.emit("click-through-changed", enabled);
    Ok(())
}

pub fn toggle_click_through_for_app(app: &AppHandle) -> Result<bool, String> {
    let state = app.state::<AppAudioState>();
    let current = *state.click_through.lock().map_err(|e| e.to_string())?;
    let next = !current;
    set_click_through_for_app(app, next)?;
    Ok(next)
}

#[tauri::command]
pub async fn toggle_click_through(
    app: tauri::AppHandle,
    state: State<'_, AppAudioState>,
) -> Result<bool, String> {
    let current = *state.click_through.lock().map_err(|e| e.to_string())?;
    set_click_through_for_app(&app, !current)?;
    Ok(!current)
}

#[tauri::command]
pub async fn get_click_through(state: State<'_, AppAudioState>) -> Result<bool, String> {
    Ok(*state.click_through.lock().map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn get_stealth_status() -> Result<StealthStatusResponse, String> {
    let status = StealthWindow::platform_status();
    Ok(StealthStatusResponse {
        os_version: status.os_version,
        capture_exclusion_supported: status.capture_exclusion_supported,
        capture_exclusion_degraded: status.capture_exclusion_degraded,
        degradation_reason: status.degradation_reason,
    })
}

/// Capture screen region and run OCR (stub — returns placeholder until Tesseract wired).
#[tauri::command]
pub async fn capture_screen_ocr() -> Result<String, String> {
    Ok("Screen OCR stub: integrate Tesseract in production build.".to_string())
}

pub fn apply_stealth(window: &WebviewWindow) -> Result<(), String> {
    let stealth = StealthWindow::new(StealthConfig::default());
    let wrapper = crate::TauriStealthWindow(window.clone());
    stealth.apply(&wrapper).map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    aniki_stealth_window::macos::hide_from_dock();

    Ok(())
}
