//! Application state management.

use crate::config::AppConfig;
use crate::events::SharedEventBus;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Root application state.
#[derive(Clone)]
pub struct AppState {
    /// Node connection state.
    pub node: Arc<RwLock<NodeState>>,
    /// Wallet state.
    pub wallet: Arc<RwLock<WalletState>>,
    /// Sync state.
    pub sync: Arc<RwLock<SyncState>>,
    /// Media player state.
    pub media: Arc<RwLock<MediaState>>,
    /// Migration state.
    pub migration: Arc<RwLock<MigrationState>>,
    /// Application configuration.
    pub config: Arc<RwLock<AppConfig>>,
    /// Event bus.
    pub events: SharedEventBus,
}

impl AppState {
    /// Create new application state with default configuration.
    #[must_use]
    pub fn new(events: SharedEventBus) -> Self {
        Self {
            node: Arc::new(RwLock::new(NodeState::default())),
            wallet: Arc::new(RwLock::new(WalletState::default())),
            sync: Arc::new(RwLock::new(SyncState::default())),
            media: Arc::new(RwLock::new(MediaState::default())),
            migration: Arc::new(RwLock::new(MigrationState::default())),
            config: Arc::new(RwLock::new(AppConfig::default())),
            events,
        }
    }
}

/// Node connection state.
#[derive(Debug, Clone, Default)]
pub struct NodeState {
    /// Current status.
    pub status: NodeStatus,
    /// Number of connected peers.
    pub peer_count: usize,
    /// Network statistics.
    pub stats: NetworkStats,
    /// Recent events (last 100).
    pub recent_events: VecDeque<String>,
}

/// Node connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeStatus {
    /// Not connected.
    #[default]
    Disconnected,
    /// Connecting to network.
    Connecting,
    /// Connected and operational.
    Connected,
    /// Synchronizing with network.
    Syncing {
        /// Sync progress (0.0 - 1.0).
        progress: u8, // 0-100
    },
    /// Error state.
    Error,
}

/// Network statistics.
#[derive(Debug, Clone, Default)]
pub struct NetworkStats {
    /// Bytes uploaded.
    pub bytes_uploaded: u64,
    /// Bytes downloaded.
    pub bytes_downloaded: u64,
    /// Data stored on network.
    pub chunks_stored: u64,
    /// Data retrieved from network.
    pub chunks_retrieved: u64,
}

/// Wallet state.
#[derive(Debug, Clone, Default)]
pub struct WalletState {
    /// Current session state.
    pub session: SessionState,
    /// List of available wallets (metadata only).
    pub wallets: Vec<WalletInfo>,
    /// Active wallet ID.
    pub active_wallet: Option<[u8; 16]>,
    /// Wallet balances.
    pub balances: Vec<WalletBalance>,
    /// Loading indicator.
    pub is_loading: bool,
    /// Error message if any.
    pub error: Option<String>,
}

/// Wallet session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SessionState {
    /// No vault exists.
    Uninitialized,
    /// Vault exists but locked.
    #[default]
    Locked,
    /// Unlocking in progress.
    Unlocking,
    /// Unlocked and ready.
    Unlocked,
    /// Locking in progress.
    Locking,
}

/// Wallet info (non-sensitive metadata).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Unique wallet ID.
    pub id: [u8; 16],
    /// Human-readable name.
    pub name: String,
    /// Saorsa network address.
    pub saorsa_address: [u8; 32],
    /// EVM address (if applicable).
    pub evm_address: Option<[u8; 20]>,
    /// Creation timestamp.
    pub created_at: i64,
    /// Last used timestamp.
    pub last_used: i64,
}

/// Wallet balance info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Wallet ID.
    pub wallet_id: [u8; 16],
    /// Native token balance (in smallest unit).
    pub native_balance: String,
    /// Token balances.
    pub tokens: Vec<TokenBalance>,
}

/// Token balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    /// Token symbol.
    pub symbol: String,
    /// Token balance.
    pub balance: String,
    /// Token contract address.
    pub contract: [u8; 20],
}

/// Sync state.
#[derive(Debug, Clone, Default)]
pub struct SyncState {
    /// Synced folders.
    pub folders: Vec<SyncFolder>,
    /// Whether sync is active.
    pub is_syncing: bool,
    /// Whether sync is paused.
    pub is_paused: bool,
    /// Current sync operation.
    pub current_operation: Option<SyncOperation>,
    /// Overall stats.
    pub stats: SyncStats,
    /// Pending conflicts.
    pub conflicts: Vec<SyncConflict>,
}

