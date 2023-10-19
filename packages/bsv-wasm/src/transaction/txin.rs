use bsv::TxIn as BSVTxIn;
use wasm_bindgen::prelude::*;

use crate::script::Script;

#[wasm_bindgen]
#[derive(Default)]
pub struct TxIn(pub(crate) BSVTxIn);

impl From<BSVTxIn> for TxIn {
    fn from(v: BSVTxIn) -> TxIn {
        TxIn(v)
    }
}

impl From<TxIn> for BSVTxIn {
    fn from(v: TxIn) -> BSVTxIn {
        v.0
    }
}

#[wasm_bindgen]
impl TxIn {
    #[wasm_bindgen(constructor)]
    pub fn new(prev_tx_id: &[u8], vout: u32, unlocking_script: &Script, sequence: Option<u32>) -> TxIn {
        TxIn(BSVTxIn::new(prev_tx_id, vout, &unlocking_script.0, sequence))
    }

    pub fn empty() -> TxIn {
        TxIn::default()
    }

    pub fn get_prev_tx_id(&self, little_endian: Option<bool>) -> Vec<u8> {
        self.0.get_prev_tx_id(little_endian)
    }

    pub fn get_prev_tx_id_hex(&self, little_endian: Option<bool>) -> String {
        self.0.get_prev_tx_id_hex(little_endian)
    }

    pub fn get_vout(&self) -> u32 {
        self.0.get_vout()
    }

    pub fn get_unlocking_script_size(&self) -> u64 {
        self.0.get_unlocking_script_size()
    }

    pub fn get_unlocking_script(&self) -> Script {
        Script(self.0.get_unlocking_script())
    }

    pub fn get_unlocking_script_hex(&self) -> String {
        self.0.get_unlocking_script_hex()
    }

    pub fn get_sequence(&self) -> u32 {
        self.0.get_sequence()
    }

    pub fn get_sequence_as_bytes(&self) -> Vec<u8> {
        self.0.get_sequence_as_bytes()
    }

    pub fn get_outpoint_bytes(&self, little_endian: Option<bool>) -> Vec<u8> {
        self.0.get_outpoint_bytes(little_endian)
    }

    pub fn get_outpoint_hex(&self, little_endian: Option<bool>) -> String {
        self.0.get_outpoint_hex(little_endian)
    }

    pub fn set_unlocking_script(&mut self, script: &Script) {
        self.0.set_unlocking_script(&script.0)
    }

    pub fn set_prev_tx_id(&mut self, txid: &[u8]) {
        self.0.set_prev_tx_id(txid)
    }

    pub fn set_vout(&mut self, vout: u32) {
        self.0.set_vout(vout);
    }

    pub fn set_sequence(&mut self, sequence: u32) {
        self.0.set_sequence(sequence);
    }

    pub fn set_satoshis(&mut self, satoshis: u64) {
        self.0.set_satoshis(satoshis);
    }

    pub fn get_satoshis(&self) -> Option<u64> {
        self.0.get_satoshis()
    }

    pub fn set_locking_script(&mut self, locking_script: &Script) {
        self.0.set_locking_script(&locking_script.0)
    }

    pub fn get_locking_script(&self) -> Option<Script> {
        self.0.get_locking_script().map(Script)
    }

    pub fn get_locking_script_bytes(&self) -> Option<Vec<u8>> {
        self.0.get_locking_script_bytes()
    }

    pub fn from_hex(hex_str: &str) -> Result<TxIn, wasm_bindgen::JsError> {
        Ok(TxIn(BSVTxIn::from_hex(hex_str)?))
    }

    pub fn to_json(&self) -> Result<JsValue, JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_json_string()?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.to_bytes()?)
    }

    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_hex()?)
    }

    pub fn from_outpoint_bytes(outpoint: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
        Ok(TxIn(BSVTxIn::from_outpoint_bytes(outpoint)?))
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
    pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
        Ok(TxIn(BSVTxIn::from_compact_bytes(compact_buffer)?))
    }

    /**
     * Deserialises the provided CBOR buffer to the TX+ format
     */
    pub fn from_compact_hex(compact_hex: String) -> Result<TxIn, wasm_bindgen::JsError> {
        Ok(TxIn(BSVTxIn::from_compact_hex(&compact_hex)?))
    }

    /// Concatenates ScriptSig and UnlockingScript into a single script.
    pub fn get_finalised_script(&self) -> Result<Script, JsError> {
        Ok(Script(self.0.get_finalised_script()?))
    }

    // Checks if input is a coinbase
    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase()
    }
}
