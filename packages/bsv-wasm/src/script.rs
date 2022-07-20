use bsv::Script;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Script)]
pub struct JsScript(Script);

/**
 * WASM Specific Functions
 */
#[wasm_bindgen]
impl JsScript {
    #[wasm_bindgen(js_name = toASMString)]
    pub fn to_asm_string(&self) -> String {
        Script::to_asm_string(&self.0)
    }

    #[wasm_bindgen(js_name = toExtendedASMString)]
    pub fn to_extended_asm_string(&self) -> String {
        Script::to_extended_asm_string(&self.0)
    }

    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: &str) -> Result<JsScript, wasm_bindgen::JsError> {
        Ok(JsScript(Script::from_hex(hex)?))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<JsScript, wasm_bindgen::JsError> {
        Ok(JsScript(Script::from_bytes(bytes)?))
    }

    #[wasm_bindgen(js_name = fromASMString)]
    pub fn from_asm_string(asm_string: &str) -> Result<JsScript, wasm_bindgen::JsError> {
        Ok(JsScript(Script::from_asm_string(asm_string)?))
    }

    #[wasm_bindgen(js_name = encodePushData)]
    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(Script::encode_pushdata(data_bytes)?)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    #[wasm_bindgen(js_name = getPushDataBytes)]
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(Script::get_pushdata_prefix_bytes(length)?)
    }

    #[wasm_bindgen(js_name = toScriptBits)]
    pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
        Ok(serde_wasm_bindgen::to_value(&self.0)?)
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        Script::script_bits_to_bytes(&self.0)
    }

    #[wasm_bindgen(js_name = getScriptLength)]
    pub fn get_script_length(&self) -> usize {
        self.to_bytes().len()
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    #[wasm_bindgen(js_name = removeCodeSeparators)]
    pub fn remove_codeseparators(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|x| *x != ScriptBit::OpCode(OpCodes::OP_CODESEPARATOR)).collect();
    }
}
