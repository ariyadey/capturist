//! This module provides a secure way to storage sensitive information using the system's keyring.

use crate::shared::error::AppResult;
use crate::shared::metadata::APP_ID;
use crate::shared::storage;
use crate::shared::storage::key::StorageKey;
use tauri::AppHandle;

const KEYRING_SERVICE_NAME: &'static str = APP_ID;

/// Saves a value to the system keyring associated with a given `StorageKey`.
pub fn set(key: StorageKey, value: &str, app_handle: &AppHandle) -> AppResult<()> {
    get_provider(&key)
        .and_then(|provider| provider.set_password(value))
        .inspect_err(|e| log::error!("{e:?}\nUsing insecure storage as fallback."))
        .or(storage::general::set(key, value, app_handle))
}

/// Retrieves a value from the system keyring associated with a given `StorageKey`.
pub fn find(key: StorageKey, app_handle: &AppHandle) -> AppResult<Option<String>> {
    match get_provider(&key).and_then(|provider| provider.get_password()) {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => {
            log::error!("{e:?}\nUsing insecure storage as fallback.");
            storage::general::find(key, app_handle)
        }
    }
}

/// Deletes a value from the system keyring associated with a given `StorageKey`.
///
/// If the entry does not exist, this function will still return `Ok(())`.
pub fn delete(key: StorageKey, app_handle: &AppHandle) -> AppResult<()> {
    match get_provider(&key)?.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => {
            log::error!("{e:?}\nUsing insecure storage as fallback.");
            storage::general::delete(key, app_handle)
        }
    }
}

/// Gets a provider for the given key.
/// In production, this is a `keyring::Entry`.
/// In tests, this can be a mock provider.
fn get_provider(key: &StorageKey) -> Result<keyring::Entry, keyring::Error> {
    keyring::Entry::new(KEYRING_SERVICE_NAME, &key.to_string())
}
