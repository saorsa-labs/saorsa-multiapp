//! Migration verification.

/// Migration verifier for checking migrated data.
pub struct MigrationVerifier;

impl MigrationVerifier {
    /// Create a new migration verifier.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for MigrationVerifier {
    fn default() -> Self {
        Self::new()
    }
}
