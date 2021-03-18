use std::{borrow::Borrow, ops::Deref};

use wasm_bindgen::{JsStatic, prelude::*, throw_str};
use k256::{PublicKey as PubKey, Secp256k1};
use elliptic_curve::sec1::*;

use crate::PrivateKey;


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PublicKey {
  point: Vec<u8>,
  is_compressed: bool
}

#[wasm_bindgen]
impl PublicKey {
  #[wasm_bindgen(js_name = fromPrivateKey)]
  pub fn from_private_key(priv_key: &PrivateKey, compress: bool) -> PublicKey {
    PublicKey{
      point: priv_key.get_point(compress),
      is_compressed: compress
    }
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> Result<String, JsValue> {
    return match self.to_bytes() {
      Ok(v) => Ok(hex::encode(v)),
      Err(e) => Err(e)
    }
  }

  #[wasm_bindgen(js_name = toBytes)]
  pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
    let point: EncodedPoint<Secp256k1> = match EncodedPoint::from_bytes(&self.point.clone()) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };
    Ok(point.as_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = fromBytes)]
  pub fn from_bytes(bytes: Vec<u8>, compress: bool) -> Result<PublicKey, JsValue> {
    let point_bytes = bytes;
    let point: EncodedPoint<Secp256k1> = match EncodedPoint::from_bytes(point_bytes) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    Ok(PublicKey {
      point: point.compress().as_bytes().to_vec(),
      is_compressed: compress
    })
  }

  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String, compress: bool) -> Result<PublicKey, JsValue> {
    let point_bytes = match hex::decode(hex_str) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    PublicKey::from_bytes(point_bytes, compress)
  }
}