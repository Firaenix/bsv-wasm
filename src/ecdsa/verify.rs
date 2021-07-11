use crate::Signature;
use crate::{get_hash_digest, PublicKey, SigningHash, ECDSA};
use anyhow::*;
use ecdsa::signature::DigestVerifier;
use k256::{ecdsa::VerifyingKey, EncodedPoint};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

impl ECDSA {
    pub(crate) fn verify_digest_impl(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let point = EncodedPoint::from_bytes(pub_key_bytes)?;
        let key = VerifyingKey::from_encoded_point(&point)?;
        let digest = get_hash_digest(hash_algo, message);
        key.verify_digest(digest, &signature.sig)?;

        Ok(true)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECDSA {
    #[wasm_bindgen(js_name = verify)]
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, JsValue> {
        match ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDSA {
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool> {
        ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo)
    }
}
