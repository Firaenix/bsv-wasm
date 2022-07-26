use bsv::Script as BSVScript;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Script(pub(crate) BSVScript);

impl From<BSVScript> for Script {
    fn from(v: BSVScript) -> Script {
        Script(v)
    }
}

/**
 * WASM Specific Functions
 */
#[wasm_bindgen]
impl Script {
    pub fn to_asm_string(&self) -> String {
        BSVScript::to_asm_string(&self.0)
    }

    pub fn to_extended_asm_string(&self) -> String {
        BSVScript::to_extended_asm_string(&self.0)
    }

    pub fn from_hex(hex: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(BSVScript::from_hex(hex)?))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(BSVScript::from_bytes(bytes)?))
    }

    pub fn from_asm_string(asm_string: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(BSVScript::from_asm_string(asm_string)?))
    }

    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVScript::encode_pushdata(data_bytes)?)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVScript::get_pushdata_prefix_bytes(length)?)
    }

    pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        BSVScript::to_bytes(&self.0)
    }

    pub fn get_script_length(&self) -> usize {
        BSVScript::get_script_length(&self.0)
    }

    pub fn to_hex(&self) -> String {
        BSVScript::to_hex(&self.0)
    }

    pub fn remove_codeseparators(&mut self) {
        BSVScript::remove_codeseparators(&mut self.0)
    }
}
