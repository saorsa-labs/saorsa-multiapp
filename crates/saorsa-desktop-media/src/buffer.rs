//! Streaming buffer for network playback.

/// Stream buffer for media playback.
pub struct StreamBuffer;

impl StreamBuffer {
    /// Create a new stream buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for StreamBuffer {
    fn default() -> Self {
        Self::new()
    }
}
