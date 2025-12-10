//! Core sync engine.

/// Sync engine state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SyncEngineState {
    /// Stopped.
    #[default]
    Stopped,
    /// Running.
    Running,
    /// Paused.
    Paused,
}

/// Core sync engine for file synchronization.
pub struct SyncEngine {
    state: SyncEngineState,
}

impl SyncEngine {
    /// Create a new sync engine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: SyncEngineState::Stopped,
        }
    }

    /// Get current state.
    #[must_use]
    pub fn state(&self) -> SyncEngineState {
        self.state
    }
}

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new()
    }
}
