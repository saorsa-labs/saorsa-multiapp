//! BIP39 to Post-Quantum key derivation.

/// BIP39 to Post-Quantum Key Derivation.
///
/// Derives ML-DSA-65 and ML-KEM-768 keys from a BIP39 seed using HKDF.
pub struct Bip39PqDerivation;

impl Bip39PqDerivation {
    /// Domain separation for HKDF.
    pub const DOMAIN_SAORSA_ROOT: &'static [u8] = b"saorsa-pq-v1";
    /// Domain for ML-DSA-65 signing keys.
    pub const DOMAIN_MLDSA_65: &'static [u8] = b"saorsa/mldsa65/v1";
    /// Domain for ML-KEM-768 encryption keys.
    pub const DOMAIN_MLKEM_768: &'static [u8] = b"saorsa/mlkem768/v1";
    /// Domain for EVM keys.
    pub const DOMAIN_EVM: &'static [u8] = b"saorsa/evm/v1";
}