/// Synced folder info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFolder {
    /// Unique folder ID.
    pub id: [u8; 16],
    /// Local path.
    pub local_path: String,
    /// Folder name (display).
    pub name: String,
    /// Last sync time.
    pub last_sync: i64,
    /// Sync status.
    pub status: SyncFolderStatus,
}

/// Sync folder status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SyncFolderStatus {
    /// Up to date.
    #[default]
    Synced,
    /// Syncing in progress.
    Syncing,
    /// Pending changes.
    Pending,
    /// Has conflicts.
    Conflict,
    /// Error state.
    Error,
}

/// Current sync operation.
#[derive(Debug, Clone)]
pub struct SyncOperation {
    /// File being synced.
    pub file_path: String,
    /// Operation type.
    pub operation: SyncOperationType,
    /// Progress (0.0 - 1.0).
    pub progress: f32,
}

/// Sync operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncOperationType {
    /// Uploading file.
    Upload,
    /// Downloading file.
    Download,
    /// Scanning changes.
    Scan,
}

/// Sync statistics.
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    /// Total files synced.
    pub files_synced: u64,
    /// Total bytes synced.
    pub bytes_synced: u64,
    /// Files pending.
    pub files_pending: u64,
    /// Last sync completed.
    pub last_sync: Option<i64>,
}

/// Sync conflict info.
#[derive(Debug, Clone)]
pub struct SyncConflict {
    /// File path.
    pub path: String,
    /// Local modification time.
    pub local_modified: i64,
    /// Remote modification time.
    pub remote_modified: i64,
    /// Local file size.
    pub local_size: u64,
    /// Remote file size.
    pub remote_size: u64,
}

/// Media player state.
#[derive(Debug, Clone, Default)]
pub struct MediaState {
    /// Current playback state.
    pub playback: PlaybackState,
    /// Currently playing track.
    pub current_track: Option<TrackInfo>,
    /// Current position in seconds.
    pub position_secs: f64,
    /// Total duration in seconds.
    pub duration_secs: f64,
    /// Volume (0.0 - 1.0).
    pub volume: f32,
    /// Play queue.
    pub queue: Vec<TrackInfo>,
    /// Queue index.
    pub queue_index: usize,
    /// Playlists.
    pub playlists: Vec<PlaylistInfo>,
    /// Buffer health (0.0 - 1.0).
    pub buffer_health: f32,
}

/// Playback state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackState {
    /// Stopped.
    #[default]
    Stopped,
    /// Playing.
    Playing,
    /// Paused.
    Paused,
    /// Buffering.
    Buffering,
}

/// Track info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
    /// Track ID (network address).
    pub id: [u8; 32],
    /// Track title.
    pub title: String,
    /// Artist name.
    pub artist: Option<String>,
    /// Album name.
    pub album: Option<String>,
    /// Duration in seconds.
    pub duration_secs: f64,
    /// File size in bytes.
    pub size_bytes: u64,
    /// Track type (audio/video).
    pub track_type: TrackType,
}

/// Track type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TrackType {
    /// Audio track.
    #[default]
    Audio,
    /// Video track.
    Video,
}

/// Playlist info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistInfo {
    /// Playlist ID.
    pub id: [u8; 16],
    /// Playlist name.
    pub name: String,
    /// Number of tracks.
    pub track_count: usize,
    /// Total duration in seconds.
    pub total_duration_secs: f64,
    /// Last modified.
    pub modified_at: i64,
    /// Whether synced to network.
    pub is_synced: bool,
}

/// Migration state.
#[derive(Debug, Clone, Default)]
pub struct MigrationState {
    /// Migration status.
    pub status: MigrationStatus,
    /// Total items discovered.
    pub total_items: u64,
    /// Items migrated.
    pub migrated_items: u64,
    /// Items failed.
    pub failed_items: u64,
    /// Current item being migrated.
    pub current_item: Option<String>,
    /// Discovered data sources.
    pub sources: Vec<MigrationSource>,
}

/// Migration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MigrationStatus {
    /// Not started.
    #[default]
    Idle,
    /// Scanning for data.
    Scanning,
    /// Ready to migrate.
    Ready,
    /// Migration in progress.
    InProgress,
    /// Migration completed.
    Completed,
    /// Migration failed.
    Failed,
}

/// Migration source info.
#[derive(Debug, Clone)]
pub struct MigrationSource {
    /// Source type.
    pub source_type: MigrationSourceType,
    /// Source path or identifier.
    pub identifier: String,
    /// Items found.
    pub item_count: u64,
    /// Total size in bytes.
    pub total_bytes: u64,
}

/// Migration source type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationSourceType {
    /// Local ant-node data.
    LocalAntNode,
    /// Autonomi network.
    AutonomiNetwork,
}
