use crate::shared::error::AppResult;
use anyhow::Context;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::RngExt;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fmt;
use url::Url;

/// The OAuth authorization endpoint.
const AUTHORIZE_ENDPOINT: &str = "https://app.todoist.com/oauth/authorize";

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
    pub expires_in: u64,
    /// The refresh token. Rotated on every refresh; omitted on grace-window retries.
    pub refresh_token: Option<String>,
    /// The scopes granted to the token (comma-separated for App Console apps,
    /// space-separated for DCR/CIMD apps).
    #[allow(dead_code)]
    pub scope: Option<String>,
}

/// Represents the query parameters received in the callback from the Todoist
/// OAuth authorization flow.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum AuthCallbackResponse {
    /// Returned when the user cancels consent or Todoist rejects the request
    /// (e.g. `?error=access_denied`).
    Error {
        error: String,
        error_description: Option<String>,
    },
    /// Returned when the user approves the authorization request.
    Success { code: String, state: String },
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
    /// Grants read-only access to billing information.
    BillingRead,
    /// Grants read and write access to billing information.
    BillingReadWrite,
    /// Grants access to the developer App Console API.
    DevAppConsole,
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
            PermissionScope::BillingRead => write!(f, "billing:read"),
            PermissionScope::BillingReadWrite => write!(f, "billing:read_write"),
            PermissionScope::DevAppConsole => write!(f, "dev:app_console"),
        }
    }
}

/// Error returned by Todoist's OAuth token endpoint.
#[derive(Debug, Eq, PartialEq)]
pub struct OAuthTokenError {
    pub status: u16,
    pub error: Option<String>,
    pub description: Option<String>,
}

impl OAuthTokenError {
    /// Returns `true` when Todoist rejected the grant/token itself (HTTP 400 or 401),
    /// indicating the stored credentials cannot be reused and the user must sign in again.
    pub fn is_token_rejected(&self) -> bool {
        matches!(self.status, 400 | 401)
    }
}

impl fmt::Display for OAuthTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Todoist OAuth token request failed (HTTP {})",
            self.status
        )?;
        if let Some(error) = &self.error {
            write!(f, ": {error}")?;
        }
        if let Some(description) = &self.description {
            write!(f, " ({description})")?;
        }
        Ok(())
    }
}

impl std::error::Error for OAuthTokenError {}

#[derive(Deserialize)]
struct OAuthErrorBody {
    error: Option<String>,
    error_description: Option<String>,
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
    let mut url =
        Url::parse(AUTHORIZE_ENDPOINT).context("Failed to parse Todoist authorization base URL")?;
    let scopes_str = scopes
        .iter()
        .map(|scope| scope.to_string())
        .collect::<Vec<_>>()
        .join(",");
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("scope", &scopes_str)
        .append_pair("state", state)
        .append_pair("response_type", "code")
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
    post_token_request(&[
        ("client_id", client_id),
        ("grant_type", "authorization_code"),
        ("code", code),
        ("code_verifier", code_verifier),
    ])
    .await
}

/// Exchanges a refresh token for a new access token.
///
/// Todoist rotates the refresh token on every refresh, so the response must be
/// stored in place of the previously used one.
pub async fn refresh_access_token(
    client_id: &str,
    refresh_token: &str,
) -> AppResult<AccessTokenResponse> {
    post_token_request(&[
        ("client_id", client_id),
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
    ])
    .await
}

async fn post_token_request(form: &[(&str, &str)]) -> AppResult<AccessTokenResponse> {
    let response = reqwest::Client::new()
        .post(ACCESS_TOKEN_ENDPOINT)
        .form(form)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    parse_token_response(status, &body)
}

