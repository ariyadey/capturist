use crate::external::todoist;
use crate::external::todoist::sdk::PermissionScope;
use crate::ipc::events::CustomEvent;
use crate::shared::error::AppResult;
use crate::shared::state::AppState;
use crate::shared::storage;
use crate::shared::storage::key::StorageKey;
use anyhow::{ensure, Context};
use serde_json::json;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_opener::OpenerExt;

/// Initiates the Todoist authentication flow.
///
/// This command generates a CSRF state, constructs the Todoist authorization URL,
/// and opens it in the user's default browser. The CSRF state is stored in the
/// application state for later verification.
pub fn start_authentication(
    app_handle: &AppHandle,
    app_state: &State<'_, AppState>,
) -> AppResult<()> {
    let client_id = todoist::TODOIST_CLIENT_ID;
    let permission_scopes = &[PermissionScope::TaskAdd];
    let csrf_state = todoist::sdk::get_auth_state_parameter();
    *app_state.csrf_state.lock().unwrap() = Some(csrf_state.to_owned());
    let url = todoist::sdk::get_authorization_url(client_id, permission_scopes, &csrf_state)?;
    app_handle.opener().open_url(url.as_str(), None::<&str>)?;

    Ok(())
}

/// This async function is spawned as a new task when a deep link is received.
/// It handles the entire backend authentication flow.
pub async fn authenticate(url: &tauri::Url, app_handle: &AppHandle) -> AppResult<()> {
    let query = url.query().context("Missing query parameters")?;
    let payload = serde_urlencoded::from_str::<todoist::sdk::AuthCallbackResponse>(query)
        .context("Invalid query parameters")?;
    let stored_state = app_handle
        .state::<AppState>()
        .csrf_state
        .lock()
        .unwrap()
        .to_owned()
        .unwrap_or_default();

    ensure!(payload.state == stored_state,
        "OAuth state mismatch. Potential CSRF attack detected. URL: {:?}, State: {}, Stored State: {}",
        url,
        payload.state,
        stored_state
    );

    let client_id = todoist::TODOIST_CLIENT_ID;
    let client_secret = todoist::TODOIST_CLIENT_SECRET;
    let token = todoist::sdk::get_auth_token(&client_id, &client_secret, &payload.code)
        .await?
        .access_token;
    storage::secure::set(StorageKey::TodoistToken, &token, app_handle)?;
    app_handle.emit(&CustomEvent::Authentication.to_string(), json!(true))?;

    Ok(())
}

/// Logs out the user by clearing user data and emitting an authentication event.
pub fn log_out(app_handle: &AppHandle) -> AppResult<()> {
    storage::secure::delete(StorageKey::TodoistToken, app_handle)?;
    app_handle.emit(&CustomEvent::Authentication.to_string(), json!(false))?;

    Ok(())
}
