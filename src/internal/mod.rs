#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(any(windows, target_os="macos")))]
mod linux;
#[cfg(not(any(windows, target_os="macos")))]
pub use linux::*;