use std::io::{Cursor, Write};

use crate::{Hash, PrivateKey, Script, Signature, VarInt, transaction::*};
use anyhow::*;
use byteorder::{LittleEndian, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::EnumString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str};

#[wasm_bindgen]
#[derive(
  Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive, ToPrimitive, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum SigHash {
  FORKID = 0x40,
  ALL = 0x01,
  NONE = 0x02,
  SINGLE = 0x03,
  ANYONECANPAY = 0x80,
  /**
   * ALL | FORKID
   */
  InputsOutputs = 0x41,
  /**
   * NONE | FORKID
   */
  Inputs = 0x42,
  /**
   * SINGLE | FORKID
   */
  InputsOutput = 0x43,
  /**
   * ALL | ANYONECANPAY | FORKID
   */
  InputOutputs = 0xc1,
  /**
   * NONE | ANYONECANPAY | FORKID
   */
  Input = 0xc2,
  /**
   * SINGLE | ANYONECANPAY | FORKID
   */
  InputOutput = 0xc3,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HashCache {
  pub(super) hash_inputs: Option<Hash>,
  pub(super) hash_sequence: Option<Hash>,
  pub(super) hash_outputs: Option<Hash>,
}

impl HashCache {
  /// Creates a new cache
  pub fn new() -> Self {
    HashCache {
      hash_inputs: None,
      hash_sequence: None,
      hash_outputs: None,
    }
  }
}

impl Transaction {
  /**
   * Calculates the SIGHASH buffer and then signs it
   */
  pub(crate) fn sign_impl(
    &mut self,
    priv_key: &PrivateKey,
    sighash: SigHash,
    n_tx_in: usize,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Signature> {
    let buffer = self.sighash_impl(n_tx_in, sighash, presigned_script, value)?;
    let hash = Hash::sha_256d(&buffer);
    let sig = match priv_key.sign_message_impl(&hash.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(anyhow!(e)),
    };

    Ok(sig)
  }

  /**
   * Calculates the SIGHASH Buffer to be signed
   */
  pub(crate) fn sighash_impl(
    &mut self,
    n_tx_in: usize,
    sighash: SigHash,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Vec<u8>> {
    // This if statement is needed because of Consensus SIGHASH_SINGLE bug
    if sighash == SigHash::InputsOutput && n_tx_in >= self.outputs.len() - 1 as usize {
      let mut bugged_buf = vec!(0x0; 31);
      bugged_buf.push(01);
      return Ok(bugged_buf);
    }

    let mut buffer: Vec<u8> = vec![];

    let input = self
      .get_input(n_tx_in)
      .ok_or(anyhow!(format!("Could not get TxIn at index {}", n_tx_in)))?;

    let hashed_outputs = self.hash_outputs(sighash, n_tx_in)?;

    buffer
      .write_u32::<LittleEndian>(self.version)
      .and_then(|_| buffer.write(&self.hash_inputs(sighash)))
      .and_then(|_| buffer.write(&self.hash_sequence(sighash)))
      .and_then(|_| buffer.write(&input.get_outpoint_bytes()))
      .and_then(|_| buffer.write_varint(presigned_script.to_bytes().len() as u64))
      .and_then(|_| buffer.write(&presigned_script.to_bytes()))
      .and_then(|_| buffer.write_u64::<LittleEndian>(value))
      .and_then(|_| buffer.write_u32::<LittleEndian>(input.get_sequence()))
      .and_then(|_| buffer.write(&hashed_outputs))
      .and_then(|_| buffer.write_u32::<LittleEndian>(self.n_locktime))?;

    let sighash_u32 = ToPrimitive::to_u32(&sighash).ok_or(anyhow!(format!(
      "Cannot convert SigHash {:?} into u32",
      sighash
    )))?;
    buffer.write_u32::<LittleEndian>(sighash_u32)?;

    Ok(Hash::sha_256d(&buffer).to_bytes())
  }

  /**
   * Checks the hash cache to see if there already are hashed sequence, otherwise calculates the hash and adds it to the cache
   */
  fn hash_sequence(&mut self, sighash: SigHash) -> Vec<u8> {
    if let Some(x) = &self.hash_cache.hash_sequence {
      return x.to_bytes();
    }

    if sighash == SigHash::InputsOutputs {
      return [0; 32].to_vec()
    }

    let input_sequences: Vec<u8> = self
      .inputs
      .iter()
      .flat_map(|x| x.get_sequence_as_bytes())
      .collect();
    let hash = Hash::sha_256d(&input_sequences);
    self.hash_cache.hash_sequence = Some(hash.clone());
    hash.to_bytes()
  }

  /**
   * Checks the hash cache to see if there already are hashed outputs, otherwise calculates the hash and adds it to the cache
   */
  fn hash_outputs(&mut self, sighash: SigHash, n_tx_in: usize) -> Result<Vec<u8>> {
    if let Some(x) = &self.hash_cache.hash_outputs {
      return Ok(x.to_bytes());
    }

    match sighash {
      // Only sign the output at the same index as the given txin
      SigHash::SINGLE | SigHash::InputOutput | SigHash::InputsOutput => {
        if n_tx_in > self.get_noutputs() as usize {
          return Err(anyhow!("Cannot sign with SIGHASH_SINGLE given input index greater than number of outputs"));
        }

        let output = self.get_output(n_tx_in).ok_or(anyhow!(format!("Could not find output at index {}", n_tx_in)))?;
        let output_bytes = output.to_bytes_impl()?;
        Ok(Hash::sha_256d(&output_bytes).to_bytes())
      }
      // Sign all outputs
      SigHash::ALL | SigHash::InputOutputs | SigHash::InputsOutputs => {
        let mut txout_bytes = Vec::new();
        for output in &self.outputs {
          txout_bytes.write(&output.to_bytes_impl()?)?;
        }
        let hash = Hash::sha_256d(&txout_bytes);
        self.hash_cache.hash_outputs = Some(hash.clone());
        Ok(hash.to_bytes())
      }
      _ => Ok([0; 32].to_vec()),
    }
  }

  /**
   * (hashPrevouts) https://github.com/bitcoincashorg/bitcoincash.org/blob/master/spec/replay-protected-sighash.md
   * Checks the hash cache to see if there already are hashed inputs, otherwise calculates the hash and adds it to the cache.
   * 
   * Logic:
   * - If SigHash does not contain ANYONECANPAY, SHA256d all input outpoints
   * - Else 32 bytes of zeroes
   */
  fn hash_inputs(&mut self, sighash: SigHash) -> Vec<u8> {
    if let Some(x) = &self.hash_cache.hash_inputs {
      return x.to_bytes();
    }

    match sighash {
      SigHash::ANYONECANPAY | SigHash::Input | SigHash::InputOutput | SigHash::InputOutputs => [0; 32].to_vec(),
      _ => {
        let input_outpoints: Vec<u8> = self
          .inputs
          .iter()
          .flat_map(|x| x.get_outpoint_bytes())
          .collect();

        let hash = Hash::sha_256d(&input_outpoints);
        self.hash_cache.hash_inputs = Some(hash.clone());

        hash.to_bytes()
      }
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl Transaction {
  pub fn sign(
    &mut self,
    priv_key: &PrivateKey,
    sighash: SigHash,
    n_tx_in: usize,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Signature> {
    Transaction::sign_impl(self, priv_key, sighash, n_tx_in, presigned_script, value)
  }

  pub fn sighash(
    &mut self,
    n_tx_in: usize,
    sighash: SigHash,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Vec<u8>> {
    Transaction::sighash_impl(self, n_tx_in, sighash, presigned_script, value)
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Transaction {
  #[wasm_bindgen(js_name = sign)]
  pub fn sign(
    &mut self,
    priv_key: &PrivateKey,
    sighash: SigHash,
    n_tx_in: usize,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Signature, JsValue> {
    match Transaction::sign_impl(self, priv_key, sighash, n_tx_in, presigned_script, value) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
  
  #[wasm_bindgen(js_name = sighash)]
  pub fn sighash(
    &mut self,
    n_tx_in: usize,
    sighash: SigHash,
    presigned_script: &Script,
    value: u64,
  ) -> Result<Vec<u8>, JsValue> {
    match Transaction::sighash_impl(self, n_tx_in, sighash, presigned_script, value) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}