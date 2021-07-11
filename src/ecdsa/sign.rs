use crate::get_hash_digest;
use crate::Digest32;
use crate::PrivateKey;
use crate::Signature;
use crate::ECDSA;
use crate::{reverse_digest::ReversibleDigest, Sha256r, SigningHash};
use anyhow::*;
use digest::{consts::U32, BlockInput, Digest, FixedOutput, Reset, Update};
use ecdsa::{
    hazmat::{FromDigest, RecoverableSignPrimitive},
    rfc6979::{self, generate_k},
};
use k256::FieldBytes;
use k256::{ecdsa::Signature as SecpSignature, Scalar, SecretKey};
use rand_core::OsRng;
use rand_core::RngCore;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;
use wasm_bindgen::JsValue;

impl ECDSA {
    fn sign_preimage_deterministic_k<D>(priv_key: &SecretKey, digest: D, reverse_endian_k: bool) -> Result<(SecpSignature, bool), ecdsa::Error>
    where
        D: Digest32,
    {
        let priv_scalar = priv_key.to_secret_scalar();
        let k_digest = match reverse_endian_k {
            true => digest.reverse(),
            false => digest.clone(),
        };
        let k = **generate_k(&priv_scalar, k_digest, &[]);
        let msg_scalar = Scalar::from_digest(digest);
        priv_scalar.try_sign_recoverable_prehashed(&k, &msg_scalar)
    }

    fn sign_preimage_random_k<D>(priv_key: &SecretKey, digest: D, reverse_endian_k: bool) -> Result<(SecpSignature, bool), ecdsa::Error>
    where
        D: Digest32,
    {
        let mut added_entropy = FieldBytes::default();
        let rng = &mut OsRng;
        rng.fill_bytes(&mut added_entropy);

        let priv_scalar = priv_key.to_secret_scalar();
        let k_digest = match reverse_endian_k {
            true => digest.reverse(),
            false => digest.clone(),
        };
        let k = **generate_k(&priv_scalar, k_digest, &added_entropy);
        let msg_scalar = Scalar::from_digest(digest);
        priv_scalar.try_sign_recoverable_prehashed(&k, &msg_scalar)
    }

    /**
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_deterministic_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature> {
        let digest = get_hash_digest(hash_algo, preimage);
        let (sig, is_recoverable) = ECDSA::sign_preimage_deterministic_k(&private_key.secret_key, digest, reverse_k)?;

        let signature = Signature::from_der_impl(sig.to_der().as_bytes().to_vec(), is_recoverable)?;

        Ok(signature)
    }

    /**
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_random_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature> {
        let digest = get_hash_digest(hash_algo, preimage);
        let (sig, is_recoverable) = ECDSA::sign_preimage_random_k(&private_key.secret_key, digest, reverse_k)?;

        let signature = Signature::from_der_impl(sig.to_der().as_bytes().to_vec(), is_recoverable)?;

        Ok(signature)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECDSA {
    #[wasm_bindgen(js_name = signWithRandomK)]
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = sign)]
    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDSA {
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature> {
        ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k)
    }

    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature> {
        ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k)
    }
}
