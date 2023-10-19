use bsv::CipherKeys as BSVCipherKeys;
use bsv::ECIESCiphertext as BSVECIESCiphertext;
use bsv::ECIES as BSVECIES;
use wasm_bindgen::prelude::*;

use crate::keypair::private_key::PrivateKey;
use crate::keypair::public_key::PublicKey;

#[wasm_bindgen]
pub struct ECIES;

#[wasm_bindgen]
impl ECIES {
    pub fn encrypt(message: &[u8], sender_priv_key: &PrivateKey, recipient_pub_key: &PublicKey, exclude_pub_key: bool) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
        Ok(ECIESCiphertext(BSVECIES::encrypt(message, &sender_priv_key.0, &recipient_pub_key.0, exclude_pub_key)?))
    }

    /**
     * Encrypt with a randomly generate private key.
     * This is intended to be used if you want to anonymously send a party an encrypted message.
     */
    pub fn encrypt_with_ephemeral_private_key(message: &[u8], recipient_pub_key: &PublicKey) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
        Ok(ECIESCiphertext(BSVECIES::encrypt_with_ephemeral_private_key(message, &recipient_pub_key.0)?))
    }

    pub fn decrypt(ciphertext: &ECIESCiphertext, recipient_priv_key: &PrivateKey, sender_pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVECIES::decrypt(&ciphertext.0, &recipient_priv_key.0, &sender_pub_key.0)?)
    }

    pub fn derive_cipher_keys(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<CipherKeys, wasm_bindgen::JsError> {
        Ok(CipherKeys(BSVECIES::derive_cipher_keys(&priv_key.0, &pub_key.0)?))
    }
}

#[wasm_bindgen]
pub struct CipherKeys(pub(crate) BSVCipherKeys);

#[wasm_bindgen]
impl CipherKeys {
    pub fn get_iv(&self) -> Vec<u8> {
        self.0.get_iv()
    }

    pub fn get_ke(&self) -> Vec<u8> {
        self.0.get_ke()
    }

    pub fn get_km(&self) -> Vec<u8> {
        self.0.get_km()
    }
}

#[wasm_bindgen]
pub struct ECIESCiphertext(pub(crate) BSVECIESCiphertext);

#[wasm_bindgen]
impl ECIESCiphertext {
    pub fn get_ciphertext(&self) -> Vec<u8> {
        self.0.get_ciphertext()
    }

    pub fn get_hmac(&self) -> Vec<u8> {
        self.0.get_hmac()
    }

    pub fn get_cipher_keys(&self) -> Option<CipherKeys> {
        self.0.get_cipher_keys().map(CipherKeys)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn extract_public_key(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
        Ok(PublicKey(self.0.extract_public_key()?))
    }

    pub fn from_bytes(buffer: &[u8], has_pub_key: bool) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
        Ok(ECIESCiphertext(BSVECIESCiphertext::from_bytes(buffer, has_pub_key)?))
    }
}
