use serde::Serialize;

/// Errors from the database/infrastructure layer.
/// App-specific error enums can wrap this via `#[from]`.
///
/// ## Module error pattern
///
/// Each sub-module should define its own error enum wrapping `CoreError`.
/// Copy this pattern for each new module:
///
/// ```rust,ignore
/// #[derive(Debug, thiserror::Error)]
/// #[non_exhaustive]
/// pub enum MyModuleError {
///     #[error(transparent)]
///     Core(#[from] orion_db::error::CoreError),
///
///     #[error("Validation: {0}")]
///     Validation(String),
/// }
///
/// impl From<MyModuleError> for orion_db::error::CommandError {
///     fn from(err: MyModuleError) -> Self {
///         match err {
///             MyModuleError::Core(e) => e.into(),
///             MyModuleError::Validation(msg) => Self::Validation(msg),
///         }
///     }
/// }
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CoreError {
    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Query failed: {0}")]
    QueryFailed(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Database error: {0}")]
    Other(String),
}

impl From<surrealdb::Error> for CoreError {
    fn from(err: surrealdb::Error) -> Self {
        CoreError::Other(err.to_string())
    }
}

/// The serialization boundary for Tauri commands.
/// Frontend receives `{ kind, message }` JSON.
/// Used by all Orion apps as the standard Tauri command error type.
#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[non_exhaustive]
pub enum CommandError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let (kind, message) = match self {
            Self::NotFound(m) => ("NOT_FOUND", m.as_str()),
            Self::Validation(m) => ("VALIDATION", m.as_str()),
            Self::Database(m) => ("DATABASE", m.as_str()),
            Self::Internal(m) => ("INTERNAL", m.as_str()),
        };
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("kind", kind)?;
        s.serialize_field("message", message)?;
        s.end()
    }
}

impl From<CoreError> for CommandError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::NotFound(m) => CommandError::NotFound(m),
            CoreError::QueryFailed(m) => CommandError::Database(m),
            CoreError::Connection(m) => CommandError::Database(m),
            CoreError::Deserialization(m) => CommandError::Internal(m),
            CoreError::Other(m) => CommandError::Internal(m),
        }
    }
}
