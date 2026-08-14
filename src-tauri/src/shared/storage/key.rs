use std::fmt;

/// Represents the keys used for storing and retrieving values in various ways.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StorageKey {
    /// The access token for the Todoist API.
    TodoistToken,
    /// The refresh token used to obtain a new Todoist access token.
    TodoistRefreshToken,
    /// The Unix timestamp (seconds) at which the current access token expires.
    TodoistTokenExpiresAt,
    /// A boolean indicating whether the application should autostart.
    Autostart,
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageKey::TodoistToken => write!(f, "TODOIST_TOKEN"),
            StorageKey::TodoistRefreshToken => write!(f, "TODOIST_REFRESH_TOKEN"),
            StorageKey::TodoistTokenExpiresAt => write!(f, "TODOIST_TOKEN_EXPIRES_AT"),
            StorageKey::Autostart => write!(f, "AUTOSTART"),
        }
    }
}
