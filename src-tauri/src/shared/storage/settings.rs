use crate::shared::error::AppResult;
use crate::shared::storage::key::StorageKey;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

/// The path to the settings store file.
///
/// TODO: 31/10/2025 Specify the location of store exactly
const STORE_PATH: &'static str = "settings.json";

/// Saves a serializable value to the settings store.
pub fn set<T: Serialize>(key: StorageKey, value: T, app_handle: &AppHandle) -> AppResult<()> {
    let store = app_handle.store(STORE_PATH)?;
    let json = serde_json::to_value(value);
    store.set(key.to_string(), json?);
    store.save()?;
    Ok(())
}

/// Retrieves and deserialize a value from the settings store.
pub fn get<T: DeserializeOwned>(key: StorageKey, app_handle: &AppHandle) -> AppResult<Option<T>> {
    let value = app_handle
        .store(STORE_PATH)?
        .get(key.to_string())
        .and_then(|value| serde_json::from_value(value).ok());
    Ok(value)
}
