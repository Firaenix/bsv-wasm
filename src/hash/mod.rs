use crate::hash::digest::Digest;
use elliptic_curve::consts::U128;
use elliptic_curve::generic_array::ArrayLength;
use hmac::crypto_mac::Key;
use hmac::{Hmac, Mac, NewMac, digest};
use anyhow::*;
use hmac::digest::{BlockInput, FixedOutput, Reset, Update};
use ripemd160::{Ripemd160};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, throw_str};
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
  #[wasm_bindgen(js_name = toBytes)]
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
    let digest = Sha256::default().chain(Sha256::digest(input)).finalize();

    Hash(digest.as_bytes())
  }

  #[wasm_bindgen(js_name = sha256)]
  pub fn sha_256(input: &[u8]) -> Self {
    Hash(Sha256::digest(input).as_bytes())
  }

  #[wasm_bindgen(js_name = sha1)]
  pub fn sha_1(input: &[u8]) -> Self {
    Hash(Sha1::digest(input).as_bytes())
  }

  #[wasm_bindgen(js_name = ripemd160)]
  pub fn ripemd_160(input: &[u8]) -> Self {
    Hash(Ripemd160::digest(input).as_bytes())
  }

  #[wasm_bindgen(js_name = hash160)]
  pub fn hash_160(input: &[u8]) -> Self {
    Hash(Ripemd160::default().chain(Sha256::digest(input)).finalize().as_bytes())
  }

  #[wasm_bindgen(js_name = sha512)]
  pub fn sha_512(input: &[u8]) -> Self {
    Hash(Sha512::digest(input).as_bytes())
  }
}

/**
  * HMAC Methods
  */
#[wasm_bindgen]
impl Hash {
  
  // D::BlockSize: ArrayLength<u8>
  fn hmac<T>(input: &[u8], key: &[u8]) -> Hmac<T> 
    where T: Digest + Update + BlockInput + FixedOutput + Reset + Default + Clone
  {
    let hmac_key = Key::<Hmac<T>>::from_slice(key);
    let mut engine = Hmac::<T>::new(hmac_key);
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
    let hmac = Hash::hmac::<hash160::Hash>(input, key);

    Self(hmac.finalize().into_bytes().to_vec())
  }
}
