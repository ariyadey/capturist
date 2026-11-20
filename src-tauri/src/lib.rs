use crate::desktop::{cli, update, window};
use crate::ipc::deeplink::DeepLinkHost;
use crate::shared::error::AppResult;
use crate::shared::metadata::APP_ID;
use crate::shared::storage::key::StorageKey;
use crate::shared::{environment, state, storage};
use desktop::{autostart, tray};
use ipc::deeplink;
use shared::state::AppState;
use std::ops::Not;
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
        // Registers D-Bus name `<identifier>.SingleInstance` on Linux (matches snapcraft.yaml).
        .plugin(tauri_plugin_single_instance::init(
            |app_handle, argv, cwd| {
                // zbus invokes this on a background thread; GTK/WebKit window operations must run on main.
                let owned_handle = app_handle.clone();
                if let Err(e) = app_handle.run_on_main_thread(move || {
                    let _ = on_another_instance_trial(&owned_handle, argv, cwd)
                        .inspect_err(|e| log::error!("{e:?}"));
                }) {
                    log::error!(
                        "Failed to dispatch single-instance callback to main thread: {e:?}"
                    );
                }
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
                if environment::is_running_as_snap().not()
                    && environment::is_running_as_flatpak().not()
                {
                    autostart::set_up_autostart(app_handle)?;
                }
                if environment::is_running_as_appimage() {
                    update::set_up_updater(app_handle);
                }
                tray::set_up_tray_menu(app_handle)?;
            }
            deeplink::set_up_deep_link_handling(app_handle)?;
            window::set_up_current_window_synchronization(app_handle);
            let minimize = app_handle
                .cli()
                .matches()
                .inspect_err(|e| log::warn!("Failed to parse startup CLI args: {e:?}"))
                .ok()
                .and_then(|matches| {
                    matches
                        .args
                        .get(&cli::Argument::Minimize.to_string())
                        .and_then(|arg| arg.value.as_bool())
                })
                .unwrap_or(false);
            show_initial_window(app_handle, minimize)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::is_debug_mode,
            ipc::commands::is_os_linux,
            ipc::commands::is_wayland_session,
            ipc::commands::is_running_as_appimage,
            ipc::commands::is_running_as_flatpak,
            ipc::commands::start_authentication,
            ipc::commands::get_todoist_access_token,
            ipc::commands::refresh_todoist_access_token,
            ipc::commands::get_global_shortcut,
            ipc::commands::get_quick_add_command,
            ipc::commands::send_notification,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application.");
}

#[derive(Debug, Eq, PartialEq)]
enum SecondaryInstanceAction<'a> {
    Ignore,
    /// `tauri-plugin-deep-link` only auto-emits `on_open_url` when `argv.len() == 2`.
    HandleOauthFallback(&'a str),
    ShowWindow,
}

fn classify_secondary_instance_args(argv: &[String]) -> SecondaryInstanceAction<'_> {
    let oauth_prefix = format!("{APP_ID}://{}", DeepLinkHost::Oauth);
    if let Some(raw_url) = argv
        .iter()
        .skip(1)
        .find(|arg| arg.starts_with(&oauth_prefix))
    {
        return if argv.len() == 2 {
            SecondaryInstanceAction::Ignore
        } else {
            SecondaryInstanceAction::HandleOauthFallback(raw_url)
        };
    }

    if argv.iter().skip(1).any(|arg| arg == cli::MINIMIZE_ARG) {
        return SecondaryInstanceAction::Ignore;
    }

    SecondaryInstanceAction::ShowWindow
}

/// Handles the event when another instance of the application tries to start.
///
/// Runs on the main thread. Brings the existing instance's window to the foreground unless
/// the new instance is an OAuth deep link or requests minimization.
fn on_another_instance_trial(
    app_handle: &AppHandle,
    argv: Vec<String>,
    cwd: String,
) -> AppResult<()> {
    log::info!("Another instance tried to start with args: {argv:#?} and cwd: {cwd:#?}.");

    match classify_secondary_instance_args(&argv) {
        SecondaryInstanceAction::Ignore => Ok(()),
        SecondaryInstanceAction::HandleOauthFallback(raw_url) => {
            log::warn!(
                "OAuth deep link received with {} args; forwarding directly since deep-link plugin skips multi-arg invocations.",
                argv.len()
            );
            let url = tauri::Url::parse(raw_url)?;
            deeplink::handle_url(app_handle, &url);
            Ok(())
        }
        SecondaryInstanceAction::ShowWindow => show_initial_window(app_handle, false),
    }
}

/// Shows the initial window based on whether the user is authenticated or not.
fn show_initial_window(app_handle: &AppHandle, minimize: bool) -> AppResult<()> {
    log::info!("Showing the initial window based on whether the user is authenticated or not.");

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

#[cfg(test)]
mod tests {
    use super::{classify_secondary_instance_args, SecondaryInstanceAction};

    #[test]
    fn normal_secondary_launch_shows_window() {
        let argv = vec!["capturist".into()];
        assert_eq!(
            classify_secondary_instance_args(&argv),
            SecondaryInstanceAction::ShowWindow
        );
    }

    #[test]
    fn minimize_flag_is_ignored() {
        let argv = vec!["capturist".into(), "--minimize".into()];
        assert_eq!(
            classify_secondary_instance_args(&argv),
            SecondaryInstanceAction::Ignore
        );
    }

    #[test]
    fn two_arg_oauth_is_left_to_deep_link_plugin() {
        let argv = vec!["capturist".into(), "capturist://oauth?code=abc".into()];
        assert_eq!(
            classify_secondary_instance_args(&argv),
            SecondaryInstanceAction::Ignore
        );
    }

    #[test]
    fn multi_arg_oauth_uses_direct_fallback() {
        let argv = vec![
            "capturist".into(),
            "--quick-add".into(),
            "capturist://oauth?code=abc".into(),
        ];
        assert_eq!(
            classify_secondary_instance_args(&argv),
            SecondaryInstanceAction::HandleOauthFallback("capturist://oauth?code=abc")
        );
    }
}
