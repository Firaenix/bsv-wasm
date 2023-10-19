use bsv::Transaction as BSVTransaction;
use wasm_bindgen::prelude::*;

mod txin;
mod txout;
pub use txin::*;
pub use txout::*;

use crate::{
    keypair::private_key::PrivateKey,
    script::Script,
    sighash::{SigHash, SighashSignature},
    PublicKey,
};

#[wasm_bindgen]
pub struct Transaction(pub(crate) BSVTransaction);

impl From<BSVTransaction> for Transaction {
    fn from(v: BSVTransaction) -> Transaction {
        Transaction(v)
    }
}

impl From<Transaction> for BSVTransaction {
    fn from(v: Transaction) -> BSVTransaction {
        v.0
    }
}

#[wasm_bindgen]
impl Transaction {
    pub fn get_version(&self) -> u32 {
        self.0.get_version()
    }

    pub fn get_ninputs(&self) -> usize {
        self.0.get_ninputs()
    }

    pub fn get_noutputs(&self) -> usize {
        self.0.get_noutputs()
    }

    pub fn get_input(&self, index: usize) -> Option<TxIn> {
        self.0.get_input(index).map(TxIn)
    }

    pub fn get_output(&self, index: usize) -> Option<TxOut> {
        self.0.get_output(index).map(TxOut)
    }

    pub fn get_n_locktime(&self) -> u32 {
        self.0.get_n_locktime()
    }

    pub fn get_n_locktime_as_bytes(&self) -> Vec<u8> {
        self.0.get_n_locktime_as_bytes()
    }

    /**
     * Creates a new empty transaction where you need to add inputs and outputs
     * Transaction.add_input(TxIn) and Transaction.add_output(TxOut)
     */
    #[wasm_bindgen(constructor)]
    pub fn new(version: u32, n_locktime: u32) -> Transaction {
        Transaction(BSVTransaction::new(version, n_locktime))
    }

    pub fn set_version(&mut self, version: u32) -> Transaction {
        Transaction(self.0.set_version(version))
    }

    pub fn set_nlocktime(&mut self, n_locktime: u32) -> Transaction {
        Transaction(self.0.set_nlocktime(n_locktime))
    }

    pub fn add_input(&mut self, input: &TxIn) {
        self.0.add_input(&input.0)
    }

    pub fn prepend_input(&mut self, input: &TxIn) {
        self.0.prepend_input(&input.0)
    }

    pub fn insert_input(&mut self, index: usize, input: &TxIn) {
        self.0.insert_input(index, &input.0)
    }

    pub fn add_output(&mut self, output: &TxOut) {
        self.0.add_output(&output.0)
    }

    pub fn prepend_output(&mut self, output: &TxOut) {
        self.0.prepend_output(&output.0)
    }

    pub fn insert_output(&mut self, index: usize, output: &TxOut) {
        self.0.insert_output(index, &output.0)
    }

    pub fn set_input(&mut self, index: usize, input: &TxIn) {
        self.0.set_input(index, &input.0);
    }

    pub fn set_output(&mut self, index: usize, output: &TxOut) {
        self.0.set_output(index, &output.0);
    }

    pub fn is_coinbase_impl(&self) -> bool {
        match (self.get_ninputs(), self.get_input(0)) {
            (1, Some(x)) => x.0.is_coinbase(),
            _ => false,
        }
    }

    /**
     * XT Method:
     * Returns the combined sum of all input satoshis.
     * If any of the inputs dont have satoshis defined, this returns None or null
     */
    pub fn satoshis_in(&self) -> Option<u64> {
        self.0.satoshis_in()
    }

    /**
     * Returns the combined sum of all output satoshis.
     */
    pub fn satoshis_out(&self) -> u64 {
        self.0.satoshis_out()
    }

    pub fn from_hex(hex_str: &str) -> Result<Transaction, wasm_bindgen::JsError> {
        Ok(Transaction(BSVTransaction::from_hex(hex_str)?))
    }

