use aes::*;
use ctr::cipher::{
    NewCipher, StreamCipher, StreamCipherSeek,
};
use block_modes::{BlockMode, Cbc, block_padding::{Pkcs7}};
use anyhow::*;
use wasm_bindgen::{prelude::*, throw_str};


#[wasm_bindgen]
pub struct AES {

}

#[wasm_bindgen]
pub enum AESAlgorithms {
   AES128,
   AES256
}

impl AES {
  pub fn encrypt_impl(key: &[u8], iv: &[ui], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    let encrypt_fn = match algo {
        AESAlgorithms::AES128 => Cbc::<Aes128, Pkcs7>,
        AESAlgorithms::AES256 => Cbc::<Aes128, Pkcs7>
    };

    let cipher = encrypt_fn::new_from_slices(key, iv)?;
    Ok(cipher.encrypt_vec(message))
  }

  pub fn decrypt_impl(key: &[u8], iv: &[ui], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    let decrypt_fn = match algo {
        AESAlgorithms::AES128 => Cbc::<Aes128, Pkcs7>,
        AESAlgorithms::AES256 => Cbc::<Aes128, Pkcs7>
    };

    let cipher = decrypt_fn::new_from_slices(key, iv)?;
    Ok(cipher.decrypt_vec(message))
  }

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

  //pub fn decrypt_ctr_128_impl(key: &[u8], message: &mut[u8]) -> Result<Vec<u8>> {
    //let cipher = Aes256Ctr.new(key);

    //let mut data = [1, 2, 3, 4, 5, 6, 7];
    ////let key = b"very secret key.";
    //let nonce = b"and secret nonce";
    //let mut cipher = Aes128Ctr::new(key.into(), nonce.into());
    //cipher.apply_keystream(&mut data);
    //Ok(data.to_vec())

    //let iv = aes::cipher::generic_array::GenericArray::from_slice(&[0u8; 16]);
    //let mut cipher = Aes128Ctr::new(key.into(), iv.into());

    //cipher.seek(0);
    //cipher.apply_keystream(message);

    //Ok(message.to_vec())
  //}
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl AES { 
  #[wasm_bindgen(js_name = encrypt)]
  pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, JsValue> {
    match AES::encrypt_impl(key, iv, message, algo) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = decrypt)]
  pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, JsValue> {
    match AES::decrypt_impl(key, iv, message, algo) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

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

  //#[wasm_bindgen(js_name = decryptCtr256)]
  //pub fn decrypt_ctr_128(key: &[u8], message: &mut[u8]) -> Result<Vec<u8>, JsValue> {
    //match AES::decrypt_ctr_128_impl(key, message) {
      //Ok(v) => Ok(v),
      //Err(e) => throw_str(&e.to_string()),
    //}
  //}
}

#[cfg(not(target_arch = "wasm32"))]
impl AES { 
  pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    AES::encrypt_impl(key, iv, message, algo)
  }

  pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    AES::decrypt_impl(key, iv, message, algo)
  }

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
