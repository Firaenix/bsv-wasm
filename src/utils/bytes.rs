use crate::utils::{from_hex, to_hex};
use hex::FromHexError;
use serde::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{throw_str, JsValue};

/**
 * A handy struct to allow calling of various utility methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bytes(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] pub(crate) Vec<u8>);

impl Bytes {
    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<Bytes, FromHexError> {
        let bytes = hex::decode(hex_str)?;
        Ok(Bytes(bytes))
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Bytes {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = readReverse))]
    pub fn to_slice_le(&self) -> Vec<u8> {
        let mut bytes = self.0.clone();
        bytes.reverse();
        bytes
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = read))]
    pub fn to_slice_be(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Bytes {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: &str) -> Result<Bytes, wasm_bindgen::JsError> {
       Ok(Bytes::from_hex_impl(hex_str)?)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Bytes {
    pub fn from_hex(hex_str: &str) -> Result<Bytes, FromHexError> {
        Bytes::from_hex_impl(hex_str)
    }
}
