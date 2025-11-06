use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents custom events that can be emitted or listened to within the Tauri application.
///
/// These events are used for inter-process communication (IPC) between the backend Rust code
/// and the frontend webview.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CustomEvent {
    /// Emitted when an authentication-related action occurs, e.g., successful login, logout.
    Authentication,
    /// Emitted to trigger a quick add action for creating new tasks.
    QuickAdd,
    /// Emitted when an autostart-related action occurs, e.g., enabling/disabling autostart.
    Autostart,
}

impl fmt::Display for CustomEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomEvent::Authentication => write!(f, "authentication"),
            CustomEvent::QuickAdd => write!(f, "quick-add"),
            CustomEvent::Autostart => write!(f, "autostart"),
        }
    }
}
