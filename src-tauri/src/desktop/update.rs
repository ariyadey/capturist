//! This module handles the update process for the application.
//! It uses the `tauri-plugin-updater` to check for, download, and install updates.
//! The update process is spawned in a separate async runtime to avoid blocking the main thread.

use crate::shared::error::AppResult;
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

/// Sets up the updater by spawning an asynchronous task to check for and install updates.
///
/// This function is typically called during application initialization.
pub fn set_up_updater(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = check_for_updates(&app_handle).await {
            log::error!("Update check failed: {}", e);
        }
    });
}

/// Checks for, downloads, and installs application updates.
///
/// Returns Ok(()) if no update is available or if update is successfully installed.
/// Returns Err if there's a network error or installation failure.
async fn check_for_updates(app_handle: &AppHandle) -> AppResult<()> {
    let update = match app_handle.updater()?.check().await? {
        Some(update) => {
            log::info!(
                "Update available: {} -> {}",
                update.current_version,
                update.version
            );
            update
        }
        None => {
            log::info!("App is up to date");
            return Ok(());
        }
    };

    let mut downloaded = 0;
    update
        .download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                log::info!("Downloaded {}/{:?} bytes", downloaded, content_length);
            },
            || {
                log::info!("Download finished");
            },
        )
        .await?;

    log::info!("Update installed successfully. Restarting...");
    app_handle.restart();
}
