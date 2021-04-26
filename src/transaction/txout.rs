use std::io::Cursor;
use std::io::Read;

use crate::utils::to_hex;
use wasm_bindgen::prelude::*;
use serde::*;

use crate::{VarIntReader};

use snafu::*;
use anyhow::*;
use byteorder::*;

#[derive(Debug, Snafu)]
pub enum TxOutErrors {
  #[snafu(display("Error deserialising transaction field {:?}: {}", field, error))]
  Deserialise {
    field: Option<String>,
    error: anyhow::Error
  },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct TxOut {
  value: i64,
  script_pub_key_size: u64,
  #[serde(serialize_with = "to_hex")]
  script_pub_key: Vec<u8>,
}

impl TxOut {
  pub(crate) fn new(value: i64, script_pub_key_size: u64, script_pub_key: Vec<u8>) -> TxOut {
    TxOut {
      value,
      script_pub_key,
      script_pub_key_size
    }
  }

  pub fn read_in(
      cursor: &mut Cursor<Vec<u8>>
  ) -> Result<TxOut, TxOutErrors> {
    // Satoshi Value - 8 bytes
    let satoshis = match cursor.read_i64::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TxOutErrors::Deserialise { field: Some("satoshis".to_string()), error: anyhow!(e) })
    };

    // Script Pub Key Size - 1-9 bytes
    let script_pub_key_size = match cursor.read_varint() {
      Ok(v) => v,
      Err(e) => return Err(TxOutErrors::Deserialise { field: Some("script_pub_key_size".to_string()), error: anyhow!(e) }),
    };

    // Script Pub Key
    let mut script_pub_key = vec![0; script_pub_key_size as usize];
    match cursor.read(&mut script_pub_key) {
      Err(e) => return Err(TxOutErrors::Deserialise { field: Some("script_pub_key".to_string()), error: anyhow!(e) }),
      _ => () 
    };

    Ok(TxOut {
      value: satoshis,
      script_pub_key,
      script_pub_key_size
    })
  }

  pub(crate) fn get_satoshi_value_impl(&self) -> i64 {
    self.value
  }

  pub(crate) fn get_script_pub_key_size_impl(&self) -> u64 {
    self.script_pub_key_size
  }

  pub(crate) fn get_script_pub_key_impl(&self) -> Vec<u8> {
    self.script_pub_key.clone()
  }

  pub(crate) fn get_script_pub_key_hex_impl(&self) -> String {
    hex::encode(self.script_pub_key.clone())
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl TxOut {
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getSatoshiValue))]
  pub fn get_satoshi_value(&self) -> i64 {
    TxOut::get_satoshi_value_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptPubKeySize))]
  pub fn get_script_pub_key_size(&self) -> u64 {
    TxOut::get_script_pub_key_size_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptPubKey))]
  pub fn get_script_pub_key(&self) -> Vec<u8> {
    TxOut::get_script_pub_key_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptPubKeyHex))]
  pub fn get_script_pub_key_hex(&self) -> String {
    TxOut::get_script_pub_key_hex_impl(&self)
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl TxOut {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_satoshi_value(&self) -> i64 {
    TxOut::get_satoshi_value_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_pub_key_size(&self) -> u64 {
    TxOut::get_script_pub_key_size_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_pub_key(&self) -> Vec<u8> {
    TxOut::get_script_pub_key_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_pub_key_hex(&self) -> String {
    TxOut::get_script_pub_key_hex_impl(&self)
  }
}