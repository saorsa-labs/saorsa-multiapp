//! Application routing.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::{Home, Media, Migration, NotFound, Settings, Sync, Wallet};

/// Application routes.
#[derive(Clone, Routable, Debug, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    /// Home page.
    #[route("/")]
    Home {},

    /// Wallet management.
    #[route("/wallet")]
    Wallet {},

    /// File synchronization.
    #[route("/sync")]
    Sync {},

    /// Media player.
    #[route("/media")]
    Media {},

    /// Data migration.
    #[route("/migration")]
    Migration {},

    /// Settings.
    #[route("/settings")]
    Settings {},

    /// 404 page.
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
