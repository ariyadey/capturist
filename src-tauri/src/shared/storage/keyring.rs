//! This module provides a secure way to storage sensitive information using the system's keyring.
//!
//! # TODO: Platform Packaging Caveats on Linux
//! - Secret Service (DBus) must be available in the session.
//! - If packaged as Snap/Flatpak, appropriate permissions/portals may be required.

use crate::shared::error::AppResult;
use crate::shared::metadata::APP_ID;
use crate::shared::storage::key::StorageKey;

const KEYRING_SERVICE_NAME: &'static str = APP_ID;

/// Saves a value to the system keyring associated with a given `StorageKey`.
pub fn set(key: StorageKey, value: &str) -> AppResult<()> {
    keyring::Entry::new(KEYRING_SERVICE_NAME, &key.to_string())?.set_password(value)?;
    Ok(())
}

/// Retrieves a value from the system keyring associated with a given `StorageKey`.
pub fn find(key: StorageKey) -> AppResult<Option<String>> {
    match keyring::Entry::new(KEYRING_SERVICE_NAME, &key.to_string())?.get_password() {
        Ok(value) => Ok(Some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Deletes a value from the system keyring associated with a given `StorageKey`.
///
/// If the entry does not exist, this function will still return `Ok(())`.
pub fn delete(key: StorageKey) -> AppResult<()> {
    match keyring::Entry::new(KEYRING_SERVICE_NAME, &key.to_string())?.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
