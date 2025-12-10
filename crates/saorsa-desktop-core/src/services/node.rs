//! Node service trait for network operations.

use crate::error::Result;
use crate::state::{NetworkStats, NodeStatus};
use async_trait::async_trait;
use std::net::SocketAddr;

/// Configuration for node connection.
#[derive(Debug, Clone)]
pub enum NodeConnectConfig {
    /// Start an embedded P2P node.
    Embedded {
        /// Port to listen on (0 = auto).
        port: u16,
        /// Bootstrap peers.
        bootstrap: Vec<String>,
    },
    /// Connect to a running external node.
    External {
        /// Node address.
        address: SocketAddr,
    },
}

/// Trait for interacting with the saorsa network.
#[async_trait]
pub trait NodeService: Send + Sync {
    /// Connect to the network.
    ///
    /// # Errors
    /// Returns an error if connection fails.
    async fn connect(&self, config: NodeConnectConfig) -> Result<()>;

    /// Disconnect from the network.
    ///
    /// # Errors
    /// Returns an error if disconnection fails.
    async fn disconnect(&self) -> Result<()>;

    /// Get the current connection status.
    fn status(&self) -> NodeStatus;

    /// Get network statistics.
    fn stats(&self) -> NetworkStats;

    /// Store data chunk on the network.
    ///
    /// # Errors
    /// Returns an error if storage fails.
    async fn put_chunk(&self, data: bytes::Bytes) -> Result<[u8; 32]>;

    /// Retrieve data chunk from the network.
    ///
    /// # Errors
    /// Returns an error if retrieval fails.
    async fn get_chunk(&self, address: &[u8; 32]) -> Result<Option<bytes::Bytes>>;

    /// Check if data exists on the network.
    ///
    /// # Errors
    /// Returns an error if check fails.
    async fn exists(&self, address: &[u8; 32]) -> Result<bool>;
}
