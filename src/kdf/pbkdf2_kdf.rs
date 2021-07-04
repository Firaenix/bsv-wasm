use pbkdf2::{Params, Pbkdf2, password_hash::{Ident, PasswordHasher, Salt, SaltString}};
use anyhow::*;
use rand_core::OsRng;
use crate::{KDF, PBKDF2Errors, hash::Hash};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, throw_str};


#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PBKDF2Hashes {
  SHA1,
  SHA256,
  SHA512
}

impl KDF {
  /**
   * 
   */
  pub fn pbkdf2_impl(password: &[u8], salt: &[u8], hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize)-> Result<KDF, PBKDF2Errors> {
    let s = match SaltString::b64_encode(salt) {
      Ok(v) => v,
      Err(e) => return Err(PBKDF2Errors::UseSaltError{ error: anyhow!("{}. Salt length must be between {} and {}. Your salt length: {}.", e, Salt::MIN_LENGTH, Salt::MAX_LENGTH, salt.len()) })
    };

    let algo_ident = match hash_algo {
      PBKDF2Hashes::SHA1 => Ident::new("pbkdf2"),
      PBKDF2Hashes::SHA256 => Ident::new("pbkdf2-sha256"),
      PBKDF2Hashes::SHA512 => Ident::new("pbkdf2-sha512"),
    };

    let params = Params { rounds, output_length };
    let password_hash = match Pbkdf2.hash_password(password, Some(algo_ident), params, s.as_salt()) {
      Ok(v) => v,
      Err(e) => return Err(PBKDF2Errors::HashError{ error: anyhow!(e) })
    };

    let hash = password_hash.hash.ok_or(PBKDF2Errors::HashError{ error: anyhow!("Failed to generate password hash") })?;
    let result = hash.as_bytes().to_vec();

    Ok(KDF{ hash: Hash(result), salt: s.as_bytes().to_vec( )})
  }

  pub fn pbkdf2_random_salt_impl(password: &[u8], hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize)-> Result<KDF, PBKDF2Errors> {
    let salt = SaltString::generate(&mut OsRng);
    KDF::pbkdf2_impl(password, salt.as_bytes(), hash_algo, rounds, output_length)
  }
}

/**
 * Native PBKDF2 Exports
 */
#[cfg(not(target_arch = "wasm32"))]
impl KDF {
  /**
   * Implementation of PBKDF2 - when None is specified for salt, a random salt will be generated
   */
  pub fn pbkdf2(password: &[u8], salt: Option<&[u8]>, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<KDF, PBKDF2Errors> {
    match salt {
      Some(s) => KDF::pbkdf2_impl(password, s, hash_algo, rounds, output_length),
      None => KDF::pbkdf2_random_salt_impl(password, hash_algo, rounds, output_length)
    }
  }
}

/**
 * JS Boundary PBKDF2 Exports
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl KDF {
  /**
   * Implementation of PBKDF2 - when undefined is specified for salt, a random salt will be generated
   */
  #[wasm_bindgen(js_name = pbkdf2)]
  pub fn pbkdf2(password: &[u8], salt: Option<&[u8]>, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> Result<KDF, JsValue> {
    let res = match salt {
      Some(s) => KDF::pbkdf2_impl(password, &s, hash_algo, rounds, output_length),
      None => KDF::pbkdf2_random_salt_impl(password, hash_algo, rounds, output_length)
    };

    match res {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}
