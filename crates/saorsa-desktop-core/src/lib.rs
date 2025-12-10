//! Core business logic and state management for Saorsa desktop application.
//!
//! This crate provides:
//! - Application state management using signals
//! - Service layer interfaces (traits)
//! - Event bus for cross-module communication
//! - Configuration management
//! - Error handling

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod config;
pub mod error;
pub mod events;
pub mod services;
pub mod state;

pub use config::AppConfig;
pub use error::{Error, Result};
pub use events::{AppEvent, EventBus, SharedEventBus, create_event_bus};
pub use state::AppState;
