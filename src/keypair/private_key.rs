use crate::KeyPairError;
use std::borrow::Borrow;

use crate::Signature;
use bitcoin_hashes::{hex::ToHex, Hash};
use k256::ecdsa::{signature::Signer, Signature as SecpSignature, SigningKey};
use k256::{EncodedPoint, SecretKey};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen]
#[derive(Debug)]
pub struct PrivateKey {
    secret_key: SecretKey,
}

/**
 * Internal Methods
 */
impl PrivateKey {
    fn sign_message_impl(&self, msg: Vec<u8>) -> Result<Signature, KeyPairError> {
        let thingo = match SigningKey::from_bytes(&self.secret_key.to_bytes()) {
          Ok(v) => v,
          Err(e) => return Err(KeyPairError::ByteDecode{ message: e.to_string() })
        };

        // let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
        let message = &msg;

        // Note: the signature type must be annotated or otherwise inferrable as
        // `Signer` has many impls of the `Signer` trait (for both regular and
        // recoverable signature types).
        let signature: SecpSignature = thingo.sign(message);
        match Signature::from_der(signature.to_der().as_bytes().to_vec()) {
          Ok(v) => Ok(v),
          Err(e) => Err(KeyPairError::Other{ message: e.to_string() })
        }
    }

    fn to_hex_impl(&self) -> String {
        let secret_key_bytes = self.secret_key.to_bytes().to_vec();
        hex::encode(secret_key_bytes)
    }

    fn get_point_impl(&self, compressed: bool) -> Vec<u8> {
        EncodedPoint::from_secret_key(&self.secret_key, compressed)
            .as_bytes()
            .into()
    }

    fn to_wif_impl(&self, compressed: bool) -> String {
        // 1. Get Private Key hex
        let priv_key_hex = self.to_hex_impl();

        // 2. Add 0x80 in front + 0x01 to end if compressed pub key
        let padded_hex = match compressed {
            true => format!("80{}01", priv_key_hex),
            false => format!("80{}", priv_key_hex),
        };

        // 3. SHA256d
        let bytes = match hex::decode(padded_hex.clone()) {
            Ok(v) => v,
            Err(e) => wasm_bindgen::throw_str(&e.to_string()),
        };

        let shad_hex = bitcoin_hashes::sha256d::Hash::hash(&bytes);

        // 4. Take first 4 bytes as checksum
        let checksum = shad_hex.to_vec()[0..4].to_hex();

        // 5. Add checksum to end of padded private key
        let extended_key = format!("{}{}", padded_hex, checksum);

        // 6 Base58 Result
        let extended_key_bytes = match hex::decode(extended_key) {
            Ok(v) => v,
            Err(e) => wasm_bindgen::throw_str(&e.to_string()),
        };

        bs58::encode(extended_key_bytes).into_string()
    }

    fn from_random_impl() -> PrivateKey {
        let secret_key = k256::SecretKey::random(&mut OsRng);

        PrivateKey { secret_key }
    }

    fn from_hex_impl(hex_str: String) -> Result<PrivateKey, KeyPairError> {
        let bytes = match hex::decode(hex_str) {
            Ok(bytes) => bytes,
            Err(e) => throw_str(&e.to_string()),
        };

        let secret_key = match SecretKey::from_bytes(bytes) {
            Ok(key) => key,
            Err(e) => throw_str(&e.to_string()),
        };

        Ok(PrivateKey { secret_key })
    }

    fn from_wif_impl(wif_string: String) -> Result<PrivateKey, KeyPairError> {
        // 1. Decode from Base58
        let wif_bytes = match bs58::decode(wif_string).into_vec() {
            Ok(v) => v,
            Err(e) => throw_str(&e.to_string()),
        };

        let wif_without_checksum = wif_bytes[0..wif_bytes.len() - 4].to_vec();

        // 2. Check the Checksum
        let checksum = wif_bytes[wif_bytes.len() - 4..].to_hex();
        let check_hash = bitcoin_hashes::sha256d::Hash::hash(&wif_without_checksum);
        let check_string = check_hash.to_vec()[0..4].to_hex();

        if check_string.ne(&checksum) {
            throw_str("Checksum does not match! Invalid WIF");
        }

        // Private Key is 32 bytes + prefix is 33 bytes, if 34 bytes and ends with 01, compressed is true
        fn is_compressed(unchecksum: &Vec<u8>) -> bool {
            if unchecksum.len() < 34 {
                return false;
            }

            match unchecksum.last() {
                Some(last_byte) => last_byte.eq(&01),
                None => false,
            }
        }

        // 3. Check if compressed public key, return private key string

        let private_key_hex = match is_compressed(&wif_without_checksum) {
            true => wif_without_checksum[1..wif_without_checksum.len() - 1].to_hex(),
            false => wif_without_checksum[1..].to_hex(),
        };

        PrivateKey::from_hex_impl(private_key_hex.into())
    }
}

/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl PrivateKey {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
      self.to_hex_impl()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromWIF))]
    pub fn from_wif(wif_string: String) -> Result<PrivateKey, JsValue> {
      match PrivateKey::from_wif_impl(wif_string) {
        Ok(v) => Ok(v),
        Err(e) => throw_str(&e.to_string())
      }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromRandom))]
    pub fn from_random() -> PrivateKey {
      PrivateKey::from_random_impl()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: String) -> Result<PrivateKey, JsValue> {
      match PrivateKey::from_hex_impl(hex_str) {
        Ok(v) => Ok(v),
        Err(e) => throw_str(&e.to_string())
      }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn sign_message(&self, msg: Vec<u8>) -> Result<Signature, JsValue> {
      match self.sign_message_impl(msg) {
        Ok(v) => Ok(v),
        Err(e) => throw_str(&e.to_string())
      }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPoint))]
    pub fn get_point(&self, compressed: bool) -> Vec<u8> { 
      self.get_point(compressed)
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PrivateKey {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_hex(&self) -> String {
      self.to_hex_impl()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_wif(wif_string: String) -> Result<PrivateKey, KeyPairError> {
      PrivateKey::from_wif_impl(wif_string)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_random() -> PrivateKey {
      PrivateKey::from_random_impl()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex(hex_str: String) -> Result<PrivateKey, KeyPairError> {
      PrivateKey::from_hex_impl(hex_str)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn sign_message(&self, msg: Vec<u8>) -> Result<Signature, KeyPairError> {
      self.sign_message_impl(msg)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_point(&self, compressed: bool) -> Vec<u8> { 
      self.get_point(compressed)
    }
}
