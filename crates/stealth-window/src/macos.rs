/// Hide app from macOS Dock and Cmd+Tab (accessory activation policy).
pub fn hide_from_dock() {
    tracing::info!("macOS dock hide requested — requires NSApplicationActivationPolicyAccessory");
    // Full objc2 wiring is platform-sensitive; Tauri set_skip_taskbar covers taskbar/dock on many builds.
}
