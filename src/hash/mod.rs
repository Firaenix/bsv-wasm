use bitcoin_hashes::{Hash as BitcoinHash, hex::ToHex};
use wasm_bindgen::prelude::*;
use serde::*;
use crate::utils::{from_hex, to_hex};

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash(
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] 
  pub(crate) Vec<u8>
);

/**
 * Serialisation Functions
 */
#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = toBuffer)]
  pub fn to_bytes(&self) -> Vec<u8> {
    self.0.clone()
  } 

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
    self.0.to_hex()
  } 
}

/**
 * Hash Functions
 */
#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = sha256d)]
  pub fn sha_256d(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::sha256d::Hash::hash(input).to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha256)]
  pub fn sha_256(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::sha256::Hash::hash(input).to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha1)]
  pub fn sha_1(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::sha1::Hash::hash(input).to_vec())
  } 
  
  #[wasm_bindgen(js_name = ripemd160)]
  pub fn ripemd_160(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::ripemd160::Hash::hash(input).to_vec())
  } 
  
  #[wasm_bindgen(js_name = hash160)]
  pub fn hash_160(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::hash160::Hash::hash(input).to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha512)]
  pub fn sha_512(input: &[u8]) -> Self {
    Hash(bitcoin_hashes::sha512::Hash::hash(input).to_vec())
  }
}