use crate::ipc::events::CustomEvent;
use crate::shared::error::AppResult;
use crate::shared::storage::general;
use crate::shared::storage::key::StorageKey;
use anyhow::Context;
use tauri::{AppHandle, Listener};
use tauri_plugin_autostart::ManagerExt;

/// Sets up the autostart feature based on user preferences.
///
/// This function reads the `Autostart` setting and enables or disables the autostart feature accordingly.
pub fn set_up_autostart(app_handle: &AppHandle) -> AppResult<()> {
    log::info!("Setting up autostart...");

    let should_autostart =
        general::find(StorageKey::Autostart, app_handle).map(|b| b.unwrap_or(true));
    toggle_autostart(should_autostart, app_handle)?;

    let owned_app_handle = app_handle.to_owned();
    app_handle.listen(CustomEvent::Autostart.to_string(), move |event| {
        let should_autostart =
            serde_json::from_str(event.payload()).context("Failed to deserialize autostart event");
        let _ = toggle_autostart(should_autostart, &owned_app_handle)
            .inspect_err(|e| log::error!("{e:?}"));
    });

    Ok(())
}

/// Applies the autostart changes based on user preferences.
fn toggle_autostart(enable: AppResult<bool>, app_handle: &AppHandle) -> AppResult<()> {
    let should_autostart = enable?;
    let currently_enabled_autostart = app_handle.autolaunch().is_enabled()?;

    log::info!(
        "Autostart feature is currently {:?}.",
        if currently_enabled_autostart {
            "enabled"
        } else {
            "disabled"
        }
    );

    if should_autostart && !currently_enabled_autostart {
        log::info!("Enabling the autostart feature based on user preferences...");
        app_handle.autolaunch().enable()?;
    } else if !should_autostart && currently_enabled_autostart {
        log::info!("Disabling the autostart feature based on user preferences...");
        app_handle.autolaunch().disable()?;
    }

    general::set(StorageKey::Autostart, should_autostart, &app_handle)?;

    Ok(())
}
