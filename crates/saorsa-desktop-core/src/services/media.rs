//! Media service trait for playback operations.

use crate::error::Result;
use crate::state::{PlaybackState, PlaylistInfo, TrackInfo};
use async_trait::async_trait;
use std::path::Path;
use std::time::Duration;

/// Media source.
#[derive(Debug, Clone)]
pub enum MediaSource {
    /// Local file path.
    Local(String),
    /// Network address (`XorName`).
    Network([u8; 32]),
}

/// Trait for media playback operations.
#[async_trait]
pub trait MediaService: Send + Sync {
    // Playback control

    /// Load a media source.
    ///
    /// # Errors
    /// Returns an error if the source cannot be loaded.
    async fn load(&self, source: MediaSource) -> Result<()>;

    /// Play the current media.
    ///
    /// # Errors
    /// Returns an error if playback cannot start.
    fn play(&self) -> Result<()>;

    /// Pause playback.
    ///
    /// # Errors
    /// Returns an error if playback cannot be paused.
    fn pause(&self) -> Result<()>;

    /// Stop playback.
    ///
    /// # Errors
    /// Returns an error if playback cannot be stopped.
    fn stop(&self) -> Result<()>;

    /// Seek to position.
    ///
    /// # Errors
    /// Returns an error if seeking fails.
    fn seek(&self, position: Duration) -> Result<()>;

    /// Set volume (0.0 - 1.0).
    ///
    /// # Errors
    /// Returns an error if volume cannot be set.
    fn set_volume(&self, volume: f32) -> Result<()>;

    /// Get current volume.
    fn volume(&self) -> f32;

    /// Get current playback state.
    fn state(&self) -> PlaybackState;

    /// Get current position.
    fn position(&self) -> Option<Duration>;

    /// Get media duration.
    fn duration(&self) -> Option<Duration>;

    /// Get buffer health (0.0 - 1.0).
    fn buffer_health(&self) -> f32;

    // Queue management

    /// Add to queue.
    ///
    /// # Errors
    /// Returns an error if the source cannot be added.
    async fn queue_add(&self, source: MediaSource) -> Result<()>;

    /// Remove from queue.
    ///
    /// # Errors
    /// Returns an error if the index is out of bounds.
    fn queue_remove(&self, index: usize) -> Result<()>;

    /// Clear queue.
    fn queue_clear(&self);

    /// Get queue.
    fn queue(&self) -> Vec<TrackInfo>;

    /// Skip to next track.
    ///
    /// # Errors
    /// Returns an error if there is no next track.
    async fn next(&self) -> Result<()>;

    /// Go to previous track.
    ///
    /// # Errors
    /// Returns an error if there is no previous track.
    async fn previous(&self) -> Result<()>;

    /// Shuffle queue.
    fn shuffle(&self);

    // Library management

    /// Scan a folder for media files.
    ///
    /// # Errors
    /// Returns an error if the folder cannot be scanned.
    async fn scan_folder(&self, path: &Path) -> Result<Vec<TrackInfo>>;

    /// Get library tracks.
    fn library(&self) -> Vec<TrackInfo>;

    /// Add track to library.
    ///
    /// # Errors
    /// Returns an error if the track cannot be added.
    async fn add_to_library(&self, source: MediaSource) -> Result<TrackInfo>;

    /// Remove track from library.
    ///
    /// # Errors
    /// Returns an error if the track cannot be removed.
    async fn remove_from_library(&self, track_id: &[u8; 32]) -> Result<()>;

    // Playlist management

    /// Create a playlist.
    ///
    /// # Errors
    /// Returns an error if the playlist cannot be created.
    async fn create_playlist(&self, name: &str) -> Result<[u8; 16]>;

    /// Delete a playlist.
    ///
    /// # Errors
    /// Returns an error if the playlist cannot be deleted.
    async fn delete_playlist(&self, playlist_id: &[u8; 16]) -> Result<()>;

    /// Rename a playlist.
    ///
    /// # Errors
    /// Returns an error if the playlist cannot be renamed.
    async fn rename_playlist(&self, playlist_id: &[u8; 16], new_name: &str) -> Result<()>;

    /// Get all playlists.
    fn list_playlists(&self) -> Vec<PlaylistInfo>;

    /// Get playlist tracks.
    ///
    /// # Errors
    /// Returns an error if the playlist is not found.
    fn playlist_tracks(&self, playlist_id: &[u8; 16]) -> Result<Vec<TrackInfo>>;

    /// Add track to playlist.
    ///
    /// # Errors
    /// Returns an error if the track cannot be added.
    async fn add_to_playlist(&self, playlist_id: &[u8; 16], track_id: &[u8; 32]) -> Result<()>;

    /// Remove track from playlist.
    ///
    /// # Errors
    /// Returns an error if the track cannot be removed.
    async fn remove_from_playlist(&self, playlist_id: &[u8; 16], index: usize) -> Result<()>;

    /// Sync playlist to network.
    ///
    /// # Errors
    /// Returns an error if sync fails.
    async fn sync_playlist(&self, playlist_id: &[u8; 16]) -> Result<()>;

    /// Import playlist from network.
    ///
    /// # Errors
    /// Returns an error if import fails.
    async fn import_playlist(&self, address: &[u8; 32]) -> Result<PlaylistInfo>;
}
