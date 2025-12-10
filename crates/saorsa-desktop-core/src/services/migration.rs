//! Migration service trait for Autonomi data migration.

use crate::error::Result;
use crate::state::{MigrationSource, MigrationSourceType, MigrationStatus};
use async_trait::async_trait;
use std::path::Path;

/// Migration plan details.
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    /// Sources discovered.
    pub sources: Vec<MigrationSource>,
    /// Total items to migrate.
    pub total_items: u64,
    /// Total bytes to migrate.
    pub total_bytes: u64,
    /// Estimated cost (if payment required).
    pub estimated_cost: Option<String>,
    /// Items already on saorsa (no migration needed).
    pub already_migrated: u64,
}

/// Migration progress info.
#[derive(Debug, Clone)]
pub struct MigrationProgress {
    /// Current status.
    pub status: MigrationStatus,
    /// Items migrated successfully.
    pub migrated: u64,
    /// Items failed.
    pub failed: u64,
    /// Total items.
    pub total: u64,
    /// Current item being processed.
    pub current_item: Option<String>,
    /// Bytes transferred.
    pub bytes_transferred: u64,
}

/// Trait for migration operations.
#[async_trait]
pub trait MigrationService: Send + Sync {
    // Discovery

    /// Scan for local ant-node data.
    async fn scan_local(&self, path: Option<&Path>) -> Result<Vec<MigrationSource>>;

    /// Scan autonomi network for user data.
    async fn scan_network(&self, public_key: &[u8; 32]) -> Result<Vec<MigrationSource>>;

    /// Create migration plan from discovered sources.
    async fn create_plan(&self, sources: Vec<MigrationSource>) -> Result<MigrationPlan>;

    // Execution

    /// Start migration.
    async fn start(&self, plan: &MigrationPlan) -> Result<()>;

    /// Pause migration.
    fn pause(&self);

    /// Resume migration.
    async fn resume(&self) -> Result<()>;

    /// Cancel migration.
    fn cancel(&self);

    // Status

    /// Get current status.
    fn status(&self) -> MigrationStatus;

    /// Get detailed progress.
    fn progress(&self) -> MigrationProgress;

    /// Get migration history (past migrations).
    fn history(&self) -> Vec<MigrationHistoryEntry>;

    // Verification

    /// Verify migrated data matches source.
    async fn verify(&self, address: &[u8; 32]) -> Result<VerificationResult>;

    /// Verify all migrated data.
    async fn verify_all(&self) -> Result<Vec<VerificationResult>>;
}

/// Migration history entry.
#[derive(Debug, Clone)]
pub struct MigrationHistoryEntry {
    /// Timestamp.
    pub timestamp: i64,
    /// Source type.
    pub source_type: MigrationSourceType,
    /// Items migrated.
    pub items_migrated: u64,
    /// Items failed.
    pub items_failed: u64,
    /// Duration in seconds.
    pub duration_secs: u64,
}

/// Verification result.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Data address.
    pub address: [u8; 32],
    /// Whether verification passed.
    pub verified: bool,
    /// Error message if verification failed.
    pub error: Option<String>,
}
