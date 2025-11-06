//! This module contains external related to the Todoist API.

pub mod auth;
pub mod sdk;

/// The client ID for the Todoist API.
pub const TODOIST_CLIENT_ID: &'static str = env!("TODOIST_CLIENT_ID");

/// The client secret for the Todoist API.
pub const TODOIST_CLIENT_SECRET: &'static str = env!("TODOIST_CLIENT_SECRET");
