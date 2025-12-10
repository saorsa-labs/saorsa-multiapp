//! Sync service trait for file synchronization.

use crate::error::Result;
use crate::state::{SyncConflict, SyncFolder, SyncFolderStatus, SyncStats};
use async_trait::async_trait;
use std::path::Path;

/// Sync folder options.
#[derive(Debug, Clone)]
pub struct SyncFolderOptions {
    /// Sync mode.
    pub mode: SyncMode,
    /// Exclude patterns (globs).
    pub exclude_patterns: Vec<String>,
    /// Maximum file size to sync (None = no limit).
    pub max_file_size: Option<u64>,
}

impl Default for SyncFolderOptions {
    fn default() -> Self {
        Self {
            mode: SyncMode::Bidirectional,
            exclude_patterns: Vec::new(),
            max_file_size: None,
        }
    }
}

/// Sync mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyncMode {
    /// Upload and download.
    #[default]
    Bidirectional,
    /// Upload only.
    UploadOnly,
    /// Download only.
    DownloadOnly,
}

/// Conflict resolution choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Keep local version.
    KeepLocal,
    /// Keep remote version.
    KeepRemote,
    /// Keep both (rename local).
    KeepBoth,
}

/// Trait for file synchronization operations.
#[async_trait]
pub trait SyncService: Send + Sync {
    // Folder management

    /// Add a folder to sync.
    async fn add_folder(&self, path: &Path, options: SyncFolderOptions) -> Result<[u8; 16]>;

    /// Remove a folder from sync.
    async fn remove_folder(&self, folder_id: &[u8; 16]) -> Result<()>;

    /// Get all synced folders.
    fn list_folders(&self) -> Vec<SyncFolder>;

    /// Get folder status.
    fn folder_status(&self, folder_id: &[u8; 16]) -> Option<SyncFolderStatus>;

    /// Update folder options.
    async fn update_folder_options(
        &self,
        folder_id: &[u8; 16],
        options: SyncFolderOptions,
    ) -> Result<()>;

    // Sync operations

    /// Start sync engine.
    async fn start(&self) -> Result<()>;

    /// Stop sync engine.
    async fn stop(&self) -> Result<()>;

    /// Pause sync.
    fn pause(&self);

    /// Resume sync.
    fn resume(&self);

    /// Force sync a specific folder now.
    async fn sync_now(&self, folder_id: &[u8; 16]) -> Result<()>;

    /// Force sync all folders now.
    async fn sync_all_now(&self) -> Result<()>;

    // Conflict handling

    /// Get pending conflicts.
    fn get_conflicts(&self) -> Vec<SyncConflict>;

    /// Resolve a conflict.
    async fn resolve_conflict(&self, path: &str, resolution: ConflictResolution) -> Result<()>;

    /// Resolve all conflicts with same resolution.
    async fn resolve_all_conflicts(&self, resolution: ConflictResolution) -> Result<()>;

    // Statistics

    /// Get sync statistics.
    fn stats(&self) -> SyncStats;

    /// Get sync history (recent operations).
    fn history(&self, limit: usize) -> Vec<SyncHistoryEntry>;
}

/// Sync history entry.
#[derive(Debug, Clone)]
pub struct SyncHistoryEntry {
    /// File path.
    pub path: String,
    /// Operation type.
    pub operation: SyncHistoryOperation,
    /// Timestamp.
    pub timestamp: i64,
    /// Success or error message.
    pub result: std::result::Result<(), String>,
}

/// Sync history operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncHistoryOperation {
    /// Uploaded file.
    Upload,
    /// Downloaded file.
    Download,
    /// Deleted locally.
    DeleteLocal,
    /// Deleted remote.
    DeleteRemote,
    /// Conflict resolved.
    ConflictResolved,
}
