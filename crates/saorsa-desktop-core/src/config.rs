//! Application configuration.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Main application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Node configuration.
    pub node: NodeConfig,
    /// Wallet configuration.
    pub wallet: WalletConfig,
    /// Sync configuration.
    pub sync: SyncConfig,
    /// Media configuration.
    pub media: MediaConfig,
    /// UI configuration.
    pub ui: UiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            node: NodeConfig::default(),
            wallet: WalletConfig::default(),
            sync: SyncConfig::default(),
            media: MediaConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

/// Node connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Whether to run embedded node or connect to external.
    pub embedded: bool,
    /// Port for embedded node (0 = auto).
    pub port: u16,
    /// Bootstrap peers for network connection.
    pub bootstrap_peers: Vec<String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            embedded: true,
            port: 0,
            bootstrap_peers: Vec::new(),
        }
    }
}

/// Wallet configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Auto-lock timeout in seconds.
    pub auto_lock_seconds: u64,
    /// Path to vault file (None = default location).
    pub vault_path: Option<PathBuf>,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            auto_lock_seconds: 15 * 60, // 15 minutes
            vault_path: None,
        }
    }
}

impl WalletConfig {
    /// Get auto-lock timeout as Duration.
    #[must_use]
    pub fn auto_lock_timeout(&self) -> Duration {
        Duration::from_secs(self.auto_lock_seconds)
    }
}

/// Sync configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Maximum concurrent uploads.
    pub max_concurrent_uploads: usize,
    /// Maximum concurrent downloads.
    pub max_concurrent_downloads: usize,
    /// Sync interval in seconds (how often to check for changes).
    pub sync_interval_seconds: u64,
    /// Exclude patterns (glob patterns to ignore).
    pub exclude_patterns: Vec<String>,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            max_concurrent_uploads: 8,
            max_concurrent_downloads: 16,
            sync_interval_seconds: 60,
            exclude_patterns: vec![
                "*.tmp".to_string(),
                ".DS_Store".to_string(),
                "Thumbs.db".to_string(),
                ".git/**".to_string(),
            ],
        }
    }
}

/// Media playback configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    /// Minimum buffer size before playback starts (bytes).
    pub min_buffer_bytes: usize,
    /// Target buffer size during playback (bytes).
    pub target_buffer_bytes: usize,
    /// Number of chunks to prefetch ahead.
    pub prefetch_chunks: usize,
    /// Default volume (0.0 - 1.0).
    pub default_volume: f32,
}

impl Default for MediaConfig {
    fn default() -> Self {
        Self {
            min_buffer_bytes: 256 * 1024,       // 256KB
            target_buffer_bytes: 4 * 1024 * 1024, // 4MB
            prefetch_chunks: 8,
            default_volume: 0.8,
        }
    }
}

/// UI configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme (light, dark, system).
    pub theme: String,
    /// Window width.
    pub window_width: u32,
    /// Window height.
    pub window_height: u32,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            window_width: 1280,
            window_height: 800,
        }
    }
}
