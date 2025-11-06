use crate::desktop::{cli, window};
use crate::ipc::deeplink::OAUTH_URI;
use crate::shared::error::AppResult;
use crate::shared::storage::key::StorageKey;
use crate::shared::{state, storage};
use desktop::{autostart, shortcut, tray};
use ipc::deeplink;
use shared::state::AppState;
use std::collections::HashSet;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_cli::CliExt;

mod desktop;
mod external;
mod ipc;
mod shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: 10/09/2025 https://v2.tauri.app/plugin/single-instance/#usage-in-snap-and-flatpak
        .plugin(tauri_plugin_single_instance::init(
            |app_handle, argv, cwd| {
                let _ = on_another_instance_trial(app_handle, argv, cwd)
                    .inspect_err(|e| log::error!("{e:?}"));
            },
        ))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![&cli::MINIMIZE_ARG]),
        ))
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState {
            authenticated: Mutex::new(
                storage::keyring::find(StorageKey::TodoistToken)
                    .unwrap()
                    .is_some(),
            ),
            ..Default::default()
        })
        .setup(|app| {
            let app_handle = &app.handle();
            state::set_up_state_synchronization(app_handle);
            #[cfg(desktop)]
            {
                shortcut::set_up_global_shortcut(app_handle)?;
                autostart::set_up_autostart(app_handle)?;
                tray::set_up_tray_menu(app_handle)?;
            }
            deeplink::set_up_deep_link_handling(app.handle());
            window::set_up_current_window_synchronization(app_handle);
            show_initial_window(app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::commands::start_authentication,
            ipc::commands::get_todoist_access_token,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application.");
}

// TODO: 06/11/2025 Test it
// TODO: 06/11/2025 Add docs
fn on_another_instance_trial(
    app_handle: &AppHandle,
    argv: Vec<String>,
    cwd: String,
) -> AppResult<()> {
    log::info!("Another instance tried to start with args: {argv:?} and cwd: {cwd:?}.");
    let app_args = argv.into_iter().collect::<HashSet<String>>();
    let ignored_args = HashSet::from([
        OAUTH_URI.to_string(),
        format!("--{:?}", cli::Argument::Minimize),
    ]);
    if HashSet::is_disjoint(&app_args, &ignored_args) {
        show_initial_window(app_handle)?;
    }

    Ok(())
}

/// Shows the initial window based on whether the user is authenticated or not.
fn show_initial_window(app_handle: &AppHandle) -> AppResult<()> {
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
