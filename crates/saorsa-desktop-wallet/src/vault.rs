//! Encrypted vault storage.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Vault file header (unencrypted metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultHeader {
    /// Magic bytes: "SAORSAVLT"
    pub magic: [u8; 9],
    /// Format version.
    pub version: u32,
    /// Argon2id parameters.
    pub kdf_params: Argon2Params,
    /// Salt for KDF (32 bytes).
    pub kdf_salt: [u8; 32],
    /// Nonce for XChaCha20-Poly1305 (24 bytes).
    pub nonce: [u8; 24],
    /// HMAC of encrypted payload.
    pub hmac: [u8; 32],
    /// Last modification timestamp.
    pub modified_at: u64,
}

/// Argon2id parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argon2Params {
    /// Memory cost in KiB.
    pub m_cost: u32,
    /// Time cost (iterations).
    pub t_cost: u32,
    /// Parallelism.
    pub p_cost: u32,
    /// Output length.
    pub output_len: u32,
}

impl Default for Argon2Params {
    fn default() -> Self {
        Self {
            m_cost: 262_144, // 256 MiB
            t_cost: 4,
            p_cost: 4,
            output_len: 32,
        }
    }
}

/// Vault manager for encrypted storage.
pub struct Vault {
    path: PathBuf,
}

impl Vault {
    /// Create a new vault manager.
    #[must_use]
    pub const fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Check if vault file exists.
    #[must_use]
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get vault path.
    #[must_use]
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }
}
