use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use crate::BSVErrors;
use crate::Script;
use crate::{Hash, VarInt};
use byteorder::*;
use serde::{Deserialize, Serialize};
use thiserror::*;
use wasm_bindgen::{prelude::*, throw_str, JsValue};

mod match_criteria;
mod sighash;
mod txin;
mod txout;

pub use match_criteria::*;
pub use sighash::*;
pub use txin::*;
pub use txout::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Transaction {
    pub(super) version: u32,
    pub(super) inputs: Vec<TxIn>,
    pub(super) outputs: Vec<TxOut>,
    pub(super) n_locktime: u32,
    #[serde(skip)]
    pub(super) hash_cache: HashCache,
}

impl Transaction {
    pub(crate) fn new_impl(version: u32, inputs: Vec<TxIn>, outputs: Vec<TxOut>, n_locktime: u32) -> Transaction {
        Transaction {
            version,
            inputs,
            outputs,
            n_locktime,
            hash_cache: HashCache::new(),
        }
    }

    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<Transaction, BSVErrors> {
        let tx_bytes = hex::decode(hex_str)?;

        Transaction::from_bytes_impl(&tx_bytes)
    }

    pub(crate) fn from_bytes_impl(tx_bytes: &[u8]) -> Result<Transaction, BSVErrors> {
        let mut cursor = Cursor::new(tx_bytes.to_vec());

        // Version - 4 bytes
        let version = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTransaction("version".to_string(), e)),
        };

        // In Counter - 1-9 tx_bytes
        let n_inputs = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTransaction("n_inputs".to_string(), e)),
        };

        let mut inputs: Vec<TxIn> = Vec::new();
        // List of Inputs
        for _ in 0..n_inputs {
            let tx_in = TxIn::read_in(&mut cursor)?;
            inputs.push(tx_in);
        }

        // Out Counter - 1-9 bytes
        let n_outputs = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTransaction("n_outputs".to_string(), e)),
        };

        // List of  Outputs
        let mut outputs: Vec<TxOut> = Vec::new();
        for _ in 0..n_outputs {
            let tx_out = TxOut::read_in(&mut cursor)?;
            outputs.push(tx_out);
        }

        // nLocktime - 4 bytes
        let n_locktime = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTransaction("n_locktime".to_string(), e)),
        };

        Ok(Transaction {
            version,
            inputs,
            outputs,
            n_locktime,
            hash_cache: HashCache::new(),
        })
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer = Vec::new();

        // Version - 4 bytes
        if let Err(e) = buffer.write_u32::<LittleEndian>(self.version) {
            return Err(BSVErrors::SerialiseTransaction("version".to_string(), e));
        }

        // In Counter - 1-9 tx_bytes
        if let Err(e) = buffer.write_varint(self.get_ninputs() as u64) {
            return Err(BSVErrors::SerialiseTransaction("n_inputs".to_string(), e));
        }

        // Inputs
        for i in 0..self.get_ninputs() {
            let input = &self.inputs[i];
            let input_bytes = input.to_bytes_impl()?;

            if let Err(e) = buffer.write_all(&input_bytes) {
                return Err(BSVErrors::SerialiseTransaction(format!("input {}", i), e));
            }
        }

        // Out Counter - 1-9 tx_bytes
        if let Err(e) = buffer.write_varint(self.get_noutputs() as u64) {
            return Err(BSVErrors::SerialiseTransaction("n_outputs".to_string(), e));
        }

        // Outputs
        for i in 0..self.get_noutputs() {
            let output = &self.outputs[i as usize];
            let output_bytes = output.to_bytes_impl()?;

            if let Err(e) = buffer.write_all(&output_bytes) {
                return Err(BSVErrors::SerialiseTransaction(format!("output {}", i), e));
            }
        }

        // nLocktime - 4 bytes
        if let Err(e) = buffer.write_u32::<LittleEndian>(self.n_locktime) {
            return Err(BSVErrors::SerialiseTransaction("n_locktime".to_string(), e));
        }

        // Write out bytes
        Ok(buffer)
    }

    pub(crate) fn get_size_impl(&self) -> Result<usize, BSVErrors> {
        let tx_bytes = self.to_bytes_impl()?;
        Ok(tx_bytes.len())
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(&self.to_bytes_impl()?))
    }

    pub(crate) fn to_json_string_impl(&self) -> Result<String, BSVErrors> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    /**
     * Gets the ID of the current transaction.
     * Returns the SHA256d Hash of the current transaction.
     *
     * Txid is the reverse of the hash result.
     */
    pub(crate) fn get_id_impl(&self) -> Result<Hash, BSVErrors> {
        let tx_bytes = self.to_bytes_impl()?;
        let mut hash = Hash::sha_256d(&tx_bytes);
        hash.0.reverse();

        Ok(hash)
    }

    pub fn to_compact_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let buf = flexbuffers::to_vec(self)?;
        Ok(buf)
    }

    pub fn from_compact_bytes_impl(buffer: &[u8]) -> Result<Transaction, BSVErrors> {
        let transaction = flexbuffers::from_slice(buffer)?;
        Ok(transaction)
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
    pub fn get_ninputs(&self) -> usize {
        self.inputs.len()
    }

    #[wasm_bindgen(js_name = getOutputsCount)]
    pub fn get_noutputs(&self) -> usize {
        self.outputs.len()
    }

    #[wasm_bindgen(js_name = getInput)]
    pub fn get_input(&self, index: usize) -> Option<TxIn> {
        self.inputs.get(index).cloned()
    }

    #[wasm_bindgen(js_name = getOutput)]
    pub fn get_output(&self, index: usize) -> Option<TxOut> {
        self.outputs.get(index).cloned()
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
    pub fn add_input(&mut self, input: &TxIn) {
        self.inputs.push(input.clone());
        // Transaction has been changed, need to recalculate inputs hashes
        self.hash_cache.hash_inputs = None;
        self.hash_cache.hash_sequence = None;
    }

    #[wasm_bindgen(js_name = addOutput)]
    pub fn add_output(&mut self, output: &TxOut) {
        self.outputs.push(output.clone());
        // Transaction has been changed, need to recalculate outputs hashes
        self.hash_cache.hash_outputs = None;
    }

    #[wasm_bindgen(js_name = setInput)]
    pub fn set_input(&mut self, index: usize, input: &TxIn) {
        self.inputs[index] = input.clone();
    }

    #[wasm_bindgen(js_name = setOutput)]
    pub fn set_output(&mut self, index: usize, output: &TxOut) {
        self.outputs[index] = output.clone();
    }

    fn is_matching_output(txout: &TxOut, criteria: &MatchCriteria) -> bool {
        // If script is specified and doesnt match
        if matches!(&criteria.script, Some(crit_script) if crit_script != &txout.script_pub_key) {
            return false;
        }
        // If exact_value is specified and doesnt match
        if criteria.exact_value.is_some() && criteria.exact_value != Some(txout.value) {
            return false;
        }

        // If min_value is specified and value is less than min value
        if criteria.min_value.is_some() && criteria.min_value > Some(txout.value) {
            return false;
        }

        // If min_value is specified and value is greater than max value
        if criteria.max_value.is_some() && criteria.max_value < Some(txout.value) {
            return false;
        }

        true
    }

    /**
     * Returns the first output index that matches the given parameters, returns None or null if not found.
     */
    #[wasm_bindgen(js_name = matchOutput)]
    pub fn match_output(&self, criteria: &MatchCriteria) -> Option<usize> {
        self.outputs.iter().enumerate().find_map(|(i, txout)| match Transaction::is_matching_output(txout, criteria) {
            true => Some(i),
            false => None,
        })
    }

    /**
     * Returns a list of outputs indexes that match the given parameters
     */
    #[wasm_bindgen(js_name = matchOutputs)]
    pub fn match_outputs(&self, criteria: &MatchCriteria) -> Vec<usize> {
        let matches = self
            .outputs
            .iter()
            .enumerate()
            .filter_map(|(i, txout)| match Transaction::is_matching_output(txout, criteria) {
                true => Some(i),
                false => None,
            })
            .collect();

        matches
    }

    fn is_matching_input(txin: &TxIn, criteria: &MatchCriteria) -> bool {
        // If script is specified and doesnt match
        if matches!(&criteria.script, Some(crit_script) if crit_script != &txin.script_sig) {
            return false;
        }

        // If exact_value is specified and doesnt match
        if criteria.exact_value.is_some() && criteria.exact_value != txin.satoshis {
            return false;
        }

        // If min_value is specified and value is less than min value
        if criteria.min_value.is_some() && criteria.min_value > txin.satoshis {
            return false;
        }

        // If min_value is specified and value is greater than max value
        if criteria.max_value.is_some() && criteria.max_value < txin.satoshis {
            return false;
        }

        true
    }

    /**
     * Returns the first input index that matches the given parameters, returns None or null if not found.
     */
    #[wasm_bindgen(js_name = matchInput)]
    pub fn match_input(&self, criteria: &MatchCriteria) -> Option<usize> {
        self.inputs.iter().enumerate().find_map(|(i, txin)| match Transaction::is_matching_input(txin, criteria) {
            true => Some(i),
            false => None,
        })
    }

    /**
     * Returns a list of input indexes that match the given parameters
     */
    #[wasm_bindgen(js_name = matchInputs)]
    pub fn match_inputs(&self, criteria: &MatchCriteria) -> Vec<usize> {
        let matches = self
            .inputs
            .iter()
            .enumerate()
            .filter_map(|(i, txin)| match Transaction::is_matching_input(txin, criteria) {
                true => Some(i),
                false => None,
            })
            .collect();

        matches
    }

    /**
     * XT Method:
     * Returns the combined sum of all input satoshis.
     * If any of the inputs dont have satoshis defined, this returns None or null
     */
    pub fn total_input_satoshis(&self) -> Option<u64> {
        self.inputs.iter().map(|x| x.satoshis).reduce(|a, b| {
            if a == None || b == None {
                return None;
            }

            Some(a.unwrap() + b.unwrap())
        })?
    }

    /**
     * Returns the combined sum of all output satoshis.
     */
    pub fn total_output_satoshis(&self) -> u64 {
        self.outputs.iter().map(|x| x.value).sum()
    }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Transaction {
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex_str: &str) -> Result<Transaction, JsValue> {
        return match Transaction::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        };
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(tx_bytes: &[u8]) -> Result<Transaction, JsValue> {
        return match Transaction::from_bytes_impl(tx_bytes) {
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
     * Get size of current serialised Transaction object
     */
    #[wasm_bindgen(js_name = getSize)]
    pub fn get_size(&self) -> Result<usize, JsValue> {
        match Transaction::get_size_impl(&self) {
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

    /**
     * Gets the ID of the current transaction as a hex string.
     */
    #[wasm_bindgen(js_name = getIdHex)]
    pub fn get_id_hex(&self) -> Result<String, JsValue> {
        match self.get_id_impl() {
            Ok(v) => Ok(v.to_hex()),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    /**
     * Gets the ID of the current transaction as a Uint8Array.
     */
    #[wasm_bindgen(js_name = getIdBytes)]
    pub fn get_id_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match self.get_id_impl() {
            Ok(v) => Ok(v.to_bytes()),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl Transaction {
    /**
     * Gets the ID of the current transaction as a hex string.
     */
    pub fn get_id_hex(&self) -> Result<String, BSVErrors> {
        Ok(self.get_id_impl()?.to_hex())
    }

    /**
     * Gets the ID of the current transaction as a Vec<u8>.
     */
    pub fn get_id_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        Ok(self.get_id_impl()?.to_bytes())
    }

    /**
     * Get size of current serialised Transaction object
     */
    pub fn get_size(&self) -> Result<usize, BSVErrors> {
        self.get_size_impl()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex(hex_str: &str) -> Result<Transaction, BSVErrors> {
        Transaction::from_hex_impl(hex_str)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_bytes(tx_bytes: &[u8]) -> Result<Transaction, BSVErrors> {
        Transaction::from_bytes_impl(tx_bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json_string(&self) -> Result<String, BSVErrors> {
        Transaction::to_json_string_impl(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json(&self) -> Result<serde_json::Value, BSVErrors> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        Transaction::to_bytes_impl(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        Transaction::to_hex_impl(self)
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
