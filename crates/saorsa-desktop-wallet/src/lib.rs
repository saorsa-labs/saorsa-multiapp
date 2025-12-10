//! Wallet and key management for Saorsa desktop application.
//!
//! This crate provides:
//! - BIP39 seed phrase generation and import
//! - ML-DSA-65 / ML-KEM-768 key derivation
//! - Encrypted vault storage (Argon2id + XChaCha20-Poly1305)
//! - EVM wallet integration
//! - Session management with auto-lock

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod bip39_pq;
pub mod keystore;
pub mod session;
pub mod vault;

pub use bip39_pq::Bip39PqDerivation;
pub use keystore::KeyStore;
pub use session::SessionManager;
pub use vault::{Vault, VaultHeader};
