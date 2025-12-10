//! Saorsa Desktop Application
//!
//! A quantum-resistant desktop application for the Saorsa decentralized network.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod app;
mod components;
mod hooks;
mod pages;
mod router;

use app::App;

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::from_default_env().add_directive("saorsa=debug".parse().unwrap_or_default()),
        )
        .init();

    tracing::info!("Starting Saorsa Desktop Application");

    // Launch Dioxus desktop app
    dioxus::launch(App);
}
