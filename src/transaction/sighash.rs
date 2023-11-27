use crate::get_hash_digest;
use crate::BSVErrors;
use crate::ReversibleDigest;
use crate::ECDSA;
use std::convert::TryFrom;
use std::io::Write;

use crate::{transaction::*, Hash, PrivateKey, PublicKey, Script, Signature};
use byteorder::{LittleEndian, WriteBytesExt};
use digest::FixedOutput;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, FromPrimitive, ToPrimitive, EnumString, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum SigHash {
    FORKID = 0x40,
    ALL = 0x01,
    NONE = 0x02,
    SINGLE = 0x03,
    ANYONECANPAY = 0x80,
    // MAGIC = 0x21e8, - Idea for the future
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

    /**
     * ALL | ANYONECANPAY
     */
    Legacy_InputOutputs = 0x81,
    /**
     * NONE | ANYONECANPAY
     */
    Legacy_Input = 0x82,
    /**
     * SINGLE | ANYONECANPAY
     */
    Legacy_InputOutput = 0x83,
}

impl TryFrom<u8> for SigHash {
    type Error = BSVErrors;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        FromPrimitive::from_u8(value).ok_or_else(|| BSVErrors::ToSighash(format!("Could not convert {} into a valid SigHash value", value)))
    }
}

impl std::ops::BitOr for SigHash {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self.to_u8().unwrap();
        lhs | rhs.to_u8().unwrap()
    }
}

impl std::ops::BitAnd for SigHash {
    type Output = u8;

    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self.to_u8().unwrap();
        lhs & rhs.to_u8().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
    pub(crate) fn sign_impl(&mut self, priv_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<SighashSignature, BSVErrors> {
        let buffer = self.sighash_preimage_impl(n_tx_in, sighash, unsigned_script, value)?;

        let signature = ECDSA::sign_with_deterministic_k_impl(priv_key, &buffer, crate::SigningHash::Sha256d, true)?;

        Ok(SighashSignature {
            signature,
            sighash_type: sighash,
            sighash_buffer: buffer,
        })
    }

    /**
     * Calculates the SIGHASH buffer and then signs it with a specific ephemeral key. I hope you know what you're doing!
     */
    pub(crate) fn sign_with_k_impl(
        &mut self,
        priv_key: &PrivateKey,
        ephemeral_key: &PrivateKey,
        sighash: SigHash,
        n_tx_in: usize,
        unsigned_script: &Script,
        value: u64,
    ) -> Result<SighashSignature, BSVErrors> {
        let buffer = self.sighash_preimage_impl(n_tx_in, sighash, unsigned_script, value)?;

        let signature = ECDSA::sign_with_k_impl(priv_key, ephemeral_key, &buffer, crate::SigningHash::Sha256d)?;

        Ok(SighashSignature {
            signature,
            sighash_type: sighash,
            sighash_buffer: buffer,
        })
    }

    /**
     * Calculates the SIGHASH Buffer to be signed
     */
    pub(crate) fn sighash_preimage_impl(&mut self, n_tx_in: usize, sighash: SigHash, unsigned_script: &Script, value: u64) -> Result<Vec<u8>, BSVErrors> {
        // If uses any of the FORK_ID sighash variants
        // Gross, fix this. Maybe a nice method on SigHash enum to check if contains another SigHash type
        match sighash {
            SigHash::Input | SigHash::InputOutput | SigHash::InputOutputs | SigHash::Inputs | SigHash::InputsOutput | SigHash::InputsOutputs => {
                self.sighash_bip143(n_tx_in, sighash, unsigned_script, value)
            }
            _ => self.sighash_legacy(n_tx_in, sighash, unsigned_script),
        }
    }

    pub(crate) fn sighash_legacy(&mut self, n_tx_in: usize, sighash: SigHash, unsigned_script: &Script) -> Result<Vec<u8>, BSVErrors> {
        let mut tx = self.clone();
        let mut script = unsigned_script.clone();
        script.remove_codeseparators();

        // Empty scripts
        tx.inputs.iter_mut().for_each(|txin| txin.set_unlocking_script(&Script::default()));

        let mut prev_txin = tx.get_input(n_tx_in).ok_or_else(|| BSVErrors::OutOfBounds(format!("Could not get TxIn at index {}", n_tx_in)))?;
        prev_txin.set_unlocking_script(&script);
        tx.set_input(n_tx_in, &prev_txin);

        match sighash {
            SigHash::SINGLE | SigHash::Legacy_InputOutput => {
                // Not supporting the SIGHASH_SINGLE bug. Sue me craig.
                // // This if statement is needed because of Consensus SIGHASH_SINGLE bug
                // // https://bitcoinfiles.org/t/9a3a165cc7881bb2e37567dec5eaab64568a889e83e6b850b42f347e1d96a555
                // if n_tx_in >= tx.outputs.len() {
                //   return Ok(hex::decode("0000000000000000000000000000000000000000000000000000000000000001").map_err(|e| anyhow!(e))?)
                // }

                let txout = tx.get_output(n_tx_in).ok_or_else(|| BSVErrors::OutOfBounds(format!("Could not get TxOut at index {}", n_tx_in)))?;
                tx.outputs = vec![txout];

                for i in 0..tx.outputs.len() {
                    if i < n_tx_in {
                        tx.set_output(i, &TxOut::new(0xffffffffffffffff, &Script::default()));
                    }
                }

                for i in 0..tx.inputs.len() {
                    if i == n_tx_in {
                        continue;
                    }

                    tx.inputs[i].set_sequence(0x00000000);
                }
            }

            SigHash::NONE | SigHash::Legacy_Input => {
                tx.outputs.clear();

                for i in 0..tx.inputs.len() {
                    if i == n_tx_in {
                        continue;
                    }

                    tx.inputs[i].set_sequence(0x00000000);
                }
            }
            _ => {}
        }

        if sighash.ge(&SigHash::ANYONECANPAY) {
            let input = tx.inputs[n_tx_in].clone();
            tx.inputs = vec![];
            tx.add_input(&input);
        }

        let mut buffer = tx.to_bytes_impl()?;
        let sighash_i32 = sighash.to_i32().ok_or_else(|| BSVErrors::FromSighash(format!("Cannot convert SigHash {:?} into i32", sighash)))?;
        buffer.write_i32::<LittleEndian>(sighash_i32)?;

        Ok(buffer)
    }

    pub(crate) fn sighash_bip143(&mut self, n_tx_in: usize, sighash: SigHash, unsigned_script: &Script, value: u64) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer: Vec<u8> = vec![];

        let input = self.get_input(n_tx_in).ok_or_else(|| BSVErrors::OutOfBounds(format!("Could not get TxIn at index {}", n_tx_in)))?;

        let hashed_outputs = self.hash_outputs(sighash, n_tx_in)?;

        buffer.write_u32::<LittleEndian>(self.version)?;
        buffer.write_all(&self.hash_inputs(sighash))?;
        buffer.write_all(&self.hash_sequence(sighash))?;
        buffer.write_all(&input.get_outpoint_bytes(Some(true)))?;
        buffer.write_varint(unsigned_script.to_bytes().len() as u64)?;
        buffer.write_all(&unsigned_script.to_bytes())?;
        buffer.write_u64::<LittleEndian>(value)?;
        buffer.write_u32::<LittleEndian>(input.get_sequence())?;
        buffer.write_all(&hashed_outputs)?;
        buffer.write_u32::<LittleEndian>(self.n_locktime)?;

        let sighash_u32 = sighash.to_u32().ok_or_else(|| BSVErrors::FromSighash(format!("Cannot convert SigHash {:?} into u32", sighash)))?;
        buffer.write_u32::<LittleEndian>(sighash_u32)?;

        Ok(buffer)
    }

