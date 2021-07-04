use hmac::Hmac;
use pbkdf2::{Params, Pbkdf2, password_hash::{Ident, PasswordHasher, Salt, SaltString}, pbkdf2};
use anyhow::*;
use rand_core::OsRng;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
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
    let pbkdf2_fn = match hash_algo {
        PBKDF2Hashes::SHA1 => pbkdf2::<Hmac<Sha1>>,
        PBKDF2Hashes::SHA256 => pbkdf2::<Hmac<Sha256>>,
        PBKDF2Hashes::SHA512 => pbkdf2::<Hmac<Sha512>>,
    };
    let mut result = vec![0; output_length];
    pbkdf2_fn(password, salt, rounds, &mut result);

    Ok(KDF{ hash: Hash(result), salt: salt.to_vec()})
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
