use crate::external::todoist::auth;
use crate::shared::error::AppSerializableResult;
use crate::shared::state::AppState;
use crate::shared::storage;
use crate::shared::storage::key::StorageKey;
use anyhow::Context;
use tauri::{AppHandle, State};

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
pub async fn get_todoist_access_token() -> AppSerializableResult<String> {
    storage::keyring::find(StorageKey::TodoistToken)?
        .context("Todoist token not found")
        .map_err(Into::into)
}
