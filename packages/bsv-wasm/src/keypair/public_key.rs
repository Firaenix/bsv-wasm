use bsv::PublicKey as BSVPublicKey;
use wasm_bindgen::prelude::*;

use crate::{address::P2PKHAddress, ecies::ECIESCiphertext, keypair::private_key::PrivateKey, signature::Signature};

#[derive(Clone)]
#[wasm_bindgen]
pub struct PublicKey(pub(crate) BSVPublicKey);

impl From<BSVPublicKey> for PublicKey {
    fn from(v: BSVPublicKey) -> PublicKey {
        PublicKey(v)
    }
}

impl From<PublicKey> for BSVPublicKey {
    fn from(v: PublicKey) -> BSVPublicKey {
        v.0
    }
}

#[wasm_bindgen]
impl PublicKey {
    pub fn to_address(&self) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(self.0.to_p2pkh_address()?))
    }

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
        Ok(self.0.verify_message(message, &signature.0)?)
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
        Ok(ECIESCiphertext(self.0.encrypt_message(message, &sender_private_key.0)?))
    }

    pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
        self.0.verify_message(message, &signature.0).is_ok()
    }

    pub fn is_compressed(&self) -> bool {
        self.0.is_compressed()
    }
}
