
use std::io::Cursor;
use std::io::Read;

use crate::utils::{to_hex, from_hex};
use wasm_bindgen::{JsValue, convert::{FromWasmAbi, IntoWasmAbi}, prelude::*};
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxIn {
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
  prev_tx_id: Vec<u8>,
  vout: u32,
  script_sig_size: u64,
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
  script_sig: Vec<u8>,
  sequence: u32,
}

impl From<JsValue> for TxIn {
    fn from<'a>(x: JsValue) -> Self {
        match x.into_serde::<TxIn>() {
          Ok(v) => v,
          Err(_) => TxIn{prev_tx_id: vec![], script_sig: vec![], script_sig_size: 0, sequence: 0, vout:0}
        }
    }
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
      script_sig_size,
      script_sig,
      sequence,
    })
  }

  
}

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
#[wasm_bindgen]
impl TxIn {
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
    self.script_sig.len() as u64
  }

  #[wasm_bindgen(js_name = getScriptSig)]
  pub fn get_script_sig(&self) -> Vec<u8> {
    self.script_sig.clone()
  }

  #[wasm_bindgen(js_name = getScriptSigHex)]
  pub fn get_script_sig_hex(&self) -> String {
    hex::encode(self.script_sig.clone())
  }

  #[wasm_bindgen(js_name = getSequence)]
  pub fn get_sequence(&self) -> u32 {
    self.sequence
  }
}


/**
 * WASM Specific Functions
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl TxIn {
  
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl TxIn {
  
}