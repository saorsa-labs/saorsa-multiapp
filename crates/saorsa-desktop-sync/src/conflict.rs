//! Conflict detection and resolution.

/// Conflict resolver with last-write-wins strategy.
pub struct ConflictResolver;

impl ConflictResolver {
    /// Create a new conflict resolver.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}
