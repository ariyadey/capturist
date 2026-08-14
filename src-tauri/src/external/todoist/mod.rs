//! This module contains external related to the Todoist API.

pub mod auth;
pub mod sdk;

/// The client ID for the Todoist API.
///
/// The client is a public OAuth client using PKCE, so no client secret exists.
pub const TODOIST_CLIENT_ID: &str = env!("TODOIST_CLIENT_ID");
