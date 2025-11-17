fn main() {
    set_environment_variables();
    tauri_build::build()
}

/// Sets environment variables for the build process.
///
/// This function loads environment variables from a `.env` file (if present)
/// and then retrieves specific Todoist API credentials. These credentials
/// are then exposed to the main crate via `cargo:rustc-env` directives,
/// allowing them to be accessed at compile time using the `env!` macro.
fn set_environment_variables() {
    dotenv::dotenv().ok();

    let client_id = std::env::var("TODOIST_CLIENT_ID").expect("TODOIST_CLIENT_ID must be set");
    let client_secret =
        std::env::var("TODOIST_CLIENT_SECRET").expect("TODOIST_CLIENT_SECRET must be set");

    println!("cargo:rustc-env=TODOIST_CLIENT_ID={}", client_id);
    println!("cargo:rustc-env=TODOIST_CLIENT_SECRET={}", client_secret);
}
