use crate::signature::Signature;
use bsv::SighashSignature as BSVSighashSignature;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SighashSignature(pub(crate) BSVSighashSignature);

impl From<BSVSighashSignature> for SighashSignature {
    fn from(v: BSVSighashSignature) -> SighashSignature {
        SighashSignature(v)
    }
}

impl From<SighashSignature> for BSVSighashSignature {
    fn from(v: SighashSignature) -> BSVSighashSignature {
        v.0
    }
}

#[wasm_bindgen]
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

impl Into<bsv::SigHash> for SigHash {
    fn into(self) -> bsv::SigHash {
        bsv::SigHash::try_from(self as u8).unwrap()
    }
}

#[wasm_bindgen(constructor)]
impl SighashSignature {
    #[wasm_bindgen(constructor)]
    pub fn new(signature: &Signature, sighash_type: SigHash, sighash_buffer: &[u8]) -> SighashSignature {
        SighashSignature(BSVSighashSignature::new(&signature.0, sighash_type.into(), sighash_buffer))
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
