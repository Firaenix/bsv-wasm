use bsv::ECDSA as BSVECDSA;
use wasm_bindgen::prelude::*;

use crate::{
    keypair::{private_key::PrivateKey, public_key::PublicKey},
    signature::Signature,
};

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}

impl From<SigningHash> for bsv::SigningHash {
    fn from(item: SigningHash) -> Self {
        match item {
            SigningHash::Sha256 => bsv::SigningHash::Sha256,
            SigningHash::Sha256d => bsv::SigningHash::Sha256d,
        }
    }
}

#[wasm_bindgen]
pub struct ECDSA;

#[wasm_bindgen]
impl ECDSA {
    pub fn private_key_from_signature_k(
        signature: &Signature,
        public_key: &PublicKey,
        ephemeral_key: &PrivateKey,
        preimage: &[u8],
        hash_algo: SigningHash,
    ) -> Result<PrivateKey, wasm_bindgen::JsError> {
        Ok(PrivateKey(BSVECDSA::private_key_from_signature_k(
            &signature.0,
            &public_key.0,
            &ephemeral_key.0,
            preimage,
            hash_algo.into(),
        )?))
    }

    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVECDSA::sign_with_random_k(&private_key.0, preimage, hash_algo.into(), reverse_k)?))
    }

    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVECDSA::sign_with_deterministic_k(&private_key.0, preimage, hash_algo.into(), reverse_k)?))
    }

    pub fn sign_with_k(private_key: &PrivateKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVECDSA::sign_with_k(&private_key.0, &ephemeral_key.0, preimage, hash_algo.into())?))
    }

    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, wasm_bindgen::JsError> {
        Ok(BSVECDSA::verify_digest(message, &pub_key.0, &signature.0, hash_algo.into())?)
    }
}
