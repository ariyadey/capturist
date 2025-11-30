use crate::desktop::shortcut;
use crate::external::todoist::auth;
use crate::shared::error::AppSerializableResult;
use crate::shared::metadata::APP_ID;
use crate::shared::state::AppState;
use crate::shared::storage::key::StorageKey;
use crate::shared::{environment, storage};
use anyhow::{format_err, Context};
use std::process::Command;
use tauri::{AppHandle, State};

/// Checks if the application is running in debug mode.
#[tauri::command]
pub fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}

/// Checks if the current environment is Linux.
#[tauri::command]
pub fn is_os_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Checks if the current desktop session is Wayland.
#[tauri::command]
pub fn is_wayland_session() -> bool {
    environment::is_running_on_wayland()
}

/// Checks if the application is running as an AppImage.
#[tauri::command]
pub fn is_running_as_appimage() -> bool {
    environment::is_running_as_appimage()
}

/// Initiates the Todoist authentication flow.
#[tauri::command]
pub async fn start_authentication(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
) -> AppSerializableResult<()> {
    auth::start_authentication(&app_handle, &app_state).map_err(Into::into)
}

/// Returns the Todoist access token from the secure storage.
#[tauri::command]
pub async fn get_todoist_access_token(app_handle: AppHandle) -> AppSerializableResult<String> {
    storage::secure::find(StorageKey::TodoistToken, &app_handle)?
        .context("Todoist token not found")
        .map_err(Into::into)
}

/// Returns the accelerator string for the global shortcut.
#[tauri::command]
pub fn get_global_shortcut() -> String {
    shortcut::get_global_shortcut_accelerator()
}

/// Sends a desktop notification using `notify-send`.
///
/// This command is only available on Linux systems with `notify-send` installed.
/// It sends a notification with a specified title and body, and sets the application name.
///
/// Note: The Tauri notifications plugin is not used due to an issue where it
/// does not open notifications on newer Gnome versions
///
/// See: https://github.com/tauri-apps/plugins-workspace/issues/2566
///
/// TODO: Replace this implementation with Tauri Notification plugin after the issue got resolved.
#[tauri::command]
pub fn send_notification(title: &str, body: &str) -> AppSerializableResult<()> {
    Command::new("notify-send")
        .arg(title)
        .arg(body)
        .arg("--app-name")
        .arg(APP_ID)
        .arg("--hint=int:transient:1")
        .status()
        .context("Failed to execute notify-send command")
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(format_err!(
                    "notify-send command failed with status: {status:#?}"
                ))
            }
        })
        .map_err(Into::into)
}
