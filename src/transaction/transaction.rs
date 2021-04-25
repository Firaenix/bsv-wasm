use std::{io::Cursor};
use std::io::Read;

use crate::{TransactionErrors, TxIn, TxOut, VarIntReader};
use anyhow::*;
use byteorder::*;
use snafu::*;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
  version: u32,
  ninputs: u64,
  inputs: Vec<TxIn>,
  noutputs: u64,
  outputs: Vec<TxOut>,
  nlocktime: u32,
}

impl Transaction {
  pub(crate) fn from_hex_impl(hex_str: String) -> Result<Transaction, TransactionErrors> {
    let tx_bytes = match hex::decode(&hex_str) {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
    };

    let mut cursor = Cursor::new(tx_bytes);

    // Version - 4 bytes
    let version = match cursor.read_u32::<LittleEndian>() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
    };

    // In Counter - 1-9 tx_bytes
    let ninputs = match cursor.read_varint() {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
    };

    let mut inputs: Vec<TxIn> = Vec::with_capacity(ninputs as usize);

    // List of Inputs
    for _i in 0..ninputs {
      // PrevTxId - 32 bytes
      let mut prev_tx_id = vec![0; 32];
      match cursor.read(&mut prev_tx_id) {
        Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
        Ok(0) => return Err(TransactionErrors::Deserialise { error: anyhow!("Read zero bytes for Prev TX Id!") }),
        Ok(v) => ()
      };
      // Error in the original bitcoin client means that all txids in TxIns are reversed
      prev_tx_id.reverse();

      // VOut - 4 bytes
      let vout = match cursor.read_u32::<LittleEndian>() {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) })
      };

      // Script Sig Size - VarInt
      let script_sig_size = match cursor.read_varint() {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
      };

      // Script Sig
      let mut script_sig = vec![0; script_sig_size as usize];
      match cursor.read(&mut script_sig) {
        Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) }),
        _ => () 
      };

      // Sequence - 4 bytes
      let sequence = match cursor.read_u32::<LittleEndian>() {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Deserialise { error: anyhow!(e) })
      };

      inputs.push(TxIn::new(prev_tx_id, vout, script_sig_size, script_sig, sequence))
    }

    // Out Counter - 1-9 bytes

    // List of  Outputs
    // nLocktime - 4 bytes

    Ok(Transaction {
      version,
      ninputs,
      inputs,
      noutputs: 0,
      outputs: [].to_vec(),
      nlocktime: 0,
    })
  }

  pub(crate) fn get_version_impl(&self) -> u32 {
    self.version
  }

  pub(crate) fn get_ninputs_impl(&self) -> u64 {
    self.ninputs
  }

  pub(crate) fn get_input_impl(&self, index: usize) -> Option<TxIn> {
    self.inputs.get(index).and_then(|x| Some(x.clone()) )
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
}
