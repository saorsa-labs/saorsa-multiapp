//! Session management with auto-lock.

use std::time::{Duration, Instant};

/// Session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// Locked.
    Locked,
    /// Unlocking in progress.
    Unlocking,
    /// Unlocked.
    Unlocked,
    /// Locking in progress.
    Locking,
}

/// Session manager for wallet lock/unlock.
pub struct SessionManager {
    state: SessionState,
    lock_timeout: Duration,
    last_activity: Instant,
}

impl SessionManager {
    /// Default lock timeout (15 minutes).
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15 * 60);

    /// Create a new session manager.
    #[must_use]
    pub fn new(lock_timeout: Duration) -> Self {
        Self {
            state: SessionState::Locked,
            lock_timeout,
            last_activity: Instant::now(),
        }
    }

    /// Get current session state.
    #[must_use]
    pub const fn state(&self) -> SessionState {
        self.state
    }

    /// Touch activity (reset auto-lock timer).
    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }

    /// Check if session should auto-lock.
    #[must_use]
    pub fn should_lock(&self) -> bool {
        self.state == SessionState::Unlocked && self.last_activity.elapsed() >= self.lock_timeout
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(Self::DEFAULT_TIMEOUT)
    }
}
