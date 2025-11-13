use crate::shared::error::AppResult;
use anyhow::Context;
use rand::Rng;
use serde::Deserialize;
use std::fmt;
use url::Url;

/// Represents the response received when exchanging an authorization code for an access token.
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    /// The access token to be used for authenticating API requests.
    pub access_token: String,
    /// The type of token, typically "Bearer".
    #[allow(dead_code)]
    pub token_type: String,
}

/// Represents the data received in the callback from the Todoist OAuth authorization flow.
/// This struct is used to deserialize the query parameters from the callback URL.
#[derive(Debug, Deserialize)]
pub struct AuthCallbackResponse {
    pub code: String,
    pub state: String,
}

/// Represents the permission scopes for the Todoist API.
#[allow(dead_code)]
pub enum PermissionScope {
    /// Grants permission to add new tasks (the application cannot read or modify any existing data).
    TaskAdd,
    /// Grants read-only access to application data, including tasks, projects, labels, and filters.
    DataRead,
    /// Grants read and write access to application data, including tasks, projects, labels, and filters. This scope includes `task:add` and `data:read` scopes.
    DataReadWrite,
    /// Grants permission to delete application data, including tasks, labels, and filters.
    DataDelete,
    /// Grants permission to delete projects.
    ProjectDelete,
    /// Grants permission to list backups bypassing MFA requirements.
    BackupsRead,
}

impl fmt::Display for PermissionScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PermissionScope::TaskAdd => write!(f, "task:add"),
            PermissionScope::DataRead => write!(f, "data:read"),
            PermissionScope::DataReadWrite => write!(f, "data:read_write"),
            PermissionScope::DataDelete => write!(f, "data:delete"),
            PermissionScope::ProjectDelete => write!(f, "project:delete"),
            PermissionScope::BackupsRead => write!(f, "backups:read"),
        }
    }
}

/// Generates a secure, random 24-character alphanumeric string to be used
/// as the `state` parameter in an OAuth2 flow.
///
/// This is the equivalent of `getAuthStateParameter`.
pub fn get_auth_state_parameter() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(24)
        .map(char::from)
        .collect()
}

/// Constructs the full Todoist authorization URL.
///
/// This is the equivalent of `getAuthorizationUrl`.
pub fn get_authorization_url(
    client_id: &str,
    scopes: &[PermissionScope],
    state: &str,
) -> AppResult<Url> {
    let mut url = Url::parse("https://todoist.com/oauth/authorize")
        .context("Failed to parse Todoist authorization base URL")?;
    let scopes_str = scopes
        .iter()
        .map(|scope| scope.to_string())
        .collect::<Vec<_>>()
        .join(",");
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("scope", &scopes_str)
        .append_pair("state", &state);
    Ok(url)
}

/// Exchanges an authorization code for an access token.
///
/// This is the equivalent of `getAuthToken`.
pub async fn get_auth_token(
    client_id: &str,
    client_secret: &str,
    code: &str,
) -> AppResult<AccessTokenResponse> {
    let response = reqwest::Client::new()
        .post("https://todoist.com/oauth/access_token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;
    Ok(response)
}
