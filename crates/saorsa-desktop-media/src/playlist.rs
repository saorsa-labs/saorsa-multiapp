//! Playlist management.

/// Playlist manager for creating and syncing playlists.
pub struct PlaylistManager;

impl PlaylistManager {
    /// Create a new playlist manager.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for PlaylistManager {
    fn default() -> Self {
        Self::new()
    }
}
