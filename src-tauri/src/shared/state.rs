use crate::ipc::events::CustomEvent;
use std::sync::Mutex;
use tauri::{AppHandle, Listener, Manager};

/// Represents the shared application state.
///
/// This struct holds various pieces of data that need to be accessible
/// and potentially mutable across different parts of the Tauri application.
#[derive(Default)]
pub struct AppState {
    pub authenticated: Mutex<bool>,
    pub csrf_state: Mutex<Option<String>>,
}

/// Sets up listeners for application state synchronization.
///
/// This function initializes event listeners that update the shared `AppState`
/// based on custom events emitted within the application.
pub fn set_up_state_synchronization(app_handle: &AppHandle) {
    log::info!("Setting up state synchronization...");

    let owned_app_handle = app_handle.to_owned();
    app_handle.listen(CustomEvent::Authentication.to_string(), move |event| {
        match serde_json::from_str::<bool>(event.payload()) {
            Ok(authenticated) => {
                let state = owned_app_handle.state::<AppState>();
                *state.authenticated.lock().unwrap() = authenticated;
                *state.csrf_state.lock().unwrap() = None;
            }
            Err(e) => {
                log::error!("{:?}", e);
            }
        }
    });
}
