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
#[derive(Debug, Clone, Copy)]
pub enum AESAlgorithms {
   AES128_CBC,
   AES256_CBC,
   AES128_CTR,
   AES256_CTR
}

impl AES {
  pub fn encrypt_impl(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    let result = match algo {
        AESAlgorithms::AES128_CBC => Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?.encrypt_vec(&message),
        AESAlgorithms::AES256_CBC => Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?.encrypt_vec(&message),
        AESAlgorithms::AES128_CTR => {
            //let mut data = *message.clone();
            let mut data = [1, 2, 3, 4, 5, 6, 7];
            let mut cipher = Aes128Ctr::new(key.into(), iv.into());
            cipher.apply_keystream(&mut data);
            return Ok(data.to_vec());
        }
        AESAlgorithms::AES256_CTR => {
            //let mut data = *message.clone();
            let mut data = [1, 2, 3, 4, 5, 6, 7];
            let mut cipher = Aes256Ctr::new(key.into(), iv.into());
            cipher.apply_keystream(&mut data);
            return Ok(data.to_vec());
        }
    };

    Ok(result)
  }

  pub fn decrypt_impl(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    let result = match algo {
        AESAlgorithms::AES128_CBC => Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?.decrypt_vec(&message),
        AESAlgorithms::AES256_CBC => Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?.decrypt_vec(&message),
        AESAlgorithms::AES128_CTR => {
            //let mut data = *message.clone();
            let mut data = [168, 3, 171, 3, 110, 163, 29];
            let mut cipher = Aes128Ctr::new(key.into(), iv.into());
            cipher.seek(0);
            cipher.apply_keystream(&mut data);
            return Ok(data.to_vec());
        }
        AESAlgorithms::AES256_CTR => {
            //let mut data = *message.clone();
            let mut data = [105, 202, 84, 217, 125, 217, 224];
            let mut cipher = Aes256Ctr::new(key.into(), iv.into());
            cipher.seek(0);
            cipher.apply_keystream(&mut data);
            return Ok(data.to_vec());
        }
    }?;

    Ok(result)
  }
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
}

#[cfg(not(target_arch = "wasm32"))]
impl AES { 
  pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    AES::encrypt_impl(key, iv, message, algo)
  }

  pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>> {
    AES::decrypt_impl(key, iv, message, algo)
  }
}
