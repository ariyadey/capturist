//! This module manages the system tray icon and its associated menu,
//! allowing users to interact with the application directly from the system tray.

use crate::desktop::window;
use crate::external::todoist::auth;
use crate::ipc::events::CustomEvent;
use crate::shared::environment;
use crate::shared::error::AppResult;
use crate::shared::metadata::APP_TITLE;
use crate::shared::state::AppState;
use anyhow::{format_err, Context};
use serde_json::json;
use std::fmt;
use tauri::menu::{CheckMenuItem, Menu, MenuBuilder, MenuEvent, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Event, Listener, Manager, Wry};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_log::log;

/// Represents the unique identifiers for the menu items in the system tray.
///
/// These IDs are used to distinguish between different menu actions when an event is triggered.
#[derive(Debug, PartialEq, Eq, Hash)]
enum MenuId {
    /// Opens the Quick-Add dialog.
    QuickAdd,
    /// Opens the settings window.
    Settings,
    /// Toggles the application's autostart setting.
    AutoStart,
    /// Logs the user out of the application.
    LogOut,
    /// Quits the application.
    Quit,
}

impl fmt::Display for MenuId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuId::QuickAdd => write!(f, "quick-add"),
            MenuId::Settings => write!(f, "settings"),
            MenuId::AutoStart => write!(f, "autostart"),
            MenuId::LogOut => write!(f, "log-out"),
            MenuId::Quit => write!(f, "quit"),
        }
    }
}

impl TryFrom<&str> for MenuId {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "quick-add" => Ok(Self::QuickAdd),
            "settings" => Ok(Self::Settings),
            "autostart" => Ok(Self::AutoStart),
            "log-out" => Ok(Self::LogOut),
            "quit" => Ok(Self::Quit),
            _ => Err("Unknown menu ID."),
        }
    }
}

/// The unique identifier for the system tray icon.
const TRAY_ID: &'static str = "capturist-tray";

/// The unique identifier for the system tray menu.
const TRAY_MENU_ID: &'static str = "capturist-tray-menu";

/// The title/tooltip of the system tray icon.
const TRAY_TITLE: &'static str = APP_TITLE;

/// Sets up the system tray icon and its associated menu.
pub fn set_up_tray_menu(app_handle: &AppHandle) -> AppResult<()> {
    log::info!("Setting up the tray menu...");

    let tray_icon = get_tray_icon(app_handle)?;
    let tray_menu = get_tray_menu(app_handle)?;

    let owned_tray_menu = tray_menu.to_owned();
    TrayIconBuilder::with_id(TRAY_ID)
        .title(TRAY_TITLE)
        .tooltip(TRAY_TITLE)
        .icon(tray_icon)
        .menu(&owned_tray_menu)
        .on_menu_event(move |app_handle, event| {
            handle_menu_event(app_handle, event, &owned_tray_menu)
        })
        .build(app_handle)?;

    let owned_tray_menu = tray_menu.to_owned();
    app_handle.listen(CustomEvent::Authentication.to_string(), move |event| {
        let _ = on_authentication_state_change(&owned_tray_menu, event)
            .inspect_err(|e| log::error!("{e:?}"));
    });

    Ok(())
}

/// Retrieves the application's default window icon for use in the tray.
fn get_tray_icon(app_handle: &AppHandle) -> AppResult<tauri::image::Image<'_>> {
    app_handle
        .default_window_icon()
        .context("Default window icon not found.")
        .map(|image| image.to_owned())
}

/// Constructs the system tray menu.
fn get_tray_menu(app_handle: &AppHandle) -> AppResult<Menu<Wry>> {
    let user_authenticated = app_handle
        .state::<AppState>()
        .authenticated
        .lock()
        .unwrap()
        .to_owned();

    let mut menu_builder = MenuBuilder::with_id(app_handle, TRAY_MENU_ID).item(&MenuItem::with_id(
        app_handle,
        MenuId::QuickAdd.to_string(),
        "Add a new task",
        user_authenticated,
        None::<String>,
    )?);

    if !environment::is_running_as_snap() {
        menu_builder = menu_builder.separator().item(&CheckMenuItem::with_id(
            app_handle,
            MenuId::AutoStart.to_string(),
            "Launch at startup",
            true,
            app_handle.autolaunch().is_enabled()?,
            None::<String>,
        )?);
    }

    menu_builder
        .separator()
        .item(&MenuItem::with_id(
            app_handle,
            MenuId::LogOut.to_string(),
            "Log out",
            user_authenticated,
            None::<String>,
        )?)
        .text(MenuId::Quit.to_string(), "Quit Capturist")
        .build()
        .context("Failed to build the tray menu.")
}

/// Handles events triggered by interactions with the system tray menu.
fn handle_menu_event(app_handle: &AppHandle, event: MenuEvent, menu: &Menu<Wry>) {
    match event.id().as_ref().try_into() {
        Ok(menu_id) => match menu_id {
            MenuId::QuickAdd => {
                let _ = window::init_quick_add_dialog(app_handle, false)
                    .inspect_err(|e| log::error!("{e:?}"));
            }
            MenuId::AutoStart => {
                let _ = toggle_autostart(app_handle, menu).inspect_err(|e| log::error!("{e:?}"));
            }
            MenuId::LogOut => {
                let _ = auth::log_out(app_handle).inspect_err(|e| log::error!("{e:?}"));
            }
            MenuId::Quit => app_handle.exit(0),
            MenuId::Settings => todo!("Settings menu item clicked (not implemented)"),
        },
        Err(_) => {
            log::warn!("Unknown menu ID: {}", event.id().as_ref());
        }
    }
}

/// Handles the autostart menu item toggle event.
fn toggle_autostart(app_handle: &AppHandle, menu: &Menu<Wry>) -> AppResult<()> {
    let is_autostart_menu_item_checked = menu
        .get(&MenuId::AutoStart.to_string())
        .and_then(|menu_item| menu_item.as_check_menuitem().cloned())
        .context("Failed to retrieve the autostart menu item.")?
        .is_checked()?;
    app_handle.emit(
        &CustomEvent::Autostart.to_string(),
        json!(is_autostart_menu_item_checked),
    )?;

    Ok(())
}

/// Handles the authentication state change event.
fn on_authentication_state_change(owned_tray_menu: &Menu<Wry>, event: Event) -> AppResult<()> {
    for menu_id in [MenuId::QuickAdd, MenuId::LogOut] {
        owned_tray_menu
            .get(&menu_id.to_string())
            .and_then(|menu_item| menu_item.as_menuitem().cloned())
            .ok_or(format_err!(
                "Failed to retrieve the {:?} menu item.",
                menu_id
            ))?
            .set_enabled(serde_json::from_str::<bool>(event.payload())?)?;
    }

    Ok(())
}
