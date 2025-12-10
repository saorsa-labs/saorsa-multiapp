//! Migration execution.

/// Migration executor for running migrations.
pub struct MigrationExecutor;

impl MigrationExecutor {
    /// Create a new migration executor.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for MigrationExecutor {
    fn default() -> Self {
        Self::new()
    }
}
