//! Wallet service trait for key management.

use crate::error::Result;
use crate::state::{SessionState, WalletBalance, WalletInfo};
use async_trait::async_trait;

/// Trait for wallet operations.
#[async_trait]
pub trait WalletService: Send + Sync {
    // Vault operations

    /// Check if a vault exists.
    fn vault_exists(&self) -> bool;

    /// Create a new vault with master password.
    async fn create_vault(&self, password: &str) -> Result<()>;

    /// Unlock the vault with password.
    async fn unlock(&self, password: &str) -> Result<()>;

    /// Lock the vault.
    fn lock(&self);

    /// Get current session state.
    fn session_state(&self) -> SessionState;

    /// Change master password.
    async fn change_password(&self, old_password: &str, new_password: &str) -> Result<()>;

    // Wallet operations

    /// Generate a new wallet, returning the seed phrase.
    async fn generate_wallet(&self, name: &str) -> Result<GeneratedWallet>;

    /// Import wallet from BIP39 mnemonic.
    async fn import_from_mnemonic(
        &self,
        name: &str,
        mnemonic: &str,
        passphrase: Option<&str>,
    ) -> Result<WalletInfo>;

    /// Import wallet from raw keys.
    async fn import_from_keys(
        &self,
        name: &str,
        signing_key: &[u8],
        encryption_key: Option<&[u8]>,
        evm_key: Option<&[u8]>,
    ) -> Result<WalletInfo>;

    /// List all wallets.
    fn list_wallets(&self) -> Vec<WalletInfo>;

    /// Get active wallet.
    fn active_wallet(&self) -> Option<WalletInfo>;

    /// Set active wallet.
    fn set_active_wallet(&self, wallet_id: &[u8; 16]) -> Result<()>;

    /// Delete a wallet (requires password confirmation).
    async fn delete_wallet(&self, wallet_id: &[u8; 16], password: &str) -> Result<()>;

    /// Rename a wallet.
    async fn rename_wallet(&self, wallet_id: &[u8; 16], new_name: &str) -> Result<()>;

    // Crypto operations

    /// Sign a message with the active wallet.
    fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>>;

    /// Verify a signature.
    fn verify_signature(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool>;

    // Balance operations

    /// Get balance for a wallet.
    async fn get_balance(&self, wallet_id: &[u8; 16]) -> Result<WalletBalance>;

    /// Refresh all balances.
    async fn refresh_balances(&self) -> Result<Vec<WalletBalance>>;

    // Backup operations

    /// Export wallet seed phrase (requires password).
    fn export_seed_phrase(&self, wallet_id: &[u8; 16], password: &str) -> Result<String>;

    /// Export encrypted backup.
    async fn export_backup(&self, password: &str) -> Result<Vec<u8>>;

    /// Import from encrypted backup.
    async fn import_backup(&self, backup: &[u8], password: &str) -> Result<()>;
}

/// Result of generating a new wallet.
#[derive(Debug, Clone)]
pub struct GeneratedWallet {
    /// Wallet info.
    pub info: WalletInfo,
    /// Seed phrase (24 words) - SHOW ONCE ONLY.
    pub seed_phrase: String,
}
