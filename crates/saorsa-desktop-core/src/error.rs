//! Error types for the application.

use thiserror::Error;

/// Result type alias using our Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Application-wide error types.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Network error.
    #[error("Network error: {0}")]
    Network(String),

    /// Cryptographic error.
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Storage/database error.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// File system error.
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Wallet error.
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Sync error.
    #[error("Sync error: {0}")]
    Sync(String),

    /// Media error.
    #[error("Media error: {0}")]
    Media(String),

    /// Migration error.
    #[error("Migration error: {0}")]
    Migration(String),

    /// Internal error (should not happen).
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileSystem(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialization(e.to_string())
    }
}
