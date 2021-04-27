use std::io::{Cursor, Write};
use std::io::Read;

use crate::{VarInt, utils::{to_hex, from_hex}};
use wasm_bindgen::prelude::*;
use serde::*;

use snafu::*;
use anyhow::*;
use byteorder::*;

#[derive(Debug, Snafu)]
pub enum TxOutErrors {
  #[snafu(display("Error deserialising TxOut field {:?}: {}", field, error))]
  Deserialise {
    field: Option<String>,
    error: anyhow::Error
  },

  #[snafu(display("Error serialising TxOut field {:?}: {}", field, error))]
  Serialise {
    field: Option<String>,
    error: anyhow::Error
  },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TxOut {
  value: i64,
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
  script_pub_key: Vec<u8>,
}

impl From<JsValue> for TxOut {
  fn from(x: JsValue) -> Self {
      match x.into_serde::<TxOut>() {
        Ok(v) => v,
        Err(_) => TxOut{script_pub_key: vec![], value: 0}
      }
  }
}

impl TxOut {

  pub(crate) fn from_hex_impl(hex_str: String) -> Result<TxOut, TxOutErrors> {
    let txout_bytes = match hex::decode(&hex_str) {
      Ok(v) => v,
      Err(e) => return Err(TxOutErrors::Deserialise { field: None, error: anyhow!(e) }),
    };

    let mut cursor = Cursor::new(txout_bytes);

    TxOut::read_in(&mut cursor)
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
      script_pub_key
    })
  }

  pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, TxOutErrors> {
    let mut cursor = Cursor::new(Vec::new());

    // Satoshi Value - 8 bytes
    match cursor.write_i64::<LittleEndian>(self.value) {
      Ok(v) => v,
      Err(e) => return Err(TxOutErrors::Serialise { field: Some("satoshis".to_string()), error: anyhow!(e) })
    };

    // Script Pub Key Size - 1-9 bytes
    match cursor.write_varint(self.get_script_pub_key_size()) {
      Ok(v) => v,
      Err(e) => return Err(TxOutErrors::Serialise { field: Some("script_pub_key_size".to_string()), error: anyhow!(e) }),
    };

    // Script Pub Key
    match cursor.write(&self.script_pub_key) {
      Err(e) => return Err(TxOutErrors::Serialise { field: Some("script_pub_key".to_string()), error: anyhow!(e) }),
      _ => () 
    };

    // Write out bytes
    let mut bytes: Vec<u8> = Vec::new();
    cursor.set_position(0);
    match cursor.read_to_end(&mut bytes) {
      Err(e) => return Err(TxOutErrors::Serialise{ field: None, error: anyhow!(e) }),
      _ => ()
    };
    Ok(bytes)
  }

  pub(crate) fn to_hex_impl(&self) -> Result<String, TxOutErrors> {
    Ok(hex::encode(&self.to_bytes_impl()?))
  }
}

#[wasm_bindgen]
impl TxOut {
  #[wasm_bindgen(constructor)]
  pub fn new(value: i64, script_pub_key: Vec<u8>) -> TxOut {
    TxOut {
      value,
      script_pub_key
    }
  }

  #[wasm_bindgen(js_name = getSatoshiValue)]
  pub fn get_satoshi_value(&self) -> i64 {
    self.value
  }

  #[wasm_bindgen(js_name = getScriptPubKeySize)]
  pub fn get_script_pub_key_size(&self) -> u64 {
    self.script_pub_key.len() as u64
  }

  #[wasm_bindgen(js_name = getScriptPubKey)]
  pub fn get_script_pub_key(&self) -> Vec<u8> {
    self.script_pub_key.clone()
  }

  #[wasm_bindgen(js_name = getScriptPubKeyHex)]
  pub fn get_script_pub_key_hex(&self) -> String {
    hex::encode(self.script_pub_key.clone())
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TxOut {

  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String) -> Result<TxOut, JsValue> {
    match TxOut::from_hex_impl(hex_str) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toBuffer)]
  pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
    match TxOut::to_bytes_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> Result<String, JsValue> {
    match TxOut::to_hex_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl TxOut {
  pub fn from_hex(hex_str: String) -> Result<TxOut, TxOutErrors> {
    TxOut::from_hex_impl(hex_str)
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>, TxOutErrors> {
    TxOut::to_bytes_impl(&self)
  }

  pub fn to_hex(&self) -> Result<String, TxOutErrors> {
    TxOut::to_hex_impl(&self)
  }
}