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
    /// Network address (XorName).
    Network([u8; 32]),
}

/// Trait for media playback operations.
#[async_trait]
pub trait MediaService: Send + Sync {
    // Playback control

    /// Load a media source.
    async fn load(&self, source: MediaSource) -> Result<()>;

    /// Play the current media.
    fn play(&self) -> Result<()>;

    /// Pause playback.
    fn pause(&self) -> Result<()>;

    /// Stop playback.
    fn stop(&self) -> Result<()>;

    /// Seek to position.
    fn seek(&self, position: Duration) -> Result<()>;

    /// Set volume (0.0 - 1.0).
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
    async fn queue_add(&self, source: MediaSource) -> Result<()>;

    /// Remove from queue.
    fn queue_remove(&self, index: usize) -> Result<()>;

    /// Clear queue.
    fn queue_clear(&self);

    /// Get queue.
    fn queue(&self) -> Vec<TrackInfo>;

    /// Skip to next track.
    async fn next(&self) -> Result<()>;

    /// Go to previous track.
    async fn previous(&self) -> Result<()>;

    /// Shuffle queue.
    fn shuffle(&self);

    // Library management

    /// Scan a folder for media files.
    async fn scan_folder(&self, path: &Path) -> Result<Vec<TrackInfo>>;

    /// Get library tracks.
    fn library(&self) -> Vec<TrackInfo>;

    /// Add track to library.
    async fn add_to_library(&self, source: MediaSource) -> Result<TrackInfo>;

    /// Remove track from library.
    async fn remove_from_library(&self, track_id: &[u8; 32]) -> Result<()>;

    // Playlist management

    /// Create a playlist.
    async fn create_playlist(&self, name: &str) -> Result<[u8; 16]>;

    /// Delete a playlist.
    async fn delete_playlist(&self, playlist_id: &[u8; 16]) -> Result<()>;

    /// Rename a playlist.
    async fn rename_playlist(&self, playlist_id: &[u8; 16], new_name: &str) -> Result<()>;

    /// Get all playlists.
    fn list_playlists(&self) -> Vec<PlaylistInfo>;

    /// Get playlist tracks.
    fn playlist_tracks(&self, playlist_id: &[u8; 16]) -> Result<Vec<TrackInfo>>;

    /// Add track to playlist.
    async fn add_to_playlist(&self, playlist_id: &[u8; 16], track_id: &[u8; 32]) -> Result<()>;

    /// Remove track from playlist.
    async fn remove_from_playlist(&self, playlist_id: &[u8; 16], index: usize) -> Result<()>;

    /// Sync playlist to network.
    async fn sync_playlist(&self, playlist_id: &[u8; 16]) -> Result<()>;

    /// Import playlist from network.
    async fn import_playlist(&self, address: &[u8; 32]) -> Result<PlaylistInfo>;
}
