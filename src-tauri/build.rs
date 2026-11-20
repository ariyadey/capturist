/// Public Todoist OAuth2 PKCE client ID (no client secret is used).
/// Can be overridden via `.env` or the `TODOIST_CLIENT_ID` environment variable.
const DEFAULT_TODOIST_CLIENT_ID: &str = "a8f74d9a1f1d424eb04189ab91c56a0e";

fn main() {
    set_environment_variables();
    tauri_build::build()
}

fn set_environment_variables() {
    dotenv::dotenv().ok();

    let client_id = std::env::var("TODOIST_CLIENT_ID")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| DEFAULT_TODOIST_CLIENT_ID.to_string());

    println!("cargo:rerun-if-env-changed=TODOIST_CLIENT_ID");
    println!("cargo:rustc-env=TODOIST_CLIENT_ID={client_id}");
}
