use std::{io::Cursor};
use std::io::Read;
use std::io::Write;

use crate::{TxIn, TxOut, VarInt};
use anyhow::*;
use byteorder::*;
use wasm_bindgen::{prelude::*, throw_str, JsValue};
use serde::{Serialize, Deserialize};
use snafu::*;
use anyhow::*;

#[derive(Debug, Snafu)]
pub enum TransactionErrors {
    #[snafu(display("Error deserialising transaction field {:?}: {}", field, error))]
    Deserialise {
      field: Option<String>,
      error: anyhow::Error
    },
    #[snafu(display("Error serialising TxIn field {:?}: {}", field, error))]
  Serialise {
    field: Option<String>,
    error: anyhow::Error
  },

  #[snafu(display("Error serialising Tx to serde_json: {}", error))]
  JsonSerialise {
    error: serde_json::Error
  },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
  version: u32,
  inputs: Vec<TxIn>,
  outputs: Vec<TxOut>,
  n_locktime: u32,
}

impl Transaction {
  pub(crate) fn new_impl(version: u32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    n_locktime: u32) -> Transaction {
      Transaction{
        version,
        inputs,
        outputs,
        n_locktime
      }
    }

  pub(crate) fn from_hex_impl(hex_str: String) -> Result<Transaction, TransactionErrors> {
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
      inputs,
      outputs,
      n_locktime,
    })
  }

  pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, TransactionErrors> {
    let mut cursor = Cursor::new(Vec::new());

    // Version - 4 bytes
    match cursor.write_u32::<LittleEndian>(self.version) {
      Ok(_) => (),
      Err(e) => return Err(TransactionErrors::Serialise { field: Some("version".to_string()), error: anyhow!(e) }),
    };

    // In Counter - 1-9 tx_bytes
    match cursor.write_varint(self.get_ninputs()) {
      Ok(_) => (),
      Err(e) => return Err(TransactionErrors::Serialise { field: Some("n_inputs".to_string()), error: anyhow!(e) }),
    };

    // Inputs
    for i in 0..self.get_ninputs() {
      let input = &self.inputs[i as usize];
      let input_bytes = match input.to_bytes_impl() {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Serialise { field: Some(format!("input {}", i)), error: anyhow!(e) }),
      };

      match cursor.write(&input_bytes) {
        Ok(_) => (),
        Err(e) => return Err(TransactionErrors::Serialise { field: Some(format!("input {}", i)), error: anyhow!(e) }),
      };
    }

    // Out Counter - 1-9 tx_bytes
    match cursor.write_varint(self.get_noutputs()) {
      Ok(_) => (),
      Err(e) => return Err(TransactionErrors::Serialise { field: Some("n_outputs".to_string()), error: anyhow!(e) }),
    };

    // Outputs
    for i in 0..self.get_noutputs() {
      let output = &self.outputs[i as usize];
      let output_bytes = match output.to_bytes_impl() {
        Ok(v) => v,
        Err(e) => return Err(TransactionErrors::Serialise { field: Some(format!("output {}", i)), error: anyhow!(e) }),
      };

      match cursor.write(&output_bytes) {
        Ok(_) => (),
        Err(e) => return Err(TransactionErrors::Serialise { field: Some(format!("output {}", i)), error: anyhow!(e) }),
      };
    }

    // nLocktime - 4 bytes
    match cursor.write_u32::<LittleEndian>(self.n_locktime) {
      Ok(v) => v,
      Err(e) => return Err(TransactionErrors::Deserialise { field: Some("n_locktime".to_string()), error: anyhow!(e) })
    };

    // Write out bytes
    let mut bytes: Vec<u8> = Vec::new();
    cursor.set_position(0);
    match cursor.read_to_end(&mut bytes) {
      Err(e) => return Err(TransactionErrors::Serialise{ field: None, error: anyhow!(e) }),
      _ => ()
    };
    Ok(bytes)
  }

  pub(crate) fn to_hex_impl(&self) -> Result<String, TransactionErrors> {
    Ok(hex::encode(&self.to_bytes_impl()?))
  }

  pub(crate) fn to_json_string_impl(&self) -> Result<String, TransactionErrors> {
    match serde_json::to_string(self) {
      Ok(v) => Ok(v),
      Err(e) => Err(TransactionErrors::Serialise{ field: None, error: anyhow!(e) })
    } 
  }
}

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
#[wasm_bindgen]
impl Transaction {
  #[wasm_bindgen(js_name = getVersion)]
  pub fn get_version(&self) -> u32 {
    self.version
  }

