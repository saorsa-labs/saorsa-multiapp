//! Two-tier chunk cache (memory + disk).

/// Chunk cache for network data.
pub struct ChunkCache;

impl ChunkCache {
    /// Create a new chunk cache.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ChunkCache {
    fn default() -> Self {
        Self::new()
    }
}
