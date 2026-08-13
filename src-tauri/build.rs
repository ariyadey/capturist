fn main() {
    set_environment_variables();
    tauri_build::build()
}

/// Loads build-time environment variables from a `.env` file (if present) and
/// exposes the Todoist client ID to the main crate via `cargo:rustc-env`.
///
/// The client ID is required to authenticate with Todoist's OAuth server. It is
/// a public identifier, so it is safe to embed in the binary; supply it via
/// `.env`, CI secrets, or the Flathub manifest's `build-options.env`. PKCE
/// means no client secret is required.
fn set_environment_variables() {
    dotenv::dotenv().ok();

    let client_id = std::env::var("TODOIST_CLIENT_ID").expect("TODOIST_CLIENT_ID must be set");
    println!("cargo:rustc-env=TODOIST_CLIENT_ID={client_id}");
}