  #[wasm_bindgen(js_name = getInputsCount)]
  pub fn get_ninputs(&self) -> u64 {
    self.inputs.len() as u64
  }

  #[wasm_bindgen(js_name = getOutputsCount)]
  pub fn get_noutputs(&self) -> u64 {
    self.outputs.len() as u64
  }

  #[wasm_bindgen(js_name = getInput)]
  pub fn get_input(&self, index: usize) -> Option<TxIn> {
    self.inputs.get(index).and_then(|x| Some(x.clone()) )
  }

  #[wasm_bindgen(js_name = getOutput)]
  pub fn get_output(&self, index: usize) -> Option<TxOut> {
    self.outputs.get(index).and_then(|x| Some(x.clone()) )
  }

  #[wasm_bindgen(js_name = getNLocktime)]
  pub fn get_n_locktime(&self) -> u32 {
    self.n_locktime
  }

  #[wasm_bindgen(js_name = getNLocktimeAsBytes)]
  pub fn get_n_locktime_as_bytes(&self) -> Vec<u8> {
    self.n_locktime.to_be_bytes().to_vec()
  }

  /**
   * Creates a new empty transaction where you need to add inputs and outputs
   * Transaction.add_input(TxIn) and Transaction.add_output(TxOut)
   */
  #[wasm_bindgen(constructor)]
  pub fn new(version: u32, n_locktime: u32) -> Transaction {
    Transaction::new_impl(version, vec![], vec![], n_locktime)
  }

  #[wasm_bindgen(js_name = addInput)]
  pub fn add_input(&mut self, input: &TxIn) -> () {
    self.inputs.push(input.clone());
  }

  #[wasm_bindgen(js_name = addOutput)]
  pub fn add_output(&mut self, output: &TxOut) -> () {
    self.outputs.push(output.clone());
  }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Transaction { 
  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String) -> Result<Transaction, JsValue> {
    return match Transaction::from_hex_impl(hex_str) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    };
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_json_string(&self) -> Result<String, JsValue> {
    match Transaction::to_json_string_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue, JsValue> {
    match JsValue::from_serde(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toBytes)]
  pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
    match Transaction::to_bytes_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> Result<String, JsValue> {
    match Transaction::to_hex_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  /**
   * Adds an array of TxIn's to the transaction
   * @param {TxIn[]} tx_ins
   */
  #[wasm_bindgen(js_name = addInputs)]
  pub fn add_inputs(&mut self, tx_ins: Box<[JsValue]>) {
    let js_value = &*tx_ins.to_vec();
    
    for elem in js_value {
      let input = elem.into_serde().unwrap();

      self.add_input(&input);
    }
  }

  /**
   * Adds an array of TxOuts to the transaction
   * @param {TxOut[]} tx_outs
   */
  #[wasm_bindgen(js_name = addOutputs)]
  pub fn add_outputs(&mut self, tx_outs: Box<[JsValue]>) {
    let js_value = &*tx_outs.to_vec();
    
    for elem in js_value {
      let output = elem.into_serde().unwrap();

      self.add_output(&output);
    }
  }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl Transaction {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_hex(hex_str: String) -> Result<Transaction, TransactionErrors> {
    return Transaction::from_hex_impl(hex_str);
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_json_string(&self) -> Result<String, TransactionErrors> {
    Transaction::to_json_string_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_json(&self) -> Result<serde_json::Value, TransactionErrors> {
    match serde_json::to_value(self) {
      Ok(v) => Ok(v),
      Err(e) => Err(TransactionErrors::Serialise{field: None, error: anyhow!(e)})
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_bytes(&self) -> Result<Vec<u8>, TransactionErrors> {
    Transaction::to_bytes_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_hex(&self) -> Result<String, TransactionErrors> {
    Transaction::to_hex_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn add_inputs(&mut self, tx_ins: Vec<TxIn>) {
    for txin in tx_ins {
      self.add_input(&txin);
    }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn add_outputs(&mut self, tx_outs: Vec<TxOut>) {
    for txout in tx_outs {
      self.add_output(&txout);
    }
  }
}
