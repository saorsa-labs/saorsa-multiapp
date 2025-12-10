//! Key storage and management.

/// Key store for managing wallet keys.
pub struct KeyStore;

impl KeyStore {
    /// Create a new key store.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for KeyStore {
    fn default() -> Self {
        Self::new()
    }
}
