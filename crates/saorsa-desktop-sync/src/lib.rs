//! File synchronization engine for Saorsa desktop application.
//!
//! This crate provides:
//! - File system watching
//! - Change detection and tracking
//! - Chunk management for large files
//! - Two-tier caching (memory + disk)
//! - Conflict resolution (last-write-wins with notification)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod cache;
pub mod chunk;
pub mod conflict;
pub mod engine;
pub mod index;
pub mod watcher;

pub use cache::ChunkCache;
pub use chunk::ChunkManager;
pub use conflict::ConflictResolver;
pub use engine::SyncEngine;
pub use index::SyncIndex;
pub use watcher::FileWatcher;