fn parse_token_response(status: reqwest::StatusCode, body: &str) -> AppResult<AccessTokenResponse> {
    if status.is_success() {
        return serde_json::from_str::<AccessTokenResponse>(body)
            .context("Failed to decode Todoist OAuth token response");
    }

    let parsed = serde_json::from_str::<OAuthErrorBody>(body).ok();
    Err(OAuthTokenError {
        status: status.as_u16(),
        error: parsed.as_ref().and_then(|b| b.error.clone()),
        description: parsed.and_then(|b| b.error_description),
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::{
        get_authorization_url, parse_token_response, AuthCallbackResponse, OAuthTokenError,
        PermissionScope,
    };
    use reqwest::StatusCode;
    use std::collections::HashMap;

    #[test]
    fn builds_authorization_url_with_pkce_and_response_type() {
        let url = get_authorization_url(
            "cid",
            &[PermissionScope::TaskAdd, PermissionScope::DataRead],
            "st123",
            "chal456",
        )
        .unwrap();
        assert_eq!(
            url.origin().ascii_serialization(),
            "https://app.todoist.com"
        );
        assert_eq!(url.path(), "/oauth/authorize");
        let params: HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(params.get("client_id").map(String::as_str), Some("cid"));
        assert_eq!(
            params.get("scope").map(String::as_str),
            Some("task:add,data:read")
        );
        assert_eq!(params.get("state").map(String::as_str), Some("st123"));
        assert_eq!(
            params.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            params.get("code_challenge").map(String::as_str),
            Some("chal456")
        );
        assert_eq!(
            params.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );
    }

    #[test]
    fn parses_successful_token_response() {
        let body = r#"{"access_token":"tok","token_type":"Bearer","expires_in":3600,"refresh_token":"ref","scope":"task:add"}"#;
        let parsed = parse_token_response(StatusCode::OK, body).unwrap();
        assert_eq!(parsed.access_token, "tok");
        assert_eq!(parsed.expires_in, 3600);
        assert_eq!(parsed.refresh_token.as_deref(), Some("ref"));
        assert_eq!(parsed.scope.as_deref(), Some("task:add"));
    }

    #[test]
    fn parses_auth_callback_success_and_error() {
        let ok: AuthCallbackResponse = serde_urlencoded::from_str("code=abc&state=xyz").unwrap();
        assert_eq!(
            ok,
            AuthCallbackResponse::Success {
                code: "abc".into(),
                state: "xyz".into(),
            }
        );

        let denied: AuthCallbackResponse =
            serde_urlencoded::from_str("error=access_denied").unwrap();
        assert_eq!(
            denied,
            AuthCallbackResponse::Error {
                error: "access_denied".into(),
                error_description: None,
            }
        );

        let detailed: AuthCallbackResponse =
            serde_urlencoded::from_str("error=invalid_scope&error_description=unknown+scope")
                .unwrap();
        assert_eq!(
            detailed,
            AuthCallbackResponse::Error {
                error: "invalid_scope".into(),
                error_description: Some("unknown scope".into()),
            }
        );

        assert!(serde_urlencoded::from_str::<AuthCallbackResponse>("foo=bar").is_err());
    }

    #[test]
    fn formats_all_permission_scopes() {
        assert_eq!(PermissionScope::TaskAdd.to_string(), "task:add");
        assert_eq!(PermissionScope::DataRead.to_string(), "data:read");
        assert_eq!(
            PermissionScope::DataReadWrite.to_string(),
            "data:read_write"
        );
        assert_eq!(PermissionScope::DataDelete.to_string(), "data:delete");
        assert_eq!(PermissionScope::ProjectDelete.to_string(), "project:delete");
        assert_eq!(PermissionScope::BackupsRead.to_string(), "backups:read");
        assert_eq!(PermissionScope::BillingRead.to_string(), "billing:read");
        assert_eq!(
            PermissionScope::BillingReadWrite.to_string(),
            "billing:read_write"
        );
        assert_eq!(
            PermissionScope::DevAppConsole.to_string(),
            "dev:app_console"
        );
    }

    #[test]
    fn treats_oauth_400_as_rejected_token() {
        let err = parse_token_response(
            StatusCode::BAD_REQUEST,
            r#"{"error":"invalid_grant","error_description":"refresh token expired"}"#,
        )
        .unwrap_err();
        let oauth_err = err.downcast_ref::<OAuthTokenError>().unwrap();
        assert!(oauth_err.is_token_rejected());
        assert_eq!(oauth_err.error.as_deref(), Some("invalid_grant"));
        assert_eq!(
            oauth_err.description.as_deref(),
            Some("refresh token expired")
        );
    }

    #[test]
    fn treats_server_error_as_transient() {
        let err =
            parse_token_response(StatusCode::SERVICE_UNAVAILABLE, "<html>503</html>").unwrap_err();
        let oauth_err = err.downcast_ref::<OAuthTokenError>().unwrap();
        assert!(!oauth_err.is_token_rejected());
        assert_eq!(oauth_err.status, 503);
        assert_eq!(oauth_err.error, None);
    }
}
