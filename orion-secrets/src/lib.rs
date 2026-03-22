//! Shared OS keyring access for Orion apps.
//!
//! Wraps the `keyring` crate with consistent error handling, whitespace trimming,
//! and `NoEntry` graceful handling. Each app provides its own `service` name
//! (e.g., `"com.orion.coding"`, `"com.orion.life"`).
//!
//! # Platform features
//! This crate enables `apple-native` for macOS Keychain. Without it, keyring v3
//! silently uses an in-memory mock that loses all secrets on restart.

/// Errors from keyring operations.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SecretError {
    #[error("Keyring entry error for key '{key}': {source}")]
    Entry {
        key: String,
        source: keyring::Error,
    },

    #[error("Failed to store secret for key '{key}': {source}")]
    Set {
        key: String,
        source: keyring::Error,
    },

    #[error("Failed to read secret for key '{key}': {source}")]
    Get {
        key: String,
        source: keyring::Error,
    },

    #[error("Failed to delete secret for key '{key}': {source}")]
    Delete {
        key: String,
        source: keyring::Error,
    },
}

/// Store a secret in the OS keyring. Trims whitespace before saving.
pub fn set_secret(service: &str, key: &str, value: &str) -> Result<(), SecretError> {
    let entry = keyring::Entry::new(service, key)
        .map_err(|e| SecretError::Entry { key: key.to_string(), source: e })?;
    let trimmed = value.trim();
    entry
        .set_password(trimmed)
        .map_err(|e| SecretError::Set { key: key.to_string(), source: e })?;
    tracing::info!("Stored secret in keyring: {key}");
    Ok(())
}

/// Retrieve a secret from the OS keyring. Returns `None` if not found.
pub fn get_secret(service: &str, key: &str) -> Result<Option<String>, SecretError> {
    let entry = keyring::Entry::new(service, key)
        .map_err(|e| SecretError::Entry { key: key.to_string(), source: e })?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(SecretError::Get { key: key.to_string(), source: e }),
    }
}

/// Delete a secret from the OS keyring. No-op if the secret doesn't exist.
pub fn delete_secret(service: &str, key: &str) -> Result<(), SecretError> {
    let entry = keyring::Entry::new(service, key)
        .map_err(|e| SecretError::Entry { key: key.to_string(), source: e })?;
    match entry.delete_credential() {
        Ok(()) => {
            tracing::info!("Deleted secret from keyring: {key}");
            Ok(())
        }
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(SecretError::Delete { key: key.to_string(), source: e }),
    }
}
