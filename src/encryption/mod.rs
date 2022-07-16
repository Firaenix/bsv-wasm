use crate::BSVErrors;
use aes::{
    cipher::{NewCipher, StreamCipher, StreamCipherSeek},
    Aes128, Aes128Ctr, Aes256, Aes256Ctr,
};
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
#[cfg(feature = "wasm-bindgen-encryption")]
use wasm_bindgen::{prelude::*, throw_str};

#[cfg_attr(all(feature = "wasm-bindgen-encryption"), wasm_bindgen)]
pub struct AES {}

#[cfg_attr(all(feature = "wasm-bindgen-encryption"), wasm_bindgen)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum AESAlgorithms {
    AES128_CBC,
    AES256_CBC,
    AES128_CTR,
    AES256_CTR,
}

impl AES {
    pub fn encrypt_impl(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, BSVErrors> {
        let result = match algo {
            AESAlgorithms::AES128_CBC => Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?.encrypt_vec(message),
            AESAlgorithms::AES256_CBC => Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?.encrypt_vec(message),
            AESAlgorithms::AES128_CTR => AES::aes_ctr::<Aes128Ctr>(key, iv, message),
            AESAlgorithms::AES256_CTR => AES::aes_ctr::<Aes256Ctr>(key, iv, message),
        };

        Ok(result)
    }

    pub fn decrypt_impl(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, BSVErrors> {
        let result = match algo {
            AESAlgorithms::AES128_CBC => Cbc::<Aes128, Pkcs7>::new_from_slices(key, iv)?.decrypt_vec(message)?,
            AESAlgorithms::AES256_CBC => Cbc::<Aes256, Pkcs7>::new_from_slices(key, iv)?.decrypt_vec(message)?,
            AESAlgorithms::AES128_CTR => AES::aes_ctr::<Aes128Ctr>(key, iv, message),
            AESAlgorithms::AES256_CTR => AES::aes_ctr::<Aes256Ctr>(key, iv, message),
        };
        Ok(result)
    }

    fn aes_ctr<T: NewCipher + StreamCipherSeek + StreamCipher>(key: &[u8], iv: &[u8], message: &[u8]) -> Vec<u8> {
        let data = &mut message.to_vec();
        let mut cipher = T::new(key.into(), iv.into());
        cipher.seek(0);
        cipher.apply_keystream(data);
        data.to_vec()
    }
}


#[cfg_attr(all(feature = "wasm-bindgen-encryption"), wasm_bindgen)]
#[cfg(feature = "wasm-bindgen-encryption")]
impl AES {
    #[cfg_attr(all(feature = "wasm-bindgen-encryption"), wasm_bindgen(js_name = encrypt))]
    pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(AES::encrypt_impl(key, iv, message, algo)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-encryption"), wasm_bindgen(js_name = decrypt))]
    pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(AES::decrypt_impl(key, iv, message, algo)?)
    }
}

#[cfg(not(feature = "wasm-bindgen-encryption"))]
impl AES {
    pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, BSVErrors> {
        AES::encrypt_impl(key, iv, message, algo)
    }

    pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, BSVErrors> {
        AES::decrypt_impl(key, iv, message, algo)
    }
}
