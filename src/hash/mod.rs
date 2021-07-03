pub mod pbkdf2_errors;
pub use pbkdf2_errors::*;

use anyhow::*;
use pbkdf2::{Algorithm, Params, Pbkdf2, password_hash::{Ident, Output, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString}};

use bitcoin_hashes::{Hash as BitcoinHash, HashEngine, Hmac, HmacEngine, hash160, hex::ToHex, ripemd160, sha1, sha256, sha256d, sha512};
use rand_core::OsRng;
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

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PBKDF2Hashes {
  SHA1,
  SHA256,
  SHA512
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PBKDF2Result {
  hash: Hash,
  #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
  salt: Vec<u8>
}

#[wasm_bindgen]
impl PBKDF2Result {
  #[wasm_bindgen(js_name = getHash)]
  pub fn get_hash(&self) -> Hash {
    self.hash.clone()
  }

  #[wasm_bindgen(js_name = getSalt)]
  pub fn get_salt(&self) -> Vec<u8> {
    self.salt.clone()
  }
}

impl Hash {
  /**
   * 
   */
  pub fn pbkdf2_impl(password: &str, salt: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize)-> Result<Self, PBKDF2Errors> {
    let s = match SaltString::b64_encode(salt.as_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(PBKDF2Errors::UseSaltError{ error: anyhow!("{}. B64 Salt length must be between {} and {}. Your salt: {}.", e, Salt::MIN_LENGTH, Salt::MAX_LENGTH, salt) })
    };

    let algo_ident = match hash_algo {
      PBKDF2Hashes::SHA1 => Ident::new("pbkdf2"),
      PBKDF2Hashes::SHA256 => Ident::new("pbkdf2-sha256"),
      PBKDF2Hashes::SHA512 => Ident::new("pbkdf2-sha512"),
    };

    let params = Params { rounds, output_length };
    let password_hash = match Pbkdf2.hash_password(password.as_bytes(), Some(algo_ident), params, s.as_salt()) {
      Ok(v) => v,
      Err(e) => return Err(PBKDF2Errors::HashError{ error: anyhow!(e) })
    };

    let hash = password_hash.hash.ok_or(PBKDF2Errors::HashError{ error: anyhow!("Failed to generate password hash") })?;
    let result = hash.as_bytes().to_vec();
    // result.reverse();
    Ok(Hash(result))
  }

  pub fn pbkdf2_random_salt_impl(password: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize)-> Result<PBKDF2Result, PBKDF2Errors> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PBKDF2Result { hash: Hash::pbkdf2_impl(password, salt.as_str(), hash_algo, rounds, output_length)?, salt: salt.as_bytes().to_vec() })
  }
}

/**
 * Native PBKDF2 Exports
 */
#[cfg(not(target_arch = "wasm32"))]
impl Hash {
  pub fn pbkdf2(password: &str, salt: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<Hash, PBKDF2Errors> {
    Hash::pbkdf2_impl(password, salt, hash_algo, rounds, output_length)
  }

  pub fn pbkdf2_random_salt(password: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<PBKDF2Result, PBKDF2Errors> {
    Hash::pbkdf2_random_salt_impl(password, hash_algo, rounds, output_length)
  }
}

/**
 * JS Boundary PBKDF2 Exports
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Hash {
  #[wasm_bindgen(js_name = pbkdf2)]
  pub fn pbkdf2(password: &str, salt: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<Hash, JsValue> {
    match Hash::pbkdf2_impl(password, salt, hash_algo, rounds, output_length) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = pbkdf2RandomSalt)]
  pub fn pbkdf2_random_salt(password: &str, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<PBKDF2Result, JsValue> {
    match Hash::pbkdf2_random_salt_impl(password, hash_algo, rounds, output_length) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}