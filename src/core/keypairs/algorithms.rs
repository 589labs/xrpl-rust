//! Ed25519 elliptic curve cryptography interface.
//! SECP256K1 elliptic curve cryptography interface.
//!
//! Note: The process for using SECP256k1 is complex and
//! more involved than ED25519.
//!
//! See SECP256K1 Key Derivation:
//! `<https://xrpl.org/cryptographic-keys.html#secp256k1-key-derivation>`

use crate::constants::CryptoAlgorithm;
use crate::core::keypairs::exceptions::XRPLKeypairsException;
use crate::core::keypairs::utils::*;
use crate::core::keypairs::CryptoImplementation;
use alloc::format;
use alloc::string::String;
use core::str::FromStr;
use ed25519_dalek::Verifier;
use ed25519_dalek::SIGNATURE_LENGTH;
use num_bigint::BigUint;
use rust_decimal::prelude::One;
use secp256k1::constants::CURVE_ORDER;
use secp256k1::SignOnly;
use secp256k1::VerifyOnly;

/// MMethods for using the ECDSA cryptographic system with
/// the SECP256K1 elliptic curve.
pub struct Secp256k1;

/// Methods for using the ED25519 cryptographic system.
pub struct Ed25519;

impl Secp256k1 {
    fn _private_key_to_str(key: secp256k1::SecretKey) -> String {
        hex::encode(key.as_ref())
    }

    fn _public_key_to_str(key: secp256k1::PublicKey) -> String {
        hex::encode(key.serialize())
    }

    fn _format_key(keystr: &str) -> String {
        let padding = SECP256K1_KEY_LENGTH - keystr.len();
        format!("{:0<pad$}", keystr.to_uppercase(), pad = padding)
    }

    fn _format_keys(
        public: secp256k1::PublicKey,
        private: secp256k1::SecretKey,
    ) -> (String, String) {
        (
            Secp256k1::_format_key(&Secp256k1::_public_key_to_str(public)),
            Secp256k1::_format_key(&Secp256k1::_private_key_to_str(private)),
        )
    }

    fn _is_secret_valid(key: secp256k1::SecretKey) -> bool {
        let key_bytes = BigUint::from_bytes_be(key.as_ref());
        key_bytes >= BigUint::one() && key_bytes <= BigUint::from_bytes_be(&CURVE_ORDER)
    }

    //fn _get_secret()
}

impl Ed25519 {
    fn _public_key_to_str(key: ed25519_dalek::PublicKey) -> String {
        hex::encode(key.as_ref())
    }

    fn _private_key_to_str(key: ed25519_dalek::SecretKey) -> String {
        hex::encode(key)
    }

    fn _format_key(keystr: &str) -> String {
        format!("{}{}", ED25519_PREFIX, keystr.to_uppercase())
    }

    fn _format_keys(
        public: ed25519_dalek::PublicKey,
        private: ed25519_dalek::SecretKey,
    ) -> (String, String) {
        (
            Ed25519::_format_key(&Ed25519::_public_key_to_str(public)),
            Ed25519::_format_key(&Ed25519::_private_key_to_str(private)),
        )
    }
}

impl CryptoImplementation for Secp256k1 {
    fn derive_keypair(
        &self,
        decoded_seed: &[u8],
        _is_validator: bool,
    ) -> Result<(String, String), XRPLKeypairsException> {
        let secp = secp256k1::Secp256k1::new();
        let secret_key = secp256k1::SecretKey::from_slice(decoded_seed)?;
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

        Ok(Secp256k1::_format_keys(public_key, secret_key))
    }

    fn sign(
        &self,
        message_bytes: &[u8],
        private_key: &str,
    ) -> Result<[u8; 64], XRPLKeypairsException> {
        let secp = secp256k1::Secp256k1::<SignOnly>::signing_only();
        let message = secp256k1::Message::from_slice(message_bytes)?;
        let private = secp256k1::SecretKey::from_str(private_key)?;
        let signature = secp.sign(&message, &private);

        Ok(signature.serialize_compact())
    }

    fn is_valid_message(
        &self,
        message_bytes: &[u8],
        signature_compact: [u8; 64],
        public_key: &str,
    ) -> bool {
        let secp = secp256k1::Secp256k1::<VerifyOnly>::verification_only();
        let msg = secp256k1::Message::from_slice(message_bytes);
        let sig = secp256k1::Signature::from_compact(&signature_compact);
        let public = secp256k1::PublicKey::from_str(public_key);

        if let (&Ok(m), &Ok(s), &Ok(p)) = (&msg.as_ref(), &sig.as_ref(), &public.as_ref()) {
            secp.verify(m, s, p).is_ok()
        } else {
            false
        }
    }
}

