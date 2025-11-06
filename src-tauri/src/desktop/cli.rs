use crate::shared::error::AppResult;
use anyhow::format_err;
use std::fmt;

pub const MINIMIZE_ARG: &'static str = "--minimize";

/// Represents a command-line argument that can be passed to the application.
#[derive(Debug)]
pub enum Argument {
    /// Minimize the application to the system tray on startup.
    Minimize,
    /// Open the quick add window on startup.
    QuickAdd,
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Minimize => write!(f, "minimize"),
            Argument::QuickAdd => write!(f, "quick-add"),
        }
    }
}

impl TryFrom<&str> for Argument {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> AppResult<Self> {
        match value {
            "minimize" => Ok(Self::Minimize),
            "quick-add" => Ok(Self::QuickAdd),
            _ => Err(format_err!("Unknown argument: {}", value)),
        }
    }
}
