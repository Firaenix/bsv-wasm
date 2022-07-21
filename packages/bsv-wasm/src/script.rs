use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Script)]
pub struct Script(bsv::Script);

/**
 * WASM Specific Functions
 */
#[wasm_bindgen]
impl Script {
    #[wasm_bindgen(js_name = toASMString)]
    pub fn to_asm_string(&self) -> String {
        bsv::Script::to_asm_string(&self.0)
    }

    #[wasm_bindgen(js_name = toExtendedASMString)]
    pub fn to_extended_asm_string(&self) -> String {
        bsv::Script::to_extended_asm_string(&self.0)
    }

    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(bsv::Script::from_hex(hex)?))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(bsv::Script::from_bytes(bytes)?))
    }

    #[wasm_bindgen(js_name = fromASMString)]
    pub fn from_asm_string(asm_string: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(bsv::Script::from_asm_string(asm_string)?))
    }

    #[wasm_bindgen(js_name = encodePushData)]
    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(bsv::Script::encode_pushdata(data_bytes)?)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    #[wasm_bindgen(js_name = getPushDataBytes)]
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(bsv::Script::get_pushdata_prefix_bytes(length)?)
    }

    #[wasm_bindgen(js_name = toScriptBits)]
    pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        bsv::Script::to_bytes(&self.0)
    }

    #[wasm_bindgen(js_name = getScriptLength)]
    pub fn get_script_length(&self) -> usize {
        bsv::Script::get_script_length(&self.0)
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        bsv::Script::to_hex(&self.0)
    }

    #[wasm_bindgen(js_name = removeCodeSeparators)]
    pub fn remove_codeseparators(&mut self) {
        bsv::Script::remove_codeseparators(&mut self.0)
    }
}
