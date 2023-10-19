use bsv::AESAlgorithms as BSVAESAlgorithms;
use bsv::AES as BSVAES;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum AESAlgorithms {
    AES128_CBC,
    AES256_CBC,
    AES128_CTR,
    AES256_CTR,
}

impl From<AESAlgorithms> for BSVAESAlgorithms {
    fn from(item: AESAlgorithms) -> Self {
        match item {
            AESAlgorithms::AES128_CBC => BSVAESAlgorithms::AES128_CBC,
            AESAlgorithms::AES256_CBC => BSVAESAlgorithms::AES256_CBC,
            AESAlgorithms::AES128_CTR => BSVAESAlgorithms::AES128_CTR,
            AESAlgorithms::AES256_CTR => BSVAESAlgorithms::AES256_CTR,
        }
    }
}

#[wasm_bindgen]
pub struct AES;

#[wasm_bindgen]
impl AES {
    pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVAES::encrypt(key, iv, message, algo.into())?)
    }

    pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVAES::decrypt(key, iv, message, algo.into())?)
    }
}