    pub fn from_bytes(tx_bytes: &[u8]) -> Result<Transaction, wasm_bindgen::JsError> {
        Ok(Transaction(BSVTransaction::from_bytes(tx_bytes)?))
    }

    pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_json_string()?)
    }

    pub fn from_json_string(json_string: &str) -> Result<Transaction, wasm_bindgen::JsError> {
        Ok(Transaction(BSVTransaction::from_json_string(json_string)?))
    }

    pub fn to_json(&self) -> Result<JsValue, wasm_bindgen::JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.to_bytes()?)
    }

    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_hex()?)
    }

    /**
     * Get size of current serialised Transaction object
     */
    pub fn get_size(&self) -> Result<usize, wasm_bindgen::JsError> {
        Ok(self.0.get_size()?)
    }

    /**
     * Adds an array of TxIn's to the transaction
     * @param {TxIn[]} tx_ins
     */
    pub fn add_inputs(&mut self, tx_ins: Vec<JsValue>) {
        let js_value = &*tx_ins.to_vec();

        for elem in js_value {
            let input = serde_wasm_bindgen::from_value(elem.clone()).unwrap();

            self.0.add_input(&input);
        }
    }

    /**
     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
     *
     * @returns {Uint8Array[]} outpoint_array
     */
    pub fn get_outpoints(&mut self) -> Result<JsValue, wasm_bindgen::JsError> {
        let outpoints = self.0.get_outpoints();
        Ok(serde_wasm_bindgen::to_value(&outpoints)?)
    }

    /**
     * Adds an array of TxOuts to the transaction
     * @param {TxOut[]} tx_outs
     */
    pub fn add_outputs(&mut self, tx_outs: Vec<JsValue>) {
        let js_value = &*tx_outs.to_vec();

        for elem in js_value {
            let output = serde_wasm_bindgen::from_value(elem.clone()).unwrap();

            self.0.add_output(&output);
        }
    }

    /**
     * Gets the ID of the current transaction as a hex string.
     */
    pub fn get_id_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.get_id_hex()?)
    }

    /**
     * Gets the ID of the current transaction as a Uint8Array.
     */
    pub fn get_id_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.get_id_bytes()?)
    }

    /**
     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
     */
    pub fn to_compact_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.to_compact_bytes()?)
    }

    pub fn to_compact_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_compact_hex()?)
    }

    /**
     * Deserialises the provided CBOR buffer to the TX+ format
     */
    pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<Transaction, wasm_bindgen::JsError> {
        Ok(Transaction(BSVTransaction::from_compact_bytes(compact_buffer)?))
    }

    /**
     * Deserialises the provided CBOR buffer to the TX+ format
     */
    pub fn from_compact_hex(compact_hex: String) -> Result<Transaction, wasm_bindgen::JsError> {
        Ok(Transaction(BSVTransaction::from_compact_hex(&compact_hex)?))
    }

    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }

    pub fn sign(&mut self, priv_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<SighashSignature, wasm_bindgen::JsError> {
        Ok(SighashSignature(self.0.sign(&priv_key.0, sighash.into(), n_tx_in, &unsigned_script.0, value)?))
    }

    pub fn sign_with_k(
        &mut self,
        priv_key: &PrivateKey,
        ephemeral_key: &PrivateKey,
        sighash: SigHash,
        n_tx_in: usize,
        unsigned_script: &Script,
        value: u64,
    ) -> Result<SighashSignature, wasm_bindgen::JsError> {
        Ok(SighashSignature(self.0.sign_with_k(
            &priv_key.0,
            &ephemeral_key.0,
            sighash.into(),
            n_tx_in,
            &unsigned_script.0,
            value,
        )?))
    }

    pub fn sighash_preimage(&mut self, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.sighash_preimage(sighash.into(), n_tx_in, &unsigned_script.0, value)?)
    }

    pub fn verify(&self, pub_key: &PublicKey, sig: &SighashSignature) -> bool {
        self.0.verify(&pub_key.0, &sig.0)
    }
}
