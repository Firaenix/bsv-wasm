use bitcoin_hashes::{Hash as BitcoinHash};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Hash {

}

#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = sha256d)]
  pub fn sha_256d(input: Vec<u8>) -> Vec<u8> {
    bitcoin_hashes::sha256d::Hash::hash(&input).to_vec()
  } 
  
  #[wasm_bindgen(js_name = sha256)]
  pub fn sha_256(input: Vec<u8>) ->  Vec<u8> {
    bitcoin_hashes::sha256::Hash::hash(&input).to_vec()
  } 
  
  
  #[wasm_bindgen(js_name = sha1)]
  pub fn sha_1(input: Vec<u8>) ->  Vec<u8> {
    bitcoin_hashes::sha1::Hash::hash(&input).to_vec()
  } 
  
  #[wasm_bindgen(js_name = ripemd160)]
  pub fn ripemd_160(input: Vec<u8>) ->  Vec<u8> {
    bitcoin_hashes::ripemd160::Hash::hash(&input).to_vec()
  } 
  
  #[wasm_bindgen(js_name = hash160)]
  pub fn hash_160(input: Vec<u8>) ->  Vec<u8> {
    bitcoin_hashes::hash160::Hash::hash(&input).to_vec()
  } 
  
  #[wasm_bindgen(js_name = sha512)]
  pub fn sha_512(input: Vec<u8>) ->  Vec<u8> {
    bitcoin_hashes::sha512::Hash::hash(&input).to_vec()
  }
}