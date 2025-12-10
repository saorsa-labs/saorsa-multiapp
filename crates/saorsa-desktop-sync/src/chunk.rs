//! Chunk management for large files.

/// Maximum chunk size (4MB - overhead).
pub const MAX_CHUNK_SIZE: usize = 4 * 1024 * 1024 - 256;

/// Chunk manager for splitting and reassembling files.
pub struct ChunkManager;

impl ChunkManager {
    /// Create a new chunk manager.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new()
    }
}
