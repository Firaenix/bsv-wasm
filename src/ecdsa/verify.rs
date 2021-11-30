use crate::BSVErrors;
use crate::Signature;
use crate::{get_hash_digest, PublicKey, SigningHash, ECDSA};
use ecdsa::signature::DigestVerifier;
use k256::{ecdsa::VerifyingKey, EncodedPoint};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{throw_str, JsValue};

impl ECDSA {
    pub(crate) fn verify_digest_impl(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let point = EncodedPoint::from_bytes(pub_key_bytes)?;
        let key = VerifyingKey::from_encoded_point(&point)?;
        let digest = get_hash_digest(hash_algo, message);
        key.verify_digest(digest, &signature.sig)?;

        Ok(true)
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ECDSA {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = verify))]
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, JsValue> {
        match ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDSA {
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo)
    }
}
