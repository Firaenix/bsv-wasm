use wasm_bindgen::prelude::*;
use bsv::PublicKey as BSVPublicKey;

use crate::{keypair::private_key::PrivateKey, address::P2PKHAddress};

#[wasm_bindgen]
pub struct PublicKey(pub(crate) BSVPublicKey);

#[wasm_bindgen]
impl PublicKey {
    pub fn from_hex(hex_str: &str) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(BSVPublicKey::from_hex(hex_str)?))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(BSVPublicKey::from_bytes(bytes)?))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.to_bytes()?)
    }

    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_hex()?)
    }

    pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
        PublicKey(BSVPublicKey::from_private_key(&priv_key.0))
    }

    pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
        Ok(self.0.verify_message(message, signature)?)
    }

    pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(self.0.to_p2pkh_address()?))
    }

    pub fn to_compressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.to_compressed()?))
    }

    pub fn to_decompressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.to_decompressed()?))
    }

    pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
        Ok(self.0.encrypt_message(message, sender_private_key)?)
    }

    pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
        self.0.verify_message(message, signature).is_ok()
    }

    pub fn is_compressed(&self) -> bool {
        self.0.is_compressed()
    }
}
