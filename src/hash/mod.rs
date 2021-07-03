pub mod sha256_digest;
pub use sha256_digest::*;
pub mod sha256d_digest;
pub use sha256d_digest::*;

use wasm_bindgen::prelude::*;
use serde::*;
use crate::utils::{from_hex, to_hex};
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash (
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] 
  pub(crate) Vec<u8>
);

/**
 * Serialisation Functions
 */
#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = toBytes)]
  pub fn to_bytes(&self) -> Vec<u8> {
    self.0.clone()
  } 

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
    hex::encode(&self.0)
  } 
}

/**
 * Hash Functions
 */
#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = sha256d)]
  pub fn sha_256d(input: &[u8]) -> Self {
    let digest = Sha256::digest(input);
    let double_digest = Sha256::digest(&digest);
    Hash(double_digest.to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha256)]
  pub fn sha_256(input: &[u8]) -> Self {
    let digest = Sha256::digest(input);
    Hash(digest.to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha1)]
  pub fn sha_1(input: &[u8]) -> Self {
    let digest = Sha1::digest(input);
    Hash(digest.to_vec())
  } 
  
  #[wasm_bindgen(js_name = ripemd160)]
  pub fn ripemd_160(input: &[u8]) -> Self {
    let digest = Ripemd160::digest(input);
    Hash(digest.to_vec())
  } 
  
  #[wasm_bindgen(js_name = hash160)]
  pub fn hash_160(input: &[u8]) -> Self {
    let digest = Sha256::digest(input);
    let double_digest = Ripemd160::digest(&digest);
    Hash(double_digest.to_vec())
  } 
  
  #[wasm_bindgen(js_name = sha512)]
  pub fn sha_512(input: &[u8]) -> Self {
    let digest = Sha512::digest(input);
    Hash(digest.to_vec())
  }
}

/**
  * HMAC Methods
  */
#[wasm_bindgen]
impl Hash {
  fn hmac<T: Update + BlockInput + FixedOutput + Reset + Default + Clone>(input: &[u8], key: &[u8]) -> Hmac<T> {
    let mut engine = Hmac::<T>::new_from_slice(key).expect("Invalid Input Length - This shouldnt happen, raise an issue if it does.");;
    engine.update(input);

    engine
  }

  #[wasm_bindgen(js_name = sha512Hmac)]
  pub fn sha_512_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Sha512>(input, key);
    Self(hmac.finalize().into_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = sha256Hmac)]
  pub fn sha_256_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Sha256>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = sha256dHmac)]
  pub fn sha_256d_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Sha256>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = sha1Hmac)]
  pub fn sha_1_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Sha1>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = ripemd160Hmac)]
  pub fn ripemd_160_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Ripemd160>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }

  #[wasm_bindgen(js_name = hash160Hmac)]
  pub fn hash_160_hmac(input: &[u8], key: &[u8]) -> Self {
    let hmac = Hash::hmac::<Ripemd160>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }
}