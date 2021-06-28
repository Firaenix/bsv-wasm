use std::io::{Cursor, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::EnumString;
use wasm_bindgen::prelude::*;
use crate::{Hash, PrivateKey, Script, Signature, transaction::*};
use anyhow::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive, ToPrimitive, EnumString)]
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
  InputOutput = 0xc3
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HashCache {
  pub(super) hash_prevouts: Option<Hash>,
  pub(super) hash_sequence: Option<Hash>,
  pub(super) hash_outputs: Option<Hash>,
  is_stale: bool
}

impl HashCache {
  /// Creates a new cache
  pub fn new() -> Self {
      HashCache {
          hash_prevouts: None,
          hash_sequence: None,
          hash_outputs: None,
          is_stale: true
      }
  }
}

impl Transaction {
  pub fn sign(&self, priv_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, presigned_script: &Script, value: u64) -> Result<Signature> {
    let mut buffer: Vec<u8> = vec![];
    let input = self.get_input(n_tx_in).ok_or(anyhow!(format!("Could not get TxIn at index {}", n_tx_in)))?;

    // Write nVersion
    buffer.write_u32::<LittleEndian>(self.version)?;

    // hashPrevouts
    let hash_inputs = match sighash {
      SigHash::Input | SigHash::InputOutput | SigHash::InputOutputs => [0; 32].to_vec(),
      _ => {
        let input_outpoints: Vec<u8> = self.inputs.iter().flat_map(|x| x.get_outpoint_bytes()).collect();

        Hash::sha_256d(&input_outpoints).to_bytes()
      }
    };
    buffer.write(&hash_inputs)?;

    // hashSequence
    let hash_sequence = match sighash {
      SigHash::InputsOutputs => [0; 32].to_vec(),
      _ => {
        let input_sequences: Vec<u8> = self.inputs.iter().flat_map(|x| x.get_sequence_as_bytes()).collect();

        Hash::sha_256d(&input_sequences).to_bytes()
      }
    };
    buffer.write(&hash_sequence)?;

    // outpoint (txid+vout)
    let outpoint = input.get_outpoint_bytes();
    buffer.write(&outpoint)?;

    // scriptCode
    buffer.write(&presigned_script.to_bytes())?;

    // value (satoshis)
    buffer.write_u64::<LittleEndian>(value)?;

    // nSequence
    buffer.write_u32::<LittleEndian>(input.get_sequence())?;

    // hashOutputs
    let hash_outputs = match sighash {
      // Only sign the output at the same index as the given txin
      SigHash::InputOutput | SigHash::InputsOutput => {
        if n_tx_in > self.get_noutputs() as usize {
          return Err(anyhow!("Cannot sign with SIGHASH_SINGLE given input index greater than number of outputs"));
        }

        let output_bytes = match self.outputs[n_tx_in].to_bytes() {
          Ok(v) => v,
          Err(e) => return Err(anyhow!(e))
        };
        Hash::sha_256d(&output_bytes).to_bytes()
      },
      // Sign all outputs
      SigHash::InputOutputs | SigHash::InputsOutputs => {
        let mut txout_bytes = Vec::new();
        for output in &self.outputs {
          txout_bytes.write(&output.to_bytes()?)?;
        }
        Hash::sha_256d(&txout_bytes).to_bytes()
      }
      _  => [0; 32].to_vec()
    };
    buffer.write(&hash_outputs)?;

    // nLocktime
    buffer.write_u32::<LittleEndian>(self.n_locktime)?;

    // SigHash type
    let sighash_u32 = ToPrimitive::to_u32(&sighash).ok_or(anyhow!(format!("Cannot convert SigHash {:?} into u32", sighash)))?;
    buffer.write_u32::<LittleEndian>(sighash_u32)?;
    
    let hash = Hash::sha_256d(&buffer);
    let sig = match priv_key.sign_message_impl(&hash.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(anyhow!(e))
    };

    Ok(sig)
  }
}