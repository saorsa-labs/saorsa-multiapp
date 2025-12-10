//! Migration planning.

/// Migration planner for creating migration plans.
pub struct MigrationPlanner;

impl MigrationPlanner {
    /// Create a new migration planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for MigrationPlanner {
    fn default() -> Self {
        Self::new()
    }
}
