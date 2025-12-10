//! Data discovery and scanning.

/// Data scanner for finding migratable data.
pub struct DataScanner;

impl DataScanner {
    /// Create a new data scanner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for DataScanner {
    fn default() -> Self {
        Self::new()
    }
}
