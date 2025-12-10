//! Autonomi to Saorsa data migration for desktop application.
//!
//! This crate provides:
//! - Discovery of ant-node local data
//! - Discovery of autonomi network data
//! - Migration planning and execution
//! - Progress tracking
//! - Verification

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod executor;
pub mod planner;
pub mod scanner;
pub mod verifier;

pub use executor::MigrationExecutor;
pub use planner::MigrationPlanner;
pub use scanner::DataScanner;
pub use verifier::MigrationVerifier;
