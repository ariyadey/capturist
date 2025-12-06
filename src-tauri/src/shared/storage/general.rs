use crate::shared::error::AppResult;
use crate::shared::storage::key::StorageKey;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

/// The path to the store file.
const STORE_PATH: &str = "capturist.json";

/// Saves a serializable value to the store.
pub fn set<T: Serialize>(key: StorageKey, value: T, app_handle: &AppHandle) -> AppResult<()> {
    let store = app_handle.store(STORE_PATH)?;
    let json = serde_json::to_value(value);
    store.set(key.to_string(), json?);
    store.save()?;
    Ok(())
}

/// Retrieves and deserialize a value from the store.
pub fn find<T: DeserializeOwned>(key: StorageKey, app_handle: &AppHandle) -> AppResult<Option<T>> {
    let value = app_handle
        .store(STORE_PATH)?
        .get(key.to_string())
        .and_then(|value| serde_json::from_value(value).ok());
    Ok(value)
}

/// Deletes a value from the store.
pub fn delete(key: StorageKey, app_handle: &AppHandle) -> AppResult<()> {
    let store = app_handle.store(STORE_PATH)?;
    store.delete(key.to_string());
    store.save()?;
    Ok(())
}
