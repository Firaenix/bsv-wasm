use hex::FromHexError;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, throw_str};
use serde::*;
use crate::utils::{from_hex, to_hex};

/**
 * A handy struct to allow calling of various utility methods
 */
#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bytes (
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] 
  pub(crate) Vec<u8>
);

impl Bytes {
  pub(crate) fn from_hex_impl(hex_str: &str) -> Result<Bytes, FromHexError> {
    let bytes = hex::decode(hex_str)?;
    Ok(Bytes(bytes))
  }
}

#[wasm_bindgen]
impl Bytes {
  #[wasm_bindgen(js_name = readReverse)]
  pub fn to_slice_le(&self) -> Vec<u8> {
    let mut bytes = self.0.clone();
    bytes.reverse();
    bytes
  }

  #[wasm_bindgen(js_name = read)]
  pub fn to_slice_be(&self) -> Vec<u8> {
    self.0.clone()
  }

  pub fn reverse(&mut self) {
    self.0.reverse();
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
    hex::encode(&self.0)
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Bytes {
  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: &str) -> Result<Bytes,  JsValue> {
    match Bytes::from_hex_impl(hex_str) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl Bytes {
  pub fn from_hex(hex_str: &str) -> Result<Bytes,  FromHexError> {
    Bytes::from_hex_impl(hex_str)
  }
}