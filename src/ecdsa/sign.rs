use crate::get_hash_digest;
use crate::BSVErrors;
use crate::PrivateKey;
use crate::Signature;
use crate::ECDSA;
use crate::{reverse_digest::ReversibleDigest, Sha256r, SigningHash};
use digest::{consts::U32, BlockInput, Digest, FixedOutput, Reset, Update};
use ecdsa::{
    hazmat::{FromDigest, RecoverableSignPrimitive},
    rfc6979::{self, generate_k},
};
use k256::FieldBytes;
use k256::{ecdsa::Signature as SecpSignature, Scalar, SecretKey};
use rand_core::OsRng;
use rand_core::RngCore;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::throw_str;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

impl ECDSA {
    fn sign_preimage_deterministic_k<D>(priv_key: &SecretKey, digest: D, reverse_endian_k: bool) -> Result<(SecpSignature, bool), ecdsa::Error>
    where
        D: digest::FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest,
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
        D: digest::FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest,
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
    pub(crate) fn sign_with_deterministic_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        let digest = get_hash_digest(hash_algo, preimage);
        let (sig, is_recoverable) = ECDSA::sign_preimage_deterministic_k(&private_key.secret_key, digest, reverse_k)?;

        let signature = Signature::from_der_impl(sig.to_der().as_bytes(), is_recoverable)?;

        Ok(signature)
    }

    /**
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_random_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        let digest = get_hash_digest(hash_algo, preimage);
        let (sig, is_recoverable) = ECDSA::sign_preimage_random_k(&private_key.secret_key, digest, reverse_k)?;

        let signature = Signature::from_der_impl(sig.to_der().as_bytes(), is_recoverable)?;

        Ok(signature)
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ECDSA {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = signWithRandomK))]
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = sign))]
    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDSA {
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k)
    }

    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k)
    }
}
