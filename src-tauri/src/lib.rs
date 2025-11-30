use crate::desktop::{cli, update, window};
use crate::ipc::deeplink::DeepLinkHost;
use crate::shared::error::AppResult;
use crate::shared::metadata::APP_ID;
use crate::shared::storage::key::StorageKey;
use crate::shared::{environment, state, storage};
use desktop::{autostart, tray};
use ipc::deeplink;
use shared::state::AppState;
use tauri::{AppHandle, Manager};
use tauri_plugin_cli::CliExt;
use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};

mod desktop;
mod external;
mod ipc;
mod shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(
            |app_handle, argv, cwd| {
                let _ = on_another_instance_trial(app_handle, argv, cwd)
                    .inspect_err(|e| log::error!("{e:?}"));
            },
        ))
        .plugin(
            tauri_plugin_log::Builder::default()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .level(log::LevelFilter::Debug)
                .with_colors(
                    ColoredLevelConfig::new()
                        .error(Color::Red)
                        .warn(Color::Yellow)
                        .info(Color::Green)
                        .debug(Color::Cyan)
                        .trace(Color::Blue),
                )
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![&cli::MINIMIZE_ARG]),
        ))
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState::default())
        .setup(|app| {
            let app_handle = &app.handle();

            *app_handle.state::<AppState>().authenticated.lock().unwrap() =
                storage::secure::find(StorageKey::TodoistToken, app_handle)?.is_some();

            state::set_up_state_synchronization(app_handle);
            #[cfg(desktop)]
            {
                if !environment::is_running_as_snap() {
                    autostart::set_up_autostart(app_handle)?;
                }
                if environment::is_running_as_appimage() {
                    update::set_up_updater(app_handle);
                }
                tray::set_up_tray_menu(app_handle)?;
            }
            deeplink::set_up_deep_link_handling(app_handle);
            window::set_up_current_window_synchronization(app_handle);
            show_initial_window(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::is_debug_mode,
            ipc::commands::is_os_linux,
            ipc::commands::is_wayland_session,
            ipc::commands::is_running_as_appimage,
            ipc::commands::start_authentication,
            ipc::commands::get_todoist_access_token,
            ipc::commands::get_global_shortcut,
            ipc::commands::send_notification,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application.");
}

/// Handles the event when another instance of the application tries to start.
///
/// This function checks the command-line arguments of the new instance.
/// If the new instance is an OAuth deep link or requests minimization, it does nothing.
/// Otherwise, it brings the existing instance's window to the foreground.
fn on_another_instance_trial(
    app_handle: &AppHandle,
    argv: Vec<String>,
    cwd: String,
) -> AppResult<()> {
    log::info!("Another instance tried to start with args: {argv:#?} and cwd: {cwd:#?}.");

    let oauth_url = format!("{APP_ID}://{}", DeepLinkHost::OAUTH);
    let is_oauth_deep_link = argv.iter().any(|arg| arg.starts_with(&oauth_url));
    let should_minimize = argv.contains(&format!("--{}", cli::Argument::Minimize));
    if is_oauth_deep_link || should_minimize {
        return Ok(());
    }
    show_initial_window(app_handle)?;

    Ok(())
}

/// Shows the initial window based on whether the user is authenticated or not.
fn show_initial_window(app_handle: &AppHandle) -> AppResult<()> {
    log::info!("Showing the initial window based on whether the user is authenticated or not.");

    let minimize = app_handle
        .cli()
        .matches()?
        .args
        .get(&cli::Argument::Minimize.to_string())
        .map(|arg| arg.value.to_owned())
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let authenticated = app_handle
        .state::<AppState>()
        .authenticated
        .lock()
        .unwrap()
        .to_owned();

    if authenticated {
        window::init_quick_add_dialog(app_handle, minimize)?;
    } else {
        window::init_authentication_window(app_handle, minimize)?;
    }

    Ok(())
}
