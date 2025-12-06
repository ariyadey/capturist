use crate::ipc;
use crate::shared::error::AppResult;
use anyhow::Context;
use ipc::events::CustomEvent;
use std::fmt;
use tauri::{AppHandle, Listener, Manager, WebviewWindow, WebviewWindowBuilder};

/// Represents the different types of windows that can be opened in the application.
#[derive(Debug, Eq, PartialEq, Hash)]
enum WindowLabel {
    /// The Quick-Add window for adding tasks.
    QuickAdd,
    /// The Authentication window for logging into Todoist.
    Authentication,
}

impl fmt::Display for WindowLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowLabel::QuickAdd => write!(f, "quick-add"),
            WindowLabel::Authentication => write!(f, "authentication"),
        }
    }
}

impl From<&str> for WindowLabel {
    fn from(value: &str) -> Self {
        match value {
            "quick-add" => WindowLabel::QuickAdd,
            "authentication" => WindowLabel::Authentication,
            _ => panic!("Unknown window label: {}", value),
        }
    }
}

/// Sets up synchronization for the current window.
///
/// This function listens for the `Authentication` event and opens either
/// the Quick-Add dialog or the Authentication window based on the authentication status.
pub fn set_up_current_window_synchronization(app_handle: &AppHandle) {
    log::info!("Setting up current window synchronization...");

    let owned_app_handle = app_handle.to_owned();
    app_handle.listen(CustomEvent::Authentication.to_string(), move |event| {
        let _ = serde_json::from_str::<bool>(event.payload())
            .context("Failed to parse authentication status from event payload")
            .and_then(|authenticated| {
                if authenticated {
                    switch_to_quick_add_dialog(&owned_app_handle)
                } else {
                    switch_to_authentication_window(&owned_app_handle)
                }
            })
            .inspect_err(|e| log::error!("{e:?}"));
    });
}

/// Opens the Quick-Add dialog window.
///
/// If the Quick-Add window already exists, it is shown. Otherwise, a new one is created.
pub fn init_quick_add_dialog(app_handle: &AppHandle, minimize: bool) -> AppResult<()> {
    log::info!("Opening the Quick-Add dialog...");

    let window = app_handle
        .get_webview_window(&WindowLabel::QuickAdd.to_string())
        .ok_or(tauri::Error::WebviewNotFound)
        .or_else(|_| create_window(WindowLabel::QuickAdd, app_handle))?;
    window.set_resizable(false)?;
    if minimize {
        window.hide()?;
    } else {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}

/// Opens the Authentication window.
///
/// If the Authentication window already exists, it is shown. Otherwise, a new one is created.
pub fn init_authentication_window(app_handle: &AppHandle, minimize: bool) -> AppResult<()> {
    log::info!("Opening the Authentication window...");

    let window = app_handle
        .get_webview_window(&WindowLabel::Authentication.to_string())
        .ok_or(tauri::Error::WebviewNotFound)
        .or_else(|_| create_window(WindowLabel::Authentication, app_handle))?;
    if minimize {
        window.hide()?;
    } else {
        window.show()?;
        window.set_focus()?;
    }

    Ok(())
}

/// Switches the application to the Quick-Add dialog.
///
/// This function destroys all existing webview windows except the Quick-Add dialog,
/// and then initializes and shows the Quick-Add dialog.
fn switch_to_quick_add_dialog(app_handle: &AppHandle) -> AppResult<()> {
    init_quick_add_dialog(app_handle, false)?;
    app_handle
        .webview_windows()
        .values()
        .filter(|w| w.label() != WindowLabel::QuickAdd.to_string())
        .try_for_each(|window| window.destroy())?;

    Ok(())
}

/// Switches the application to the Authentication window.
///
/// This function destroys all existing webview windows except the Authentication window,
/// and then initializes and shows the Authentication window.
fn switch_to_authentication_window(app_handle: &AppHandle) -> AppResult<()> {
    init_authentication_window(app_handle, false)?;
    app_handle
        .webview_windows()
        .to_owned()
        .values()
        .filter(|w| w.label() != WindowLabel::Authentication.to_string())
        .try_for_each(|window| window.destroy())?;

    Ok(())
}

/// Creates a new webview window based on the provided `WindowLabel`.
///
/// This function retrieves the window configuration from `tauri.conf.json` based on the
/// `WindowLabel` and builds a new `WebviewWindow`.
fn create_window(window_label: WindowLabel, app_handle: &AppHandle) -> AppResult<WebviewWindow> {
    log::info!("Creating a new webview window with label: {window_label:?}...",);

    let window_config = app_handle
        .config()
        .app
        .windows
        .iter()
        .find(|w| w.label == window_label.to_string())
        .ok_or(tauri::Error::WebviewNotFound)?;
    let window = WebviewWindowBuilder::from_config(app_handle, window_config)?.build()?;

    Ok(window)
}
