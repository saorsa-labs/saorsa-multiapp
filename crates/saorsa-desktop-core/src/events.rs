//! Event bus for cross-module communication.

use std::sync::Arc;
use tokio::sync::broadcast;

/// Application-wide events.
#[derive(Debug, Clone)]
pub enum AppEvent {
    // Node events
    /// Node started successfully.
    NodeStarted,
    /// Node stopped.
    NodeStopped,
    /// Peer connected.
    PeerConnected {
        /// Peer identifier.
        peer_id: String,
    },
    /// Peer disconnected.
    PeerDisconnected {
        /// Peer identifier.
        peer_id: String,
    },

    // Wallet events
    /// Wallet unlocked.
    WalletUnlocked,
    /// Wallet locked.
    WalletLocked,
    /// Active wallet changed.
    WalletChanged {
        /// New active wallet ID.
        wallet_id: [u8; 16],
    },

    // Sync events
    /// File sync started.
    SyncStarted {
        /// Path being synced.
        path: String,
    },
    /// File sync completed.
    SyncCompleted {
        /// Path that was synced.
        path: String,
    },
    /// Sync conflict detected.
    SyncConflict {
        /// Path with conflict.
        path: String,
        /// Local modification time.
        local_modified: i64,
        /// Remote modification time.
        remote_modified: i64,
    },
    /// Sync progress update.
    SyncProgress {
        /// Files completed.
        completed: u64,
        /// Total files.
        total: u64,
    },

    // Media events
    /// Playback started.
    PlaybackStarted {
        /// Track name.
        track: String,
    },
    /// Playback paused.
    PlaybackPaused,
    /// Playback stopped.
    PlaybackStopped,
    /// Playback position changed.
    PlaybackPosition {
        /// Current position in seconds.
        position_secs: f64,
        /// Total duration in seconds.
        duration_secs: f64,
    },

    // Migration events
    /// Migration started.
    MigrationStarted {
        /// Total items to migrate.
        total_items: u64,
    },
    /// Migration progress.
    MigrationProgress {
        /// Items migrated.
        migrated: u64,
        /// Total items.
        total: u64,
    },
    /// Migration completed.
    MigrationCompleted {
        /// Number of items successfully migrated.
        successful: u64,
        /// Number of items that failed.
        failed: u64,
    },

    // General events
    /// Error occurred.
    Error {
        /// Error message.
        message: String,
    },
    /// Toast notification.
    Toast {
        /// Toast message.
        message: String,
        /// Toast level (info, warning, error).
        level: String,
    },
}

/// Event bus for broadcasting application events.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AppEvent>,
}

impl EventBus {
    /// Create a new event bus with specified capacity.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, event: AppEvent) {
        // Ignore send errors (no subscribers)
        let _ = self.sender.send(event);
    }

    /// Subscribe to events.
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}

/// Shared event bus wrapped in Arc for use across threads.
pub type SharedEventBus = Arc<EventBus>;

/// Create a new shared event bus.
#[must_use]
pub fn create_event_bus() -> SharedEventBus {
    Arc::new(EventBus::default())
}
