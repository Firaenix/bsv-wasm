
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
pub enum TxInErrors {
  #[snafu(display("Error deserialising transaction field {:?}: {}", field, error))]
  Deserialise {
    field: Option<String>,
    error: anyhow::Error
  },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TxIn {
  #[serde(serialize_with = "to_hex")]
  prev_tx_id: Vec<u8>,
  vout: u32,
  script_sig_size: u64,
  #[serde(serialize_with = "to_hex")]
  script_sig: Vec<u8>,
  sequence: u32,
}


impl TxIn {
  pub fn new(
    prev_tx_id: Vec<u8>,
    vout: u32,
    script_sig_size: u64,
    script_sig: Vec<u8>,
    sequence: u32,
  ) -> TxIn {
    TxIn {
      prev_tx_id,
      vout,
      script_sig_size,
      script_sig,
      sequence,
    }
  }

  pub fn read_in(
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
      script_sig_size,
      script_sig,
      sequence,
    })
  }

  pub(crate) fn get_prev_tx_id_impl(&self) -> Vec<u8> {
    self.prev_tx_id.clone()
  }

  pub(crate) fn get_prev_tx_id_hex_impl(&self) -> String {
    hex::encode(self.prev_tx_id.clone())
  }

  pub(crate) fn get_vout_impl(&self) -> u32 {
    self.vout
  }

  pub(crate) fn get_script_sig_size_impl(&self) -> u64 {
    self.script_sig_size
  }

  pub(crate) fn get_script_sig_impl(&self) -> Vec<u8> {
    self.script_sig.clone()
  }

  pub(crate) fn get_script_sig_hex_impl(&self) -> String {
    hex::encode(self.script_sig.clone())
  }

  pub(crate) fn get_sequence_impl(&self) -> u32 {
    self.sequence
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl TxIn {
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPrevTxId))]
  pub fn get_prev_tx_id(&self) -> Vec<u8> {
    TxIn::get_prev_tx_id_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getVOut))]
  pub fn get_vout(&self) -> u32 {
    TxIn::get_vout_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSigSize))]
  pub fn get_script_sig_size(&self) -> u64 {
    TxIn::get_script_sig_size_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSig))]
  pub fn get_script_sig(&self) -> Vec<u8> {
    TxIn::get_script_sig_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getSequence))]
  pub fn get_sequence(&self) -> u32 {
    TxIn::get_sequence_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPrevTxIdHex))]
  pub fn get_prev_tx_id_hex(&self) -> String {
    TxIn::get_prev_tx_id_hex_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSigHex))]
  pub fn get_script_sig_hex(&self) -> String {
    TxIn::get_script_sig_hex_impl(&self)
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl TxIn {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_prev_tx_id(&self) -> Vec<u8> {
    TxIn::get_prev_tx_id_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_vout(&self) -> u32 {
    TxIn::get_vout_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_sig_size(&self) -> u64 {
    TxIn::get_script_sig_size_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_sig(&self) -> Vec<u8> {
    TxIn::get_script_sig_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_sequence(&self) -> u32 {
    TxIn::get_sequence_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_prev_tx_id_hex(&self) -> String {
    TxIn::get_prev_tx_id_hex_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_script_sig_hex(&self) -> String {
    TxIn::get_script_sig_hex_impl(&self)
  }
}