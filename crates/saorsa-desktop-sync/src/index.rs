//! `SQLite` sync index.

/// Sync index for tracking file states.
pub struct SyncIndex;

impl SyncIndex {
    /// Create a new sync index.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for SyncIndex {
    fn default() -> Self {
        Self::new()
    }
}
