/// The application's name, read from `Cargo.toml`.
pub const APP_ID: &str = env!("CARGO_PKG_NAME");

/// The application's title.
///
/// This is used in the window title and other places where a human-readable name is needed.
pub const APP_TITLE: &str = "Capturist";
