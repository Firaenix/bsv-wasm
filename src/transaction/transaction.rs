use std::{io::Cursor, u32};

use snafu::*;
use wasm_bindgen::{prelude::*, throw_str};
use crate::{TxIn, TxOut};
use byteorder::*;

#[derive(Debug, Snafu)]
pub enum TransactionError {
    #[snafu(display("Could not parse hex: {}", message))]
    ParseHex {
      message: String
    },
    #[snafu(display("Something went wrong: {}", message))]
    Other{
      message: String
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Transaction {
  pub version: u32,
  pub ninputs: u32,
  inputs: Vec<TxIn>,
  pub noutputs: u32,
  outputs: Vec<TxOut>,
  pub nlocktime: u32,
}

impl Transaction {
  pub(crate) fn from_hex_impl(hex_str: String) -> Result<Transaction, TransactionError> {
   let tx_bytes =  match hex::decode(&hex_str) {
     Ok(v) => v,
     Err(e) => return Err(TransactionError::ParseHex{ message: e.to_string() })
   };

   let mut cursor = Cursor::new(tx_bytes);

   // Version - 4 bytes
   let version = match cursor.read_u32::<LittleEndian>() {
     Ok(v) => v,
     Err(e) => return Err(TransactionError::Other { message: e.to_string() })
   };

   // In Counter - 1-9 tx_bytes
   // List of Inputs
   // Out Counter - 1-9 bytes
   // List of  Outputs
   // nLocktime - 4 bytes

  Ok(Transaction{
    version: version,
    ninputs: 0,
    inputs: [].to_vec(),
    noutputs: 0,
    outputs: [].to_vec(),
    nlocktime: 0
  })
 }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl Transaction {
 pub fn from_hex(hex_str: String) -> Result<Transaction, JsValue> {
  return match Transaction::from_hex_impl(hex_str) {
    Ok(v) => Ok(v),
    Err(e) => throw_str(&e.to_string())
  }
 }
}

#[cfg(not(target_arch = "wasm32"))]
impl Transaction {
 #[cfg(not(target_arch = "wasm32"))]
 pub fn from_hex(hex_str: String) -> Result<Transaction, TransactionError> {
  return Transaction::from_hex_impl(hex_str);
 }
}