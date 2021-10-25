//! Core codec functions for interacting with the XRPL.

pub mod algorithms;
pub mod exceptions;
#[cfg(test)]
pub(crate) mod test_cases;
pub(crate) mod utils;

use crate::constants::CryptoAlgorithm;
use crate::core::addresscodec::exceptions::XRPLAddressCodecException;
use crate::core::addresscodec::utils::SEED_LENGTH;
use crate::core::addresscodec::*;
use crate::core::keypairs::algorithms::Ed25519;
use crate::core::keypairs::exceptions::XRPLKeypairsException;
use crate::core::keypairs::utils::*;
use alloc::string::String;
use ed25519_dalek::SIGNATURE_LENGTH;
use rand::Rng;
use rand::SeedableRng;

/// Return the trait implementation for the provided
/// algorithm enum.
fn _get_algorithm_engine(algo: CryptoAlgorithm) -> impl CryptoImplementation {
    match algo {
        CryptoAlgorithm::ED25519 => Ed25519,
        CryptoAlgorithm::SECP256K1 => Ed25519,
    }
}

/// Return the trait implementation based on the
/// provided key.
fn _get_algorithm_engine_from_key(key: &str) -> impl CryptoImplementation {
    match &key[..2] {
        ED25519_PREFIX => _get_algorithm_engine(CryptoAlgorithm::ED25519),
        _ => _get_algorithm_engine(CryptoAlgorithm::SECP256K1),
    }
}

/// Generate a seed value that cryptographic keys
/// can be derived from.
pub fn generate_seed(
    entropy: Option<[u8; SEED_LENGTH]>,
    algorithm: Option<CryptoAlgorithm>,
) -> Result<String, XRPLAddressCodecException> {
    let mut random_bytes: [u8; SEED_LENGTH] = [0u8; SEED_LENGTH];
    let algo: CryptoAlgorithm;

    if let Some(value) = algorithm {
        algo = value;
    } else {
        algo = CryptoAlgorithm::ED25519;
    }

    if let Some(value) = entropy {
        random_bytes = value;
    } else {
        let mut rng = rand_hc::Hc128Rng::from_entropy();
        rng.fill(&mut random_bytes);
    }

    encode_seed(random_bytes, algo)
}

/// Derive the public and private keys from a given seed value.
pub fn derive_keypair(
    seed: &str,
    validator: bool,
) -> Result<(String, String), XRPLKeypairsException> {
    let (decoded_seed, algorithm) = decode_seed(seed)?;
    let module = _get_algorithm_engine(algorithm);
    let (public, private) = module.derive_keypair(&decoded_seed, validator)?;
    let signature = module.sign(SIGNATURE_VERIFICATION_MESSAGE, &private)?;

    if module.is_valid_message(SIGNATURE_VERIFICATION_MESSAGE, signature, &public) {
        Ok((public, private))
    } else {
        Err(XRPLKeypairsException::InvalidSignature)
    }
}

/// Derive the XRP Ledger classic address for a given
/// public key. For more information, see
/// Address Derivation:
/// `<https://xrpl.org/cryptographic-keys.html#account-id-and-address>`
pub fn derive_classic_address(public_key: &str) -> Result<String, XRPLAddressCodecException> {
    let account_id = get_account_id(&hex::decode(public_key)?);
    encode_classic_address(&account_id)
}

/// Sign a message using a given private key.
pub fn sign(message: &[u8], private_key: &str) -> Result<String, XRPLKeypairsException> {
    let module = _get_algorithm_engine_from_key(private_key);
    let result = module.sign(message, private_key)?;

    Ok(hex::encode_upper(result))
}

/// Verifies the signature on a given message.
pub fn is_valid_message(
    message: &[u8],
    signature: [u8; SIGNATURE_LENGTH],
    public_key: &str,
) -> bool {
    let module = _get_algorithm_engine_from_key(public_key);
    module.is_valid_message(message, signature, public_key)
}

/// Trait for cryptographic algorithms in the XRP Ledger.
/// The classes for all cryptographic algorithms are
/// derived from this trait.
pub(crate) trait CryptoImplementation {
    /// Derives a key pair for use with the XRP Ledger
    /// from a seed value.
    fn derive_keypair(
        &self,
        decoded_seed: &[u8],
        is_validator: bool,
    ) -> Result<(String, String), XRPLKeypairsException>;

    /// Signs a message using a given private key.
    /// * `message` - Text about foo.
    /// * `private_key` - Text about bar.
    fn sign(&self, message: &[u8], private_key: &str) -> Result<[u8; 64], XRPLKeypairsException>;

    /// Verifies the signature on a given message.
    fn is_valid_message(&self, message: &[u8], signature: [u8; 64], public_key: &str) -> bool;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::keypairs::test_cases::*;

    #[test]
    fn test_generate_seed() {
        assert!(generate_seed(None, None).is_ok());
        assert_eq!(SEED_ED25519, generate_seed(Some(TEST_BYTES), None).unwrap());
    }

    #[test]
    fn test_derive_keypair() {
        let (public, private) = derive_keypair(SEED_ED25519, false).unwrap();

        assert_eq!(PRIVATE_ED25519, private);
        assert_eq!(PUBLIC_ED25519, public);
    }

    #[test]
    fn test_derive_classic_address() {
        assert_eq!(
            CLASSIC_ADDRESS_ED25519,
            derive_classic_address(PUBLIC_ED25519).unwrap()
        );
    }

    #[test]
    fn test_sign() {
        assert_eq!(
            hex::encode_upper(SIGNATURE_ED25519),
            sign(TEST_MESSAGE.as_bytes(), PRIVATE_ED25519).unwrap()
        );
    }

    #[test]
    fn test_is_valid_message() {
        assert!(is_valid_message(
            TEST_MESSAGE.as_bytes(),
            SIGNATURE_ED25519,
            PUBLIC_ED25519
        ));
    }
}
