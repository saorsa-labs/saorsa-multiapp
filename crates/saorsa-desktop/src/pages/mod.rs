//! Application pages.

mod home;
mod media;
mod migration;
mod not_found;
mod settings;
mod sync;
mod wallet;

pub use home::Home;
pub use media::Media;
pub use migration::Migration;
pub use not_found::NotFound;
pub use settings::Settings;
pub use sync::Sync;
pub use wallet::Wallet;
