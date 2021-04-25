use crate::PublicKeyErrors;
use std::{borrow::Borrow, ops::Deref};

use elliptic_curve::sec1::*;
use k256::{PublicKey as PubKey, Secp256k1};
use wasm_bindgen::{prelude::*, throw_str, JsStatic};

use crate::PrivateKey;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PublicKey {
    point: Vec<u8>,
    is_compressed: bool,
}

impl PublicKey {
   pub(crate) fn from_private_key_impl(priv_key: &PrivateKey, compress: bool) -> PublicKey {
        PublicKey {
            point: priv_key.get_point(compress),
            is_compressed: compress,
        }
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, PublicKeyErrors> {
        let bytes = self.to_bytes_impl()?;
        return Ok(hex::encode(bytes));
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, PublicKeyErrors> {
        let point: EncodedPoint<Secp256k1> = match EncodedPoint::from_bytes(&self.point.clone()) {
            Ok(v) => v,
            Err(e) => {
                return Err(PublicKeyErrors::InvalidPoint {
                  error: e,
                })
            }
        };
        Ok(point.as_bytes().to_vec())
    }

    pub(crate) fn from_bytes_impl(bytes: Vec<u8>, compress: bool) -> Result<PublicKey, PublicKeyErrors> {
        let point_bytes = bytes;
        let point: EncodedPoint<Secp256k1> = match EncodedPoint::from_bytes(point_bytes) {
            Ok(v) => v,
            Err(e) => {
                return Err(PublicKeyErrors::InvalidPoint {
                    error: e,
                })
            }
        };

        Ok(PublicKey {
            point: point.compress().as_bytes().to_vec(),
            is_compressed: compress,
        })
    }

    pub(crate) fn from_hex_impl(hex_str: String, compress: bool) -> Result<PublicKey, PublicKeyErrors> {
        let point_bytes = match hex::decode(hex_str) {
            Ok(v) => v,
            Err(e) => {
                return Err(PublicKeyErrors::ParseHex {
                    error: e
                })
            }
        };

        PublicKey::from_bytes_impl(point_bytes, compress)
    }
}

/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl PublicKey {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: String, compress: bool) -> Result<PublicKey, JsValue> {
        match PublicKey::from_hex_impl(hex_str, compress) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: Vec<u8>, compress: bool) -> Result<PublicKey, JsValue> {
        match PublicKey::from_bytes_impl(bytes, compress) {
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
    pub fn from_private_key(priv_key: &PrivateKey, compress: bool) -> PublicKey {
      PublicKey::from_private_key_impl(priv_key, compress)
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PublicKey {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex(hex_str: String, compress: bool) -> Result<PublicKey, PublicKeyErrors> {
        PublicKey::from_hex_impl(hex_str, compress)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_bytes(bytes: Vec<u8>, compress: bool) -> Result<PublicKey, PublicKeyErrors> {
        PublicKey::from_bytes_impl(bytes, compress)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, PublicKeyErrors> {
        PublicKey::to_bytes_impl(&self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_hex(&self) -> Result<String, PublicKeyErrors> {
        PublicKey::to_hex_impl(&self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_private_key(priv_key: &PrivateKey, compress: bool) -> PublicKey {
        PublicKey::from_private_key_impl(priv_key, compress)
    }
}
