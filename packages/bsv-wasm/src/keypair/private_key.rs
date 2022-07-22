use wasm_bindgen::prelude::*;
use bsv::PrivateKey as BSVPrivateKey;

use crate::{keypair::public_key::PublicKey, ecies::ECIESCiphertext, signature::Signature};

#[wasm_bindgen]
pub struct PrivateKey(pub(crate) BSVPrivateKey);

#[wasm_bindgen]
impl PrivateKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn from_random() -> PrivateKey {
        PrivateKey(BSVPrivateKey::from_random())
    }

    pub fn get_point(&self) -> Vec<u8> {
        self.0.get_point()
    }

    pub fn compress_public_key(&self, should_compress: bool) -> PrivateKey {
       PrivateKey(self.0.compress_public_key(should_compress)) 
    }

    pub fn from_wif(wif_string: &str) -> Result<PrivateKey, wasm_bindgen::JsError> {
        Ok(PrivateKey(BSVPrivateKey::from_wif(wif_string)?))
    }

    pub fn from_hex(hex_str: &str) -> Result<PrivateKey, wasm_bindgen::JsError> {
        Ok(PrivateKey(BSVPrivateKey::from_hex(hex_str)?))
    }

    /**
     * Standard ECDSA Message Signing using SHA256 as the digestg
     */
    pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
        Ok(Signature(self.0.sign_message(msg)?))
    }

    pub fn to_wif(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(PrivateKey::to_wif(&self)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey, wasm_bindgen::JsError> {
        Ok(PrivateKey(BSVPrivateKey::from_bytes(bytes)?))
    }

    pub fn to_public_key(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.to_public_key()?))
    }

    /**
     * Encrypt a message to the public key of this private key.
     */
    pub fn encrypt_message(&self, message: &[u8]) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
        Ok(ECIESCiphertext(self.0.encrypt_message(message)?))
    }

    /**
     * Decrypt a message that was sent to the public key corresponding to this private key.
     */
    pub fn decrypt_message(&self, ciphertext: &ECIESCiphertext, sender_pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.decrypt_message(&ciphertext.0, &sender_pub_key.0)?)
    }
}

