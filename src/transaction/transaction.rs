use std::{io::Cursor};
use std::io::Read;

use crate::{TxIn, TxOut, VarIntReader};
use anyhow::*;
use byteorder::*;
use wasm_bindgen::{prelude::*, throw_str, JsValue};
use serde::Serialize;
use snafu::*;
use anyhow::*;

#[derive(Debug, Snafu)]
pub enum TransactionErrors {
    #[snafu(display("Error deserialising transaction field {:?}: {}", field, error))]
    Deserialise {
      field: Option<String>,
      error: anyhow::Error
    },
    #[snafu(display("Error serialising transaction: {}", error))]
    Serialise {
      error: anyhow::Error
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Transaction {
  version: u32,
  n_inputs: u64,
  inputs: Vec<TxIn>,
  n_outputs: u64,
  outputs: Vec<TxOut>,
  n_locktime: u32,
}

impl Transaction {
  pub(crate) fn from_hex_impl<'f>(hex_str: String) -> Result<Transaction, TransactionErrors> {
    let tx_bytes = match hex::decode(&hex_str) {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: None, error: anyhow!(e) }),
    };

    let mut cursor = Cursor::new(tx_bytes);

    // Version - 4 bytes
    let version = match cursor.read_u32::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: Some("version".to_string()), error: anyhow!(e) }),
    };

    // In Counter - 1-9 tx_bytes
    let n_inputs = match cursor.read_varint() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: Some("n_inputs".to_string()), error: anyhow!(e) }),
    };

    let mut inputs: Vec<TxIn> = Vec::with_capacity(n_inputs as usize);

    // List of Inputs
    for i in 0..n_inputs {
      let tx_in = match TxIn::read_in(&mut cursor) {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Deserialise { field: Some(format!("tx_in {}", i)), error: anyhow!(e) })
      };
      inputs.push(tx_in);
    }

    // Out Counter - 1-9 bytes
    let n_outputs = match cursor.read_varint() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: Some("n_outputs".to_string()), error: anyhow!(e) }),
    };

    // List of  Outputs
    let mut outputs: Vec<TxOut> = Vec::with_capacity(n_outputs as usize);
    for i in 0..n_outputs {
      let tx_out = match TxOut::read_in(&mut cursor) {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Deserialise { field: Some(format!("tx_out {}", i)), error: anyhow!(e) })
      };

      outputs.push(tx_out);
    }

    // nLocktime - 4 bytes
    let n_locktime = match cursor.read_u32::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: Some("n_locktime".to_string()), error: anyhow!(e) })
    };

    Ok(Transaction {
      version,
      n_inputs,
      inputs,
      n_outputs,
      outputs,
      n_locktime,
    })
  }

  pub(crate) fn get_version_impl(&self) -> u32 {
    self.version
  }

  pub(crate) fn get_ninputs_impl(&self) -> u64 {
    self.n_inputs
  }

  pub(crate) fn get_input_impl(&self, index: usize) -> Option<TxIn> {
    self.inputs.get(index).and_then(|x| Some(x.clone()) )
  }

  pub(crate) fn get_output_impl(&self, index: usize) -> Option<TxOut> {
    self.outputs.get(index).and_then(|x| Some(x.clone()) )
  }

  pub(crate) fn to_json_impl(&self) -> Result<String, TransactionErrors> {
    match serde_json::to_string(self) {
      Ok(v) => Ok(v),
      Err(e) => Err(TransactionErrors::Serialise{error: anyhow!(e) })
    } 
  }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl Transaction {
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
  pub fn from_hex(hex_str: String) -> Result<Transaction, JsValue> {
    return match Transaction::from_hex_impl(hex_str) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    };
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getVersion))]
  pub fn get_version(&self) -> u32 {
    Transaction::get_version_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getInputsCount))]
  pub fn get_ninputs(&self) -> u64 {
    Transaction::get_ninputs_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getInput))]
  pub fn get_input(&self, index: usize) -> Option<TxIn> {
    Transaction::get_input_impl(&self, index)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getOutput))]
  pub fn get_output(&self, index: usize) -> Option<TxOut> {
    Transaction::get_output_impl(&self, index)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJSON))]
  pub fn to_json(&self) -> Result<String, JsValue> {
    match Transaction::to_json_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl Transaction {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_hex(hex_str: String) -> Result<Transaction, TransactionErrors> {
    return Transaction::from_hex_impl(hex_str);
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_version(&self) -> u32 {
    Transaction::get_version_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_ninputs(&self) -> u64 {
    Transaction::get_ninputs_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_input(&self, index: usize) -> Option<TxIn> {
    Transaction::get_input_impl(&self, index)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn get_output(&self, index: usize) -> Option<TxOut> {
    Transaction::get_output_impl(&self, index)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_json(&self) -> Result<String, TransactionErrors> {
    Transaction::to_json_impl(&self)
  }
}
