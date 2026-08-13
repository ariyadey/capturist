use crate::shared::error::AppResult;
use anyhow::Context;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::RngExt;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fmt;
use url::Url;

/// The OAuth token endpoint.
const ACCESS_TOKEN_ENDPOINT: &str = "https://api.todoist.com/oauth/access_token";

/// Represents the response received when exchanging an authorization code (or a
/// refresh token) for an access token.
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    /// The access token to be used for authenticating API requests.
    pub access_token: String,
    /// The type of token, typically "Bearer".
    #[allow(dead_code)]
    pub token_type: String,
    /// The access token lifetime in seconds. Present for apps with refresh tokens enabled.
    #[allow(dead_code)]
    pub expires_in: u64,
    /// The refresh token. Rotated on every refresh; only present once in any response.
    pub refresh_token: Option<String>,
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

/// Generates a random PKCE `code_verifier`.
///
/// A 64-character string of alphanumeric characters, within the 43-128 character
/// range required by RFC 7636.
pub fn get_pkce_verifier() -> String {
    rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

/// Computes the S256 PKCE `code_challenge` for the given `code_verifier`.
pub fn get_pkce_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

/// Constructs the full Todoist authorization URL.
///
/// This is the equivalent of `getAuthorizationUrl`.
pub fn get_authorization_url(
    client_id: &str,
    scopes: &[PermissionScope],
    state: &str,
    code_challenge: &str,
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
        .append_pair("state", state)
        .append_pair("code_challenge", code_challenge)
        .append_pair("code_challenge_method", "S256");
    Ok(url)
}

/// Exchanges an authorization code for an access token.
///
/// This is the equivalent of `getAuthToken`, using a PKCE public client so no
/// client secret is required.
pub async fn get_auth_token(
    client_id: &str,
    code: &str,
    code_verifier: &str,
) -> AppResult<AccessTokenResponse> {
    let response = reqwest::Client::new()
        .post(ACCESS_TOKEN_ENDPOINT)
        .form(&[
            ("client_id", client_id),
            ("code", code),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;
    Ok(response)
}

/// Exchanges a refresh token for a new access token.
///
/// Todoist rotates the refresh token on every refresh, so the response must be
/// stored in place of the previously used one.
pub async fn refresh_access_token(
    client_id: &str,
    refresh_token: &str,
) -> AppResult<AccessTokenResponse> {
    let response = reqwest::Client::new()
        .post(ACCESS_TOKEN_ENDPOINT)
        .form(&[
            ("client_id", client_id),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;
    Ok(response)
}
