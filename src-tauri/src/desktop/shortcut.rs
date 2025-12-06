use crate::shared::error::AppResult;
use crate::window;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_log::log;

/// Sets up the global shortcut for opening the Quick-Add dialog.
///
/// The shortcut is `Alt + Space` on macOS and `Ctrl + Space` on other operating systems.
/// When the shortcut is pressed, it attempts to open the Quick-Add dialog.
///
/// Wayland is currently unsupported.
///
/// TODO:
///  05/11/2025 Enable global shortcut and make it configurable after the following issue got resolved.
///
/// See: https://github.com/tauri-apps/global-hotkey/issues/28
#[allow(dead_code)]
pub fn set_up_global_shortcut(app_handle: &AppHandle) -> AppResult<()> {
    log::info!("Setting up global shortcut...");

    let shortcut = get_global_shortcut();
    if app_handle.global_shortcut().is_registered(shortcut) {
        log::info!("Global shortcut is already registered. Skipping the setup...");
        return Ok(());
    }
    app_handle
        .global_shortcut()
        .on_shortcut(shortcut, move |app_handle, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                log::info!("Global shortcut triggered: {}", shortcut);
                let _ = window::init_quick_add_dialog(app_handle, false)
                    .inspect_err(|e| log::error!("{e:?}"));
            }
        })?;

    Ok(())
}

/// Returns the accelerator string for the global shortcut.
pub fn get_global_shortcut_accelerator() -> String {
    get_global_shortcut().to_string()
}

/// Returns the platform-specific global shortcut.
fn get_global_shortcut() -> Shortcut {
    #[cfg(target_os = "macos")]
    {
        Shortcut::new(Some(Modifiers::ALT), Code::Space)
    }
    #[cfg(not(target_os = "macos"))]
    {
        Shortcut::new(Some(Modifiers::CONTROL), Code::Space)
    }
}
