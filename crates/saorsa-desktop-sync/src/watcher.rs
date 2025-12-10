//! File system watching.

/// File watcher for detecting changes.
pub struct FileWatcher;

impl FileWatcher {
    /// Create a new file watcher.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}
