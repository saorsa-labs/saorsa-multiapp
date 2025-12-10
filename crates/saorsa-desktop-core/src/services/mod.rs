//! Service layer interfaces (traits).
//!
//! These traits define the contracts between layers and allow for
//! dependency injection and testing with mocks.

pub mod media;
pub mod migration;
pub mod node;
pub mod sync;
pub mod wallet;

pub use media::MediaService;
pub use migration::MigrationService;
pub use node::NodeService;
pub use sync::SyncService;
pub use wallet::WalletService;