    /**
     * Checks the hash cache to see if there already are hashed sequence, otherwise calculates the hash and adds it to the cache
     */
    fn hash_sequence(&mut self, sighash: SigHash) -> Vec<u8> {
        match sighash {
            SigHash::ALL | SigHash::InputsOutputs => {
                if let Some(x) = &self.hash_cache.hash_sequence {
                    return x.to_bytes();
                }
                let input_sequences: Vec<u8> = self.inputs.iter().flat_map(|x| x.get_sequence_as_bytes()).collect();
                let hash = Hash::sha_256d(&input_sequences);
                self.hash_cache.hash_sequence = Some(hash.clone());
                hash.to_bytes()
            }
            _ => [0; 32].to_vec(),
        }
    }

    /**
     * Checks the hash cache to see if there already are hashed outputs, otherwise calculates the hash and adds it to the cache
     */
    fn hash_outputs(&mut self, sighash: SigHash, n_tx_in: usize) -> Result<Vec<u8>, BSVErrors> {
        match sighash {
            // Only sign the output at the same index as the given txin
            SigHash::SINGLE | SigHash::InputOutput | SigHash::Legacy_InputOutput | SigHash::InputsOutput => {
                if n_tx_in > self.get_noutputs() {
                    return Err(BSVErrors::OutOfBounds("Cannot sign with SIGHASH_SINGLE given input index greater than number of outputs".into()));
                }

                let output = self.get_output(n_tx_in).ok_or_else(|| BSVErrors::OutOfBounds(format!("Could not find output at index {}", n_tx_in)))?;
                let output_bytes = output.to_bytes_impl()?;
                Ok(Hash::sha_256d(&output_bytes).to_bytes())
            }
            // Sign all outputs
            SigHash::ALL | SigHash::InputOutputs | SigHash::Legacy_InputOutputs | SigHash::InputsOutputs => {
                if let Some(x) = &self.hash_cache.hash_outputs {
                    return Ok(x.to_bytes());
                }
                let mut txout_bytes = Vec::new();
                for output in &self.outputs {
                    txout_bytes.write_all(&output.to_bytes_impl()?)?;
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
    pub fn hash_inputs(&mut self, sighash: SigHash) -> Vec<u8> {
        match sighash {
            SigHash::ANYONECANPAY | SigHash::Input | SigHash::InputOutput | SigHash::Legacy_Input | SigHash::Legacy_InputOutput | SigHash::InputOutputs => [0; 32].to_vec(),
            _ => {
                if let Some(x) = &self.hash_cache.hash_inputs {
                    return x.to_bytes();
                }
                let input_bytes: Vec<u8> = self.inputs.iter().flat_map(|txin| txin.get_outpoint_bytes(Some(true))).collect();

                let hash = Hash::sha_256d(&input_bytes);
                self.hash_cache.hash_inputs = Some(hash.clone());

                hash.to_bytes()
            }
        }
    }
}

impl Transaction {
    pub fn verify(&self, pub_key: &PublicKey, sig: &SighashSignature) -> bool {
        ECDSA::verify_digest_impl(&sig.sighash_buffer, pub_key, &sig.signature, crate::SigningHash::Sha256d).unwrap_or(false)
    }

    pub fn _verify(&self, pub_key: &PublicKey, sig: &SighashSignature, reverse_k: bool) -> bool {
        let digest = get_hash_digest(crate::SigningHash::Sha256d, &sig.sighash_buffer);
        let hashbuf = match reverse_k {
            true => digest.reverse().finalize_fixed(),
            false => digest.finalize_fixed(),
        };
        ECDSA::verify_hashbuf_impl(hashbuf, pub_key, &sig.signature).unwrap_or(false)
    }
}

impl Transaction {
    pub fn sign(&mut self, priv_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<SighashSignature, BSVErrors> {
        Transaction::sign_impl(self, priv_key, sighash, n_tx_in, unsigned_script, value)
    }

    pub fn sign_with_k(&mut self, priv_key: &PrivateKey, ephemeral_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<SighashSignature, BSVErrors> {
        Transaction::sign_with_k_impl(self, priv_key, ephemeral_key, sighash, n_tx_in, unsigned_script, value)
    }

    pub fn sighash_preimage(&mut self, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<Vec<u8>, BSVErrors> {
        Transaction::sighash_preimage_impl(self, n_tx_in, sighash, unsigned_script, value)
    }
}

pub struct SighashSignature {
    pub(crate) signature: Signature,
    pub(crate) sighash_type: SigHash,
    pub(crate) sighash_buffer: Vec<u8>,
}

impl SighashSignature {
    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(self.to_bytes_impl()?))
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut sig_bytes = self.signature.to_der_bytes();
        let sighash_u8 = self
            .sighash_type
            .to_u8()
            .ok_or_else(|| BSVErrors::FromSighash(format!("Cannot convert SigHash {:?} into u8", self.sighash_type)))?;

        sig_bytes.push(sighash_u8);
        Ok(sig_bytes)
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8], sighash_buffer: &[u8]) -> Result<Self, BSVErrors> {
        let der_bytes = if bytes.len() <= 72 { bytes } else { &bytes[..bytes.len() - 1] };
        let signature = Signature::from_der_impl(der_bytes)?;
        let sighash_type: SigHash = bytes
            .last()
            .cloned()
            .ok_or_else(|| BSVErrors::ToSighash("Could not convert last byte of signature to Sighash flag".into()))?
            .try_into()?;
        Ok(Self {
            sighash_type,
            signature,
            sighash_buffer: sighash_buffer.to_vec(),
        })
    }
}

impl SighashSignature {
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(constructor))]
    pub fn new(signature: &Signature, sighash_type: SigHash, sighash_buffer: &[u8]) -> SighashSignature {
        SighashSignature {
            signature: signature.clone(),
            sighash_type,
            sighash_buffer: sighash_buffer.to_vec(),
        }
    }
}

impl SighashSignature {
    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        self.to_hex_impl()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        self.to_bytes_impl()
    }

    pub fn from_bytes(bytes: &[u8], sighash_buffer: &[u8]) -> Result<SighashSignature, BSVErrors> {
        Self::from_bytes_impl(bytes, sighash_buffer)
    }
}

// #[cfg(all(feature = "wasm-bindgen-transaction"))]
// #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen)]
// impl SighashSignature {
//     #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
//     pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(self.to_hex_impl()?)
//     }

//     #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toBytes))]
//     pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//         Ok(self.to_bytes_impl()?)
//     }

//     #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
//     pub fn from_bytes(bytes: &[u8], sighash_buffer: &[u8]) -> Result<SighashSignature, wasm_bindgen::JsError> {
//         Ok(Self::from_bytes_impl(bytes, sighash_buffer)?)
//     }
// }
