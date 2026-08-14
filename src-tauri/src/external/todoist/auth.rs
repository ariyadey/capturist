use crate::external::todoist;
use crate::external::todoist::sdk::PermissionScope;
use crate::ipc::events::CustomEvent;
use crate::shared::error::AppResult;
use crate::shared::state::AppState;
use crate::shared::storage;
use crate::shared::storage::key::StorageKey;
use anyhow::{ensure, Context};
use serde_json::json;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_opener::OpenerExt;

/// Refresh the access token this many seconds before it actually expires.
const REFRESH_EARLY_SECS: u64 = 60;

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
    let pkce_verifier = todoist::sdk::get_pkce_verifier();
    let pkce_challenge = todoist::sdk::get_pkce_challenge(&pkce_verifier);
    *app_state.csrf_state.lock().unwrap() = Some(csrf_state.to_owned());
    *app_state.pkce_verifier.lock().unwrap() = Some(pkce_verifier);
    let url = todoist::sdk::get_authorization_url(
        client_id,
        permission_scopes,
        &csrf_state,
        &pkce_challenge,
    )?;
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
        .take()
        .unwrap_or_default();
    let code_verifier = app_handle
        .state::<AppState>()
        .pkce_verifier
        .lock()
        .unwrap()
        .take()
        .unwrap_or_default();

    ensure!(payload.state == stored_state,
        "OAuth state mismatch. Potential CSRF attack detected. URL: {:?}, State: {}, Stored State: {}",
        url,
        payload.state,
        stored_state
    );

    let response =
        todoist::sdk::get_auth_token(todoist::TODOIST_CLIENT_ID, &payload.code, &code_verifier)
            .await?;
    store_tokens(&response, app_handle)?;
    app_handle.emit(&CustomEvent::Authentication.to_string(), json!(true))?;

    Ok(())
}

/// Returns a Todoist access token that is valid for the near future, refreshing
/// it first if it is about to expire or has already expired.
pub async fn get_valid_access_token(app_handle: &AppHandle) -> AppResult<String> {
    let expires_at_secs = storage::secure::find(StorageKey::TodoistTokenExpiresAt, app_handle)?
        .context("No Todoist token expiration entry; please sign in again")
        .inspect_err(|_| log_out(app_handle).unwrap())?
        .parse::<u64>()
        .context("Invalid Todoist token expiration entry; please sign in again")
        .inspect_err(|_| log_out(app_handle).unwrap())?;

    let now_secs = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    if expires_at_secs <= now_secs + REFRESH_EARLY_SECS {
        refresh_stored_token(app_handle).await?;
    }

    storage::secure::find(StorageKey::TodoistToken, app_handle)?
        .context("No Todoist token found; please sign in again")
        .inspect_err(|_| log_out(app_handle).unwrap())
}

/// Refreshes the stored access token using the stored refresh token.
///
/// Todoist rotates the refresh token on every refresh, so the response replaces
/// the previously stored one.
pub async fn refresh_stored_token(app_handle: &AppHandle) -> AppResult<()> {
    let refresh_token = storage::secure::find(StorageKey::TodoistRefreshToken, app_handle)?
        .context("No Todoist refresh token available; please sign in again")
        .inspect_err(|_| log_out(app_handle).unwrap())?;
    let response =
        todoist::sdk::refresh_access_token(todoist::TODOIST_CLIENT_ID, &refresh_token).await?;
    store_tokens(&response, app_handle)?;
    Ok(())
}

/// Logs out the user by clearing user data and emitting an authentication event.
pub fn log_out(app_handle: &AppHandle) -> AppResult<()> {
    storage::secure::delete(StorageKey::TodoistToken, app_handle)?;
    storage::secure::delete(StorageKey::TodoistTokenExpiresAt, app_handle)?;
    storage::secure::delete(StorageKey::TodoistRefreshToken, app_handle)?;
    app_handle.emit(&CustomEvent::Authentication.to_string(), json!(false))?;

    Ok(())
}

/// Persists the access token, its expiry and the refresh token from a token response.
fn store_tokens(
    response: &todoist::sdk::AccessTokenResponse,
    app_handle: &AppHandle,
) -> AppResult<()> {
    storage::secure::set(StorageKey::TodoistToken, &response.access_token, app_handle)?;
    let expires_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .checked_add(Duration::from_secs(response.expires_in))
        .with_context(|| "Expires timestamp overflow")?
        .as_secs();
    storage::secure::set(
        StorageKey::TodoistTokenExpiresAt,
        &expires_at.to_string(),
        app_handle,
    )?;
    if let Some(refresh_token) = &response.refresh_token {
        storage::secure::set(StorageKey::TodoistRefreshToken, refresh_token, app_handle)?;
    }
    Ok(())
}
