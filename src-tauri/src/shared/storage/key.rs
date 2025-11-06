use std::fmt;

/// Represents the keys used for storing and retrieving values in various ways.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StorageKey {
    /// The access token for the Todoist API.
    TodoistToken,
    /// A boolean indicating whether the application should autostart.
    Autostart,
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageKey::TodoistToken => write!(f, "TODOIST_TOKEN"),
            StorageKey::Autostart => write!(f, "AUTOSTART"),
        }
    }
}
