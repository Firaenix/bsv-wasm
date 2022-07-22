use crate::signature::Signature;
use bsv::{SigHash as BSVSigHash, SighashSignature as BSVSighashSignature};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SighashSignature(pub(crate) BSVSighashSignature);

#[wasm_bindgen]
pub struct SigHash(pub(crate) BSVSigHash);

#[wasm_bindgen(constructor)]
impl SighashSignature {
    #[wasm_bindgen(constructor)]
    pub fn new(signature: &Signature, sighash_type: SigHash, sighash_buffer: &[u8]) -> SighashSignature {
        SighashSignature(BSVSighashSignature::new(&signature.0, sighash_type.0, sighash_buffer))
    }

    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(BSVSighashSignature::to_hex(&self.0)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVSighashSignature::to_bytes(&self.0)?)
    }

    pub fn from_bytes(bytes: &[u8], sighash_buffer: &[u8]) -> Result<SighashSignature, wasm_bindgen::JsError> {
        Ok(SighashSignature(BSVSighashSignature::from_bytes(bytes, sighash_buffer)?))
    }
}
