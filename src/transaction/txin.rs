
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use crate::{Script, VarInt, utils::{to_hex, from_hex}};
use wasm_bindgen::{JsValue, prelude::*, throw_str};
use serde::*;

use snafu::*;
use anyhow::*;
use byteorder::*;

#[derive(Debug, Snafu)]
pub enum TxInErrors {
  #[snafu(display("Error deserialising TxIn field {:?}: {}", field, error))]
  Deserialise {
    field: Option<String>,
    error: anyhow::Error
  },

  #[snafu(display("Error serialising TxIn field {:?}: {}", field, error))]
  Serialise {
    field: Option<String>,
    error: anyhow::Error
  },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxIn {
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
  prev_tx_id: Vec<u8>,
  vout: u32,
  script_sig: Script,
  sequence: u32,
}

impl From<JsValue> for TxIn {
    fn from<'a>(x: JsValue) -> Self {
        match x.into_serde::<TxIn>() {
          Ok(v) => v,
          Err(_) => TxIn{prev_tx_id: vec![], script_sig: Script::default(), sequence: 0, vout:0}
        }
    }
}

impl TxIn {
  pub(crate) fn from_hex_impl(hex_str: String) -> Result<TxIn, TxInErrors> {
    let txin_bytes = match hex::decode(&hex_str) {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Deserialise { field: None, error: anyhow!(e) }),
    };

    let mut cursor = Cursor::new(txin_bytes);

    TxIn::read_in(&mut cursor)
  }

  pub(crate) fn read_in(
     cursor: &mut Cursor<Vec<u8>>
  ) -> Result<TxIn, TxInErrors> {
    // PrevTxId - 32 bytes
    let mut prev_tx_id = vec![0; 32];
    match cursor.read(&mut prev_tx_id) {
      Err(e) => return Err(TxInErrors::Deserialise { field: Some("prev_tx_id".to_string()), error: anyhow!(e) }),
      Ok(0) => return Err(TxInErrors::Deserialise { field: Some("prev_tx_id".to_string()), error: anyhow!("Read zero bytes for Prev TX Id!") }),
      Ok(v) => ()
    };
    // Error in the original bitcoin client means that all txids in TxIns are reversed
    prev_tx_id.reverse();

    // VOut - 4 bytes
    let vout = match cursor.read_u32::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Deserialise { field: Some("vout".to_string()), error: anyhow!(e) })
    };

    // Script Sig Size - VarInt
    let script_sig_size = match cursor.read_varint() {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Deserialise { field: Some("script_sig_size".to_string()), error: anyhow!(e) }),
    };

    // Script Sig
    let mut script_sig = vec![0; script_sig_size as usize];
    match cursor.read(&mut script_sig) {
      Err(e) => return Err(TxInErrors::Deserialise { field: Some("script_sig".to_string()), error: anyhow!(e) }),
      _ => () 
    };

    // Sequence - 4 bytes
    let sequence = match cursor.read_u32::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Deserialise { field: Some("sequence".to_string()), error: anyhow!(e) })
    };
    
    Ok(TxIn {
      prev_tx_id,
      vout,
      script_sig: Script(script_sig),
      sequence,
    })
  }

  pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, TxInErrors> {
    let mut cursor = Cursor::new(Vec::new());

    let mut txid = self.prev_tx_id.clone();
    txid.reverse();

    // Write Prev TxID first
    match cursor.write(&txid) {
      Err(e) => return Err(TxInErrors::Serialise { field: Some("prev_tx_id".to_string()), error: anyhow!(e) }),
      Ok(0) => return Err(TxInErrors::Serialise { field: Some("prev_tx_id".to_string()), error: anyhow!("Wrote zero bytes for Prev TX Id!") }),
      Ok(_) => ()
    };

    // Vout
    match cursor.write_u32::<LittleEndian>(self.vout) {
      Err(e) => return Err(TxInErrors::Serialise { field: Some("vout".to_string()), error: anyhow!(e) }),
      _ => ()
    };

    // Script Sig Size
     match cursor.write_varint(self.get_script_sig_size()) {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Serialise { field: Some("script_sig_size".to_string()), error: anyhow!(e) }),
    };

    // Script Sig
    match cursor.write(&self.script_sig.0) {
      Err(e) => return Err(TxInErrors::Serialise { field: Some("script_sig".to_string()), error: anyhow!(e) }),
      _ => () 
    };

    // Sequence
    match cursor.write_u32::<LittleEndian>(self.sequence) {
      Ok(v) => v,
      Err(e) => return Err(TxInErrors::Serialise { field: Some("sequence".to_string()), error: anyhow!(e) })
    };

    // Write out bytes
    let mut bytes: Vec<u8> = Vec::new();
    cursor.set_position(0);
    match cursor.read_to_end(&mut bytes) {
      Err(e) => return Err(TxInErrors::Serialise{ field: None, error: anyhow!(e) }),
      _ => ()
    };
    Ok(bytes)
  }

  pub(crate) fn to_hex_impl(&self) -> Result<String, TxInErrors> {
    Ok(hex::encode(&self.to_bytes_impl()?))
  }
}

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
#[wasm_bindgen]
impl TxIn {
  #[wasm_bindgen(constructor)]
  pub fn new(
    prev_tx_id: Vec<u8>,
    vout: u32,
    script_sig: Vec<u8>,
    sequence: u32,
  ) -> TxIn {
    TxIn {
      prev_tx_id,
      vout,
      script_sig: Script(script_sig),
      sequence,
    }
  }

  #[wasm_bindgen(js_name = getPrevTxId)]
  pub fn get_prev_tx_id(&self) -> Vec<u8> {
    self.prev_tx_id.clone()
  }

  #[wasm_bindgen(js_name = getPrevTxIdHex)]
  pub fn get_prev_tx_id_hex(&self) -> String {
    hex::encode(self.prev_tx_id.clone())
  }

  #[wasm_bindgen(js_name = getVOut)]
  pub fn get_vout(&self) -> u32 {
    self.vout
  }

  #[wasm_bindgen(js_name = getScriptSigSize)]
  pub fn get_script_sig_size(&self) -> u64 {
    self.script_sig.0.len() as u64
  }

  #[wasm_bindgen(js_name = getScriptSig)]
  pub fn get_script_sig(&self) -> Script {
    self.script_sig.clone()
  }

  #[wasm_bindgen(js_name = getScriptSigHex)]
  pub fn get_script_sig_hex(&self) -> String {
    hex::encode(self.script_sig.0.clone())
  }

  #[wasm_bindgen(js_name = getSequence)]
  pub fn get_sequence(&self) -> u32 {
    self.sequence
  }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TxIn {

  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String) -> Result<TxIn, JsValue> {
    match TxIn::from_hex_impl(hex_str) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toBytes)]
  pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
    match TxIn::to_bytes_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> Result<String, JsValue> {
    match TxIn::to_hex_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl TxIn {
  pub fn from_hex(hex_str: String) -> Result<TxIn, TxInErrors> {
    TxIn::from_hex_impl(hex_str)
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>, TxInErrors> {
    TxIn::to_bytes_impl(&self)
  }

  pub fn to_hex(&self) -> Result<String, TxInErrors> {
    TxIn::to_hex_impl(&self)
  }
}