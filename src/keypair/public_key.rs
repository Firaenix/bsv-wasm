use std::io::{Cursor, Read};

use crate::{BSVErrors, ECIESCiphertext, P2PKHAddress, Signature, SigningHash, ECDSA, ECIES};
use byteorder::ReadBytesExt;
use elliptic_curve::sec1::*;
use k256::Secp256k1;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str};

use crate::PrivateKey;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    point: Vec<u8>,
    is_compressed: bool,
}

impl PublicKey {
    pub(crate) fn from_private_key_impl(priv_key: &PrivateKey) -> PublicKey {
        PublicKey {
            point: priv_key.get_point(),
            is_compressed: priv_key.is_pub_key_compressed,
        }
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        let bytes = self.to_bytes_impl()?;
        Ok(hex::encode(bytes))
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let point: EncodedPoint<Secp256k1> = EncodedPoint::from_bytes(&self.point)?;
        Ok(point.as_bytes().to_vec())
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<PublicKey, BSVErrors> {
        let point: EncodedPoint<Secp256k1> = EncodedPoint::from_bytes(bytes)?;
        Ok(PublicKey::from_encoded_point(&point))
    }

    fn from_encoded_point(point: &EncodedPoint<Secp256k1>) -> PublicKey {
        PublicKey {
            point: point.as_bytes().to_vec(),
            is_compressed: point.is_compressed(),
        }
    }

    pub(crate) fn to_decompressed_impl(&self) -> Result<PublicKey, BSVErrors> {
        let point: EncodedPoint<Secp256k1> = EncodedPoint::from_bytes(&self.point)?;
        if let Some(decompressed_point) = point.decompress() {
            return Ok(PublicKey::from_encoded_point(&decompressed_point));
        }

        Ok(PublicKey::from_encoded_point(&point))
    }

    pub(crate) fn to_compressed_impl(&self) -> Result<PublicKey, BSVErrors> {
        let point: EncodedPoint<Secp256k1> = EncodedPoint::from_bytes(&self.point)?;
        Ok(PublicKey::from_encoded_point(&point.compress()))
    }

    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<PublicKey, BSVErrors> {
        let point_bytes = hex::decode(hex_str)?;
        PublicKey::from_bytes_impl(&point_bytes)
    }

    /**
     * Standard ECDSA Message Verification
     */
    pub(crate) fn verify_message_impl(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, self, signature, SigningHash::Sha256)
    }

    pub(crate) fn to_p2pkh_address_impl(&self) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_pubkey_impl(self)
    }

    /**
     * Encrypt a message to be sent to this public key with the provided private key.
     */
    pub(crate) fn encrypt_message_impl(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, BSVErrors> {
        ECIES::encrypt_impl(message, sender_private_key, self, false)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PublicKey {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = isValidMessage))]
    pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
        self.verify_message_impl(message, signature).is_ok()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = isCompressed))]
    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }
}

/**
 * WASM Exported Methods
 */
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PublicKey {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: &str) -> Result<PublicKey, JsValue> {
        match PublicKey::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, JsValue> {
        match PublicKey::from_bytes_impl(bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match PublicKey::to_bytes_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        match PublicKey::to_hex_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromPrivateKey))]
    pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
        PublicKey::from_private_key_impl(priv_key)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = verifyMessage))]
    pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, JsValue> {
        match self.verify_message_impl(message, signature) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toAddress))]
    pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, JsValue> {
        match self.to_p2pkh_address_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toCompressed))]
    pub fn to_compressed(&self) -> Result<PublicKey, JsValue> {
        match self.to_compressed_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toDecompressed))]
    pub fn to_decompressed(&self) -> Result<PublicKey, JsValue> {
        match self.to_decompressed_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encryptMessage))]
    pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, JsValue> {
        match self.encrypt_message_impl(message, sender_private_key) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PublicKey {
    pub fn from_hex(hex_str: &str) -> Result<PublicKey, BSVErrors> {
        PublicKey::from_hex_impl(hex_str)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, BSVErrors> {
        PublicKey::from_bytes_impl(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        PublicKey::to_bytes_impl(self)
    }

    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        PublicKey::to_hex_impl(self)
    }

    pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
        PublicKey::from_private_key_impl(priv_key)
    }

    pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        self.verify_message_impl(message, signature)
    }

    pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, BSVErrors> {
        self.to_p2pkh_address_impl()
    }

    pub fn to_compressed(&self) -> Result<PublicKey, BSVErrors> {
        self.to_compressed_impl()
    }

    pub fn to_decompressed(&self) -> Result<PublicKey, BSVErrors> {
        self.to_decompressed_impl()
    }

    pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, BSVErrors> {
        self.encrypt_message_impl(message, sender_private_key)
    }
}
