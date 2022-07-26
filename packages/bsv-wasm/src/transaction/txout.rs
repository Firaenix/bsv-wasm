use bsv::TxOut as BSVTxOut;
use wasm_bindgen::prelude::*;

use crate::script::Script;

#[wasm_bindgen]
pub struct TxOut(pub(crate) BSVTxOut);

impl From<BSVTxOut> for TxOut {
    fn from(v: BSVTxOut) -> TxOut {
        TxOut(v)
    }
}

#[wasm_bindgen]
impl TxOut {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u64, script_pub_key: &Script) -> TxOut {
        TxOut(BSVTxOut::new(value, &script_pub_key.0))
    }

    pub fn get_satoshis(&self) -> u64 {
        self.0.get_satoshis()
    }

    pub fn get_satoshis_as_bytes(&self) -> Vec<u8> {
        self.0.get_satoshis_as_bytes()
    }

    pub fn get_script_pub_key_size(&self) -> usize {
        self.0.get_script_pub_key_size()
    }

    pub fn get_script_pub_key(&self) -> Script {
        Script(self.0.get_script_pub_key())
    }

    pub fn get_script_pub_key_hex(&self) -> String {
        self.0.get_script_pub_key_hex()
    }

    pub fn from_hex(hex_str: &str) -> Result<TxOut, wasm_bindgen::JsError> {
        Ok(TxOut(BSVTxOut::from_hex(hex_str)?))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(self.0.to_bytes()?)
    }

    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_hex()?)
    }

    pub fn to_json(&self) -> Result<JsValue, JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_json_string()?)
    }
}
