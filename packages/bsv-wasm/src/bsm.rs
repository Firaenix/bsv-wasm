use wasm_bindgen::prelude::*;
use bsv::BSM as BSVBSM;

use crate::{signature::Signature, address::P2PKHAddress, keypair::private_key::PrivateKey};

#[wasm_bindgen]
pub struct BSM;

#[wasm_bindgen]
impl BSM {
    /**
     * Sign a message with the intention of verifying with this same Address.
     * Used when using Bitcoin Signed Messages
     *
     * Returns boolean
     */
    pub fn is_valid_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> bool {
        BSVBSM::verify_message(message, &signature.0, &address.0).is_ok()
    }

    pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, wasm_bindgen::JsError> {
        Ok(BSVBSM::verify_message(message, &signature.0, &address.0)?)
    }

    pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVBSM::sign_message(&priv_key.0, message)?))
    }

    pub fn sign_message_with_k(priv_key: &PrivateKey, ephemeral_key: &PrivateKey, message: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(BSVBSM::sign_message_with_k(&priv_key.0, &ephemeral_key.0, message)?))
    }
}
