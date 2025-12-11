use std::env;
use std::ops::Not;

/// Checks if the application is currently running on a Wayland display server.
///
/// This is determined by checking the `XDG_SESSION_TYPE` environment variable.
pub fn is_running_on_wayland() -> bool {
    env::var("XDG_SESSION_TYPE").is_ok_and(|value| value == "wayland")
}

/// Checks if the application is currently running as a Snap package.
///
/// This is determined by checking for the presence of the `SNAP` environment variable.
pub fn is_running_as_snap() -> bool {
    cfg!(dev).not() && env::var("SNAP").is_ok()
}

/// Checks if the application is currently running as a Flatpak.
///
/// This is determined by checking for the presence of the `FLATPAK_ID` environment variable.
pub fn is_running_as_flatpak() -> bool {
    env::var("FLATPAK_ID").is_ok()
}

/// Checks if the application is currently running as an AppImage.
///
/// This is determined by checking for the presence of the `APPIMAGE` environment variable.
pub fn is_running_as_appimage() -> bool {
    env::var("APPIMAGE").is_ok()
}
