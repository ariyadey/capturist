use serde::Serialize;

/// A type alias for `Result` that uses `AppError` as the error type.
///
/// This is used for internal functions where the error does not need to be serialized.
pub type AppResult<T> = anyhow::Result<T>;

pub type AppSerializableResult<T> = Result<T, AppSerializableError>;

/// A serializable error type for use in Tauri commands.
///
/// `anyhow::Error` is not `Serialize`, so we convert it to this simple string-based
/// error at the boundary of our application (in the `invoke_handler`).
#[derive(Debug, Serialize)]
pub struct AppSerializableError {
    pub message: String,
}

impl From<anyhow::Error> for AppSerializableError {
    fn from(error: anyhow::Error) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}
