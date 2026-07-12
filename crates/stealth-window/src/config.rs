use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Exclude window from screen capture / screen share.
    pub exclude_from_capture: bool,
    /// Hide from dock (macOS) or taskbar (Windows).
    pub hide_from_dock: bool,
    /// Exclude from Alt+Tab / Cmd+Tab switcher.
    pub exclude_from_switcher: bool,
    /// Enable click-through overlay mode.
    pub click_through: bool,
    /// Panic-hide hotkey (e.g. "Cmd+Shift+H").
    pub panic_hide_hotkey: Option<String>,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            exclude_from_capture: true,
            hide_from_dock: true,
            exclude_from_switcher: true,
            click_through: false,
            panic_hide_hotkey: Some("Cmd+Shift+H".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStealthStatus {
    pub os_version: String,
    pub capture_exclusion_supported: bool,
    pub capture_exclusion_degraded: bool,
    pub degradation_reason: Option<String>,
}
