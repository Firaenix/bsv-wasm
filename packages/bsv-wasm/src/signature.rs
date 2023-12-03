use crate::{ecdsa::SigningHash, keypair::public_key::PublicKey};
use bsv::{RecoveryInfo as BSVRecoveryInfo, Signature as BSVSignature};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Signature(pub(crate) BSVSignature);

impl From<BSVSignature> for Signature {
    fn from(v: BSVSignature) -> Signature {
        Signature(v)
    }
}

#[wasm_bindgen]
pub struct RecoveryInfo(pub(crate) BSVRecoveryInfo);

impl From<BSVRecoveryInfo> for RecoveryInfo {
    fn from(v: BSVRecoveryInfo) -> RecoveryInfo {
        RecoveryInfo(v)
    }
}

impl From<RecoveryInfo> for BSVRecoveryInfo {
    fn from(v: RecoveryInfo) -> BSVRecoveryInfo {
        v.0
    }
}

#[wasm_bindgen]
impl RecoveryInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(is_y_odd: bool, is_x_reduced: bool, is_pubkey_compressed: bool) -> RecoveryInfo {
        RecoveryInfo(BSVRecoveryInfo::new(is_y_odd, is_x_reduced, is_pubkey_compressed))
    }

    pub fn from_byte(recovery_byte: u8, is_pubkey_compressed: bool) -> RecoveryInfo {
        RecoveryInfo(BSVRecoveryInfo::from_byte(recovery_byte, is_pubkey_compressed))
    }
}

#[wasm_bindgen]
impl Signature {
    pub fn from_der(bytes: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVSignature::from_der(bytes)?))
    }

    pub fn from_hex_der(hex: &str) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVSignature::from_hex_der(hex)?))
    }

    pub fn from_compact_bytes(compact_bytes: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVSignature::from_compact_impl(compact_bytes)?))
    }

    pub fn recover_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.recover_public_key(message, hash_algo.into())?))
    }

    pub fn recover_public_key_from_digest(&self, digest: &[u8]) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.recover_public_key_from_digest(digest)?))
    }

    pub fn to_der_hex(&self) -> String {
        BSVSignature::to_der_hex(&self.0)
    }

    pub fn to_der_bytes(&self) -> Vec<u8> {
        BSVSignature::to_der_bytes(&self.0)
    }

    pub fn to_compact_bytes(&self, recovery_info: Option<RecoveryInfo>) -> Vec<u8> {
        let r = match recovery_info {
            Some(v) => Some(v.0),
            None => None,
        };

        BSVSignature::to_compact_bytes(&self.0, r)
    }

    pub fn r(&self) -> Vec<u8> {
        BSVSignature::r(&self.0)
    }

    pub fn r_hex(&self) -> String {
        BSVSignature::r_hex(&self.0)
    }

    pub fn s(&self) -> Vec<u8> {
        BSVSignature::s(&self.0)
    }

    pub fn s_hex(&self) -> String {
        BSVSignature::s_hex(&self.0)
    }

    pub fn to_compact_hex(&self, recovery_info: Option<RecoveryInfo>) -> String {
        let r = match recovery_info {
            Some(v) => Some(v.0),
            None => None,
        };
        BSVSignature::to_compact_hex(&self.0, r)
    }

    pub fn verify_message(&self, message: &[u8], pub_key: &PublicKey) -> bool {
        BSVSignature::verify_message(&self.0, message, &pub_key.0)
    }
}
