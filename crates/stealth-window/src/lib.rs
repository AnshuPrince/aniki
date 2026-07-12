mod config;
mod platform;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

pub use config::*;
pub use platform::*;
