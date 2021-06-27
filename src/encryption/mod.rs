use aes::*;
use block_modes::{BlockMode, Cbc, block_padding::{Pkcs7}};
use anyhow::*;
use wasm_bindgen::{prelude::*, throw_str};


#[wasm_bindgen]
pub struct AES {

}

impl AES {
  pub fn encrypt_128_impl(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?;

    Ok(cipher.encrypt_vec(message))
  } 

  pub fn decrypt_128_impl(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?;

    Ok(cipher.decrypt_vec(&message)?)
  }

  pub fn encrypt_256_impl(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?;

    Ok(cipher.encrypt_vec(&message))
  }

  pub fn decrypt_256_impl(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let cipher = Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?;

    Ok(cipher.decrypt_vec(&message)?)
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl AES { 
  #[wasm_bindgen(js_name = encrypt128)]
  pub fn encrypt_128(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    match AES::encrypt_128_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = decrypt128)]
  pub fn decrypt_128(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    match AES::decrypt_128_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = encrypt256)]
  pub fn encrypt_256(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    match AES::encrypt_256_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = decrypt256)]
  pub fn decrypt_256(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>, JsValue> {
    match AES::decrypt_256_impl(key, iv, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl AES { 
  pub fn encrypt_128(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    AES::encrypt_128_impl(key, iv, message)
  }

  pub fn decrypt_128(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    AES::decrypt_128_impl(key, iv, message)
  }
  pub fn encrypt_256(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    AES::encrypt_256_impl(key, iv, message)
  }

  pub fn decrypt_256(key: &[u8], iv: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    AES::decrypt_256_impl(key, iv, message)
  }
}