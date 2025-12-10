//! Media library management.

/// Media library for organizing tracks.
pub struct MediaLibrary;

impl MediaLibrary {
    /// Create a new media library.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for MediaLibrary {
    fn default() -> Self {
        Self::new()
    }
}
