/// Exclude window from screen capture on Windows.
#[cfg(target_os = "windows")]
pub fn exclude_from_capture(hwnd: isize) -> Result<(), crate::StealthError> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{SetWindowDisplayAffinity, WDA_EXCLUDEFROMCAPTURE};

    unsafe {
        SetWindowDisplayAffinity(HWND(hwnd), WDA_EXCLUDEFROMCAPTURE)
            .map_err(|e| crate::StealthError::Platform(e.to_string()))?;
    }
    Ok(())
}