impl CryptoImplementation for Ed25519 {
    fn derive_keypair(
        &self,
        decoded_seed: &[u8],
        is_validator: bool,
    ) -> Result<(String, String), XRPLKeypairsException> {
        if is_validator {
            Err(XRPLKeypairsException::UnsupportedValidatorAlgorithm {
                expected: CryptoAlgorithm::ED25519,
            })
        } else {
            let raw_private = sha512_first_half(decoded_seed);
            let private = ed25519_dalek::SecretKey::from_bytes(&raw_private)?;
            let public = ed25519_dalek::PublicKey::from(&private);

            Ok(Ed25519::_format_keys(public, private))
        }
    }

    fn sign(
        &self,
        message: &[u8],
        private_key: &str,
    ) -> Result<[u8; SIGNATURE_LENGTH], XRPLKeypairsException> {
        let raw_private = hex::decode(&private_key[ED25519_PREFIX.len()..])?;
        let private = ed25519_dalek::SecretKey::from_bytes(&raw_private)?;
        let expanded_private = ed25519_dalek::ExpandedSecretKey::from(&private);
        let public = ed25519_dalek::PublicKey::from(&private);
        let signature: ed25519_dalek::Signature = expanded_private.sign(message, &public);

        Ok(signature.to_bytes())
    }

    fn is_valid_message(
        &self,
        message: &[u8],
        signature: [u8; SIGNATURE_LENGTH],
        public_key: &str,
    ) -> bool {
        let raw_public = hex::decode(&public_key[ED25519_PREFIX.len()..]);

        if raw_public.is_err() {
            return false;
        };

        let public = ed25519_dalek::PublicKey::from_bytes(&raw_public.unwrap());

        if let Ok(value) = public {
            value
                .verify(message, &ed25519_dalek::Signature::from(signature))
                .is_ok()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::keypairs::test_cases::PUBLIC_ED25519;
    use crate::core::keypairs::test_cases::RAW_PRIVATE_ED25519;
    use crate::core::keypairs::test_cases::RAW_PUBLIC_ED25519;
    use crate::core::keypairs::test_cases::SEED_ED25519;
    use crate::core::keypairs::test_cases::SIGNATURE_ED25519;
    use crate::core::keypairs::test_cases::TEST_MESSAGE;

    // TODO
    // use super::*;
    // use crate::core::keypairs::test_cases::PRIVATE_SECP256K1;
    // use crate::core::keypairs::test_cases::PUBLIC_SECP256K1;
    // use crate::core::keypairs::test_cases::SEED_SECP256K1;
    // use crate::core::keypairs::test_cases::TEST_MESSAGE;

    #[test]
    fn test_secp256k1_derive_keypair() {
        // let (public, private) = Secp256k1
        //     .derive_keypair(SEED_SECP256K1.as_bytes(), false)
        //     .unwrap();

        //assert_eq!(PRIVATE_SECP256K1, public);
        //assert_eq!(PUBLIC_SECP256K1, private);
    }

    #[test]
    fn test_secp256k1_sign() {
        // let success = Secp256k1.sign(TEST_MESSAGE.as_bytes(), PRIVATE_SECP256K1);
        // let error = Secp256k1.sign(TEST_MESSAGE.as_bytes(), "abc123");

        // assert!(success.is_ok());
        // assert!(error.is_err());
    }

    #[test]
    fn test_secp256k1_is_valid_message() {
        // assert!(Secp256k1.is_valid_message(
        //     TEST_MESSAGE.as_bytes(),
        //     SIGNATURE_SECP256K1,
        //     PUBLIC_SECP256K1
        // ))
    }

    #[test]
    fn test_ed25519_derive_keypair() {
        let (public, private) = Ed25519
            .derive_keypair(SEED_ED25519.as_bytes(), false)
            .unwrap();

        assert_eq!(RAW_PRIVATE_ED25519, public);
        assert_eq!(RAW_PUBLIC_ED25519, private);
    }

    #[test]
    fn test_ed25519_sign() {
        let success = Ed25519.sign(TEST_MESSAGE.as_bytes(), RAW_PRIVATE_ED25519);
        let error = Ed25519.sign(TEST_MESSAGE.as_bytes(), "abc123");

        assert!(success.is_ok());
        assert!(error.is_err());
    }

    #[test]
    fn test_ed25519_is_valid_message() {
        assert!(Ed25519.is_valid_message(
            TEST_MESSAGE.as_bytes(),
            SIGNATURE_ED25519,
            PUBLIC_ED25519
        ))
    }
}
