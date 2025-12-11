use crate::shared::environment;
use crate::shared::error::AppResult;
use anyhow::{format_err, Context};
use std::fmt;
use std::ops::Not;
use tauri::AppHandle;
use tauri_plugin_deep_link::DeepLinkExt;

/// Represents the host part of a deep link URL, used to
/// differentiate between different deep link purposes.
#[derive(Debug)]
pub enum DeepLinkHost {
    /// Represents a deep link for OAuth authentication.
    ///
    /// Example: `capturist://oauth?...`
    Oauth,
}

impl fmt::Display for DeepLinkHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeepLinkHost::Oauth => write!(f, "oauth"),
        }
    }
}

impl TryFrom<&str> for DeepLinkHost {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> AppResult<Self> {
        match value {
            "oauth" => Ok(DeepLinkHost::Oauth),
            _ => Err(format_err!("Unknown deep-link host: {}", value)),
        }
    }
}

/// Sets up the deep link handling for the application.
pub fn set_up_deep_link_handling(app_handle: &AppHandle) -> AppResult<()> {
    log::info!("Setting up deep link handling...");

    if environment::is_running_as_snap().not() && environment::is_running_as_flatpak().not() {
        app_handle.deep_link().register_all()?;
    }

    let owned_app_handle = app_handle.to_owned();
    app_handle.deep_link().on_open_url(move |event| {
        match event.urls().first() {
            Some(url) => {
                log::info!("Received deep link event with URL: {url:?}");

                match url
                    .host_str()
                    .context("Invalid URL")
                    .and_then(DeepLinkHost::try_from)
                {
                    Ok(DeepLinkHost::Oauth) => {
                        let owned_url = url.to_owned();
                        let owned_app_handle = owned_app_handle.to_owned();
                        // Spawns an async task to handle the authentication flow
                        // without blocking the event loop.
                        tauri::async_runtime::spawn(async move {
                            crate::external::todoist::auth::authenticate(
                                &owned_url,
                                &owned_app_handle,
                            )
                            .await
                        });
                    }
                    _ => log::error!("Invalid URL: {url:?}"),
                }
            }
            None => log::error!("No URL found in deep link"),
        }
    });

    Ok(())
}
