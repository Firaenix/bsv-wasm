use bitcoin_hashes::{Hash as BitcoinHash, HashEngine, Hmac, HmacEngine, hash160, hex::ToHex, ripemd160, sha1, sha256, sha256d, sha512};
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

/**
  * HMAC Methods
  */
#[wasm_bindgen]
impl Hash {
  fn hmac<T>(input: &[u8], key: &[u8]) -> Hmac<T> where T: BitcoinHash {
    let mut engine = HmacEngine::<T>::new(key);
    engine.input(input);
    let hmac = Hmac::<T>::from_engine(engine);

    hmac
  }

  #[wasm_bindgen(js_name = sha512Hmac)]
  pub fn sha_512_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<sha512::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }

  #[wasm_bindgen(js_name = sha256Hmac)]
  pub fn sha_256_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<sha256::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }

  #[wasm_bindgen(js_name = sha256dHmac)]
  pub fn sha_256d_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<sha256d::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }

  #[wasm_bindgen(js_name = sha1Hmac)]
  pub fn sha_1_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<sha1::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }

  #[wasm_bindgen(js_name = ripemd160Hmac)]
  pub fn ripemd_160_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<ripemd160::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }

  #[wasm_bindgen(js_name = hash160Hmac)]
  pub fn hash_160_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<hash160::Hash>(input, key);

    Self(hmac.as_inner().to_vec())
  }
}