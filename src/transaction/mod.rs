use std::io::Cursor;
use std::io::Write;

use crate::BSVErrors;
use crate::Hash;
use crate::VarIntReader;
use crate::VarIntWriter;
use byteorder::*;
use serde::{Deserialize, Serialize};

mod match_criteria;
mod sighash;
mod txin;
mod txout;

pub use match_criteria::*;
pub use sighash::*;
pub use txin::*;
pub use txout::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        for i in 0..n_outputs {
            let tx_out = match TxOut::read_in(&mut cursor) {
                Ok(v) => v,
                Err(e) => return Err(BSVErrors::DeserialiseScript(format!("TxOut: {} {}", i, e))),
            };
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

    /**
     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
     */
    pub(crate) fn to_compact_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer = vec![];
        ciborium::ser::into_writer(&self, &mut buffer)?;
        Ok(buffer)
    }

    /**
     * Deserialises the provided buffer to the TX+ format
     */
    pub(crate) fn from_compact_bytes_impl(compact_buffer: &[u8]) -> Result<Self, BSVErrors> {
        let tx = ciborium::de::from_reader(compact_buffer)?;
        Ok(tx)
    }

    pub(crate) fn get_size_impl(&self) -> Result<usize, BSVErrors> {
        let tx_bytes = self.to_bytes_impl()?;
        Ok(tx_bytes.len())
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(&self.to_bytes_impl()?))
    }

    pub(crate) fn to_compact_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(&self.to_compact_bytes_impl()?))
    }

    pub(crate) fn to_json_string_impl(&self) -> Result<String, BSVErrors> {
        let json = serde_json::to_string(self)?;
        Ok(json)
    }

    pub(crate) fn from_json_string_impl(json_string: &str) -> Result<Transaction, BSVErrors> {
        Ok(serde_json::from_str(json_string)?)
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

    /**
     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
     *
     * Transaction.get_outpoints()
     */
    pub(crate) fn get_outpoints_impl(&self) -> Vec<Vec<u8>> {
        self.inputs
            .iter()
            .map(|x| {
                let mut outpoint: Vec<u8> = vec![];

                outpoint.extend(x.prev_tx_id.clone());
                outpoint.reverse();
                outpoint.extend(x.vout.to_le_bytes());

                outpoint
            })
            .collect()
    }
}

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
impl Transaction {
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getVersion))]
    pub fn get_version(&self) -> u32 {
        self.version
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getInputsCount))]
    pub fn get_ninputs(&self) -> usize {
        self.inputs.len()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getOutputsCount))]
    pub fn get_noutputs(&self) -> usize {
        self.outputs.len()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getInput))]
    pub fn get_input(&self, index: usize) -> Option<TxIn> {
        self.inputs.get(index).cloned()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getOutput))]
    pub fn get_output(&self, index: usize) -> Option<TxOut> {
        self.outputs.get(index).cloned()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getNLocktime))]
    pub fn get_n_locktime(&self) -> u32 {
        self.n_locktime
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getNLocktimeAsBytes))]
    pub fn get_n_locktime_as_bytes(&self) -> Vec<u8> {
        self.n_locktime.to_be_bytes().to_vec()
    }

    /**
     * Creates a new empty transaction where you need to add inputs and outputs
     * Transaction.add_input(TxIn) and Transaction.add_output(TxOut)
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(constructor))]
    pub fn new(version: u32, n_locktime: u32) -> Transaction {
        Transaction::new_impl(version, vec![], vec![], n_locktime)
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen)]
    pub fn default() -> Transaction {
        Transaction::new_impl(2, vec![], vec![], 0)
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setVersion))]
    pub fn set_version(&mut self, version: u32) -> Transaction {
        self.version = version;
        self.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setNLocktime))]
    pub fn set_nlocktime(&mut self, n_locktime: u32) -> Transaction {
        self.n_locktime = n_locktime;
        self.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = addInput))]
    pub fn add_input(&mut self, input: &TxIn) {
        self.inputs.push(input.clone());
        // Transaction has been changed, need to recalculate inputs hashes
        self.hash_cache.hash_inputs = None;
        self.hash_cache.hash_sequence = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = prependInput))]
    pub fn prepend_input(&mut self, input: &TxIn) {
        self.inputs.insert(0, input.clone());
        // Transaction has been changed, need to recalculate inputs hashes
        self.hash_cache.hash_inputs = None;
        self.hash_cache.hash_sequence = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = insertInput))]
    pub fn insert_input(&mut self, index: usize, input: &TxIn) {
        self.inputs.insert(index, input.clone());
        // Transaction has been changed, need to recalculate inputs hashes
        self.hash_cache.hash_inputs = None;
        self.hash_cache.hash_sequence = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = addOutput))]
    pub fn add_output(&mut self, output: &TxOut) {
        self.outputs.push(output.clone());
        // Transaction has been changed, need to recalculate outputs hashes
        self.hash_cache.hash_outputs = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = prependOutput))]
    pub fn prepend_output(&mut self, output: &TxOut) {
        self.outputs.insert(0, output.clone());
        // Transaction has been changed, need to recalculate outputs hashes
        self.hash_cache.hash_outputs = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = insertOutput))]
    pub fn insert_output(&mut self, index: usize, output: &TxOut) {
        self.outputs.insert(index, output.clone());
        // Transaction has been changed, need to recalculate outputs hashes
        self.hash_cache.hash_outputs = None;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setInput))]
    pub fn set_input(&mut self, index: usize, input: &TxIn) {
        self.inputs[index] = input.clone();
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setOutput))]
    pub fn set_output(&mut self, index: usize, output: &TxOut) {
        self.outputs[index] = output.clone();
    }

    pub fn is_coinbase_impl(&self) -> bool {
        match (self.get_ninputs(), self.get_input(0)) {
            (1, Some(x)) => x.is_coinbase_impl(),
            _ => false,
        }
    }

    /**
     * XT Method:
     * Returns the combined sum of all input satoshis.
     * If any of the inputs dont have satoshis defined, this returns None or null
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = satoshisIn))]
    pub fn satoshis_in(&self) -> Option<u64> {
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
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = satoshisOut))]
    pub fn satoshis_out(&self) -> u64 {
        self.outputs.iter().map(|x| x.value).sum()
    }
}

/**
 * Native Specific Functions
 */
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

    pub fn from_hex(hex_str: &str) -> Result<Transaction, BSVErrors> {
        Transaction::from_hex_impl(hex_str)
    }

    pub fn from_bytes(tx_bytes: &[u8]) -> Result<Transaction, BSVErrors> {
        Transaction::from_bytes_impl(tx_bytes)
    }

    pub fn to_json_string(&self) -> Result<String, BSVErrors> {
        Transaction::to_json_string_impl(self)
    }

    pub fn from_json_string(json_string: &str) -> Result<Transaction, BSVErrors> {
        Transaction::from_json_string_impl(json_string)
    }

    pub fn to_json(&self) -> Result<serde_json::Value, BSVErrors> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        Transaction::to_bytes_impl(self)
    }

    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        Transaction::to_hex_impl(self)
    }

    pub fn to_compact_hex(&self) -> Result<String, BSVErrors> {
        Transaction::to_compact_hex_impl(self)
    }

    pub fn add_inputs(&mut self, tx_ins: Vec<TxIn>) {
        for txin in tx_ins {
            self.add_input(&txin);
        }
    }

    pub fn add_outputs(&mut self, tx_outs: Vec<TxOut>) {
        for txout in tx_outs {
            self.add_output(&txout);
        }
    }

    pub fn get_outpoints(&mut self) -> Vec<Vec<u8>> {
        self.get_outpoints_impl()
    }

    /**
     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + XT
     */
    pub fn to_compact_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        self.to_compact_bytes_impl()
    }

    /**
     * Deserialises the provided CBOR buffer to the XT format
     */
    pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<Self, BSVErrors> {
        Transaction::from_compact_bytes_impl(compact_buffer)
    }

    pub fn from_compact_hex(compact_hex: &str) -> Result<Self, BSVErrors> {
        Transaction::from_compact_bytes_impl(&hex::decode(compact_hex)?)
    }

    pub fn is_coinbase(&self) -> bool {
        self.is_coinbase_impl()
    }
}
