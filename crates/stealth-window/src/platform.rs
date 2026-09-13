#[cfg(target_os = "windows")]
pub mod windows;

use crate::{PlatformStealthStatus, StealthConfig};

pub struct StealthWindow {
    config: StealthConfig,
    visible: bool,
}

impl StealthWindow {
    pub fn new(config: StealthConfig) -> Self {
        Self {
            config,
            visible: true,
        }
    }

    pub fn config(&self) -> &StealthConfig {
        &self.config
    }

    /// Apply stealth settings. Platform-specific work uses the Tauri window handle when provided.
    pub fn apply<W: StealthWindowHandle>(&self, window: &W) -> Result<(), StealthError> {
        if self.config.exclude_from_capture {
            window.set_content_protected(true)?;
        }
        if self.config.hide_from_dock {
            window.set_skip_taskbar(true)?;
        }

        #[cfg(target_os = "macos")]
        if self.config.hide_from_dock {
            crate::macos::hide_from_dock();
        }

        #[cfg(target_os = "windows")]
        if self.config.exclude_from_capture {
            if let Ok(hwnd) = window.native_window_handle() {
                windows::exclude_from_capture(hwnd)?;
            }
        }

        tracing::info!("Stealth settings applied");
        Ok(())
    }

    pub fn panic_hide(&mut self) -> Result<(), StealthError> {
        self.visible = false;
        Ok(())
    }

    pub fn show(&mut self) -> Result<(), StealthError> {
        self.visible = true;
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn platform_status() -> PlatformStealthStatus {
        let os = std::env::consts::OS;
        let version = sys_info::get_os_version();
        let degraded = cfg!(target_os = "macos");
        PlatformStealthStatus {
            os_version: format!("{os} {version}"),
            capture_exclusion_supported: true,
            capture_exclusion_degraded: degraded,
            degradation_reason: if degraded {
                Some("macOS 15+ ScreenCaptureKit may ignore sharing exclusion".to_string())
            } else {
                None
            },
        }
    }
}

pub trait StealthWindowHandle {
    fn set_content_protected(&self, enabled: bool) -> Result<(), StealthError>;
    fn set_skip_taskbar(&self, skip: bool) -> Result<(), StealthError>;
    fn native_window_handle(&self) -> Result<isize, StealthError> {
        Err(StealthError::Platform("no native handle".into()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StealthError {
    #[error("platform API error: {0}")]
    Platform(String),
    #[error("window not found")]
    WindowNotFound,
}

mod sys_info {
    pub fn get_os_version() -> String {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("sw_vers")
                .arg("-productVersion")
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .unwrap_or_default()
                .trim()
                .to_string()
        }
        #[cfg(target_os = "windows")]
        {
            "Windows".to_string()
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            "unknown".to_string()
        }
    }
}
