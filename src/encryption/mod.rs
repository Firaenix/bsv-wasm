use aes::*;
use block_modes::{BlockMode, Cbc, block_padding::NoPadding};
use anyhow::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AES {

}

impl AES {
  pub fn encrypt_128_impl(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes128, NoPadding>::new_from_slices(key, iv).unwrap();

    Ok(cipher.encrypt(message, message.len())?.to_vec())
  } 

  pub fn decrypt_128_impl(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes128, NoPadding>::new_from_slices(key, iv).unwrap();

    Ok(cipher.decrypt(message)?.to_vec())
  }

  pub fn encrypt_256_impl(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes256, NoPadding>::new_from_slices(key, iv).unwrap();

    Ok(cipher.encrypt(message, message.len())?.to_vec())
  }

  pub fn decrypt_256_impl(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes256, NoPadding>::new_from_slices(key, iv).unwrap();

    Ok(cipher.decrypt(message)?.to_vec())
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl AES { 
  #[wasm_bindgen(js_name = encrypt128)]
  pub fn encrypt_128(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>, JsValue> {
    match AES::encrypt_128_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = decrypt128)]
  pub fn decrypt_128(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>, JsValue> {
    match AES::decrypt_128_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = encrypt256)]
  pub fn encrypt_256(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>, JsValue> {
    match AES::encrypt_256_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = decrypt256)]
  pub fn decrypt_256(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>, JsValue> {
    match AES::decrypt_256_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl AES { 
  pub fn encrypt_128(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    AES::encrypt_128_impl(key, iv, message)
  }

  pub fn decrypt_128(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    AES::decrypt_128_impl(key, iv, message)
  }
  pub fn encrypt_256(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    AES::encrypt_256_impl(key, iv, message)
  }

  pub fn decrypt_256(key: &[u8], iv: &[u8], message: &mut [u8]) -> Result<Vec<u8>> {
    AES::decrypt_256_impl(key, iv, message)
  }
}