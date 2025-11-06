use tauri::AppHandle;
use tauri_plugin_deep_link::DeepLinkExt;

pub const OAUTH_URI: &str = "capturist://oauth";

/// Sets up the deep link handling for the application.
pub fn set_up_deep_link_handling(app_handle: &AppHandle) {
    log::info!("Setting up deep link handling...");

    let owned_app_handle = app_handle.to_owned();
    app_handle.deep_link().on_open_url(move |event| {
        match event.urls().first() {
            Some(url) => {
                log::info!("Received deep link event with URL: {:?}", url);

                match url.as_str() {
                    OAUTH_URI => {
                        let owned_url = url.to_owned();
                        let owned_app_handle = owned_app_handle.to_owned();
                        // Spawn an async task to handle the authentication flow without blocking the event loop.
                        tauri::async_runtime::spawn(async move {
                            crate::external::todoist::auth::authenticate(
                                &owned_url,
                                &owned_app_handle,
                            )
                            .await
                        });
                    }
                    _ => log::error!("Invalid URL: {:?}", url),
                }
            }
            None => log::error!("No URL found in deep link"),
        }
    });
}
