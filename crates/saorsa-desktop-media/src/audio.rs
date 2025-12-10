//! Audio playback using rodio.

/// Audio player for music playback.
pub struct AudioPlayer;

impl AudioPlayer {
    /// Create a new audio player.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
