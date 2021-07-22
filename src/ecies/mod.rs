use k256::ecdh::{self, *};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

use crate::{BSVErrors, Hash, PrivateKey, PublicKey, AES, ECDH};

pub struct ECIES {}

pub struct CipherKeys {
  pub iv: Vec<u8>,
  pub ke: Vec<u8>,
  pub km: Vec<u8>,
}

impl ECIES {
  pub(crate) fn encrypt_impl(message: &[u8], sender_priv_key: Option<PrivateKey>, recipient_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    let private_key = match sender_priv_key {
      Some(pk) => pk,
      None => PrivateKey::from_random(),
    };

    let shared_key = ECDH::derive_shared_key_impl(&private_key, recipient_pub_key)?;
    let cipher = ECIES::derive_cipher_keys(&shared_key)?;

    let cipher_text = AES::encrypt_impl(&cipher.ke, &cipher.iv, message, crate::AESAlgorithms::AES128_CBC)?;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(b"BIE1");

    let r_buf = private_key.get_public_key_impl()?.to_compressed_impl()?.to_bytes_impl()?;
    buffer.extend_from_slice(&r_buf);
    buffer.extend_from_slice(&cipher_text);

    let hmac = Hash::sha_256_hmac(&buffer, &cipher.km).to_bytes();
    buffer.extend_from_slice(&hmac);

    Ok(buffer)
  }

  pub(crate) fn decrypt_impl(bie_cipher_text: &[u8], recipient_priv_key: &PrivateKey, sender_pub_key: Option<PublicKey>) -> Result<Vec<u8>, BSVErrors> {
    if &bie_cipher_text[0..4] != b"BIE1" {
      return Err(BSVErrors::DecryptionError("Cipher text did not start with BIE".into()));
    }

    let pub_key = match sender_pub_key.clone() {
      Some(p) => p.clone(),
      None => PublicKey::from_bytes_impl(&bie_cipher_text[4..37])?,
    };

    let shared_key = ECDH::derive_shared_key_impl(recipient_priv_key, &pub_key)?;
    let cipher_keys = ECIES::derive_cipher_keys(&shared_key)?;

    let hmac_start_idx = bie_cipher_text.len() - 32;
    let cipher_text = &bie_cipher_text[37..hmac_start_idx];

    let hmac = &bie_cipher_text[hmac_start_idx..bie_cipher_text.len()];
    let verify_hmac = Hash::sha_256_hmac(&bie_cipher_text[0..hmac_start_idx], &cipher_keys.km);

    if hmac != &verify_hmac.to_bytes() {
      return Err(BSVErrors::DecryptionError("Invalid Checksum".into()));
    }

    let plain_text = AES::decrypt_impl(&cipher_keys.ke, &cipher_keys.iv, cipher_text, crate::AESAlgorithms::AES128_CBC)?;
    Ok(plain_text)
  }

  pub fn derive_cipher_keys(shared_key_bytes: &[u8]) -> Result<CipherKeys, BSVErrors> {
    let hash = Hash::sha_512(shared_key_bytes).to_bytes();

    Ok(CipherKeys {
      iv: hash[0..16].into(),
      ke: hash[16..32].into(),
      km: hash[32..64].into(),
    })
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECIES {
  pub fn encrypt(message: &[u8], sender_priv_key: Option<PrivateKey>, recipient_pub_key: &PublicKey) -> Result<Vec<u8>, JsValue> {
    match ECIES::encrypt_impl(message, sender_priv_key, recipient_pub_key) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  pub fn decrypt(bie_cipher_text: &[u8], recipient_priv_key: &PrivateKey, sender_pub_key: Option<PublicKey>) -> Result<Vec<u8>, JsValue> {
    match ECIES::decrypt_impl(bie_cipher_text, recipient_priv_key, sender_pub_key) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECIES {
  pub fn encrypt(message: &[u8], sender_priv_key: Option<PrivateKey>, recipient_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    ECIES::encrypt_impl(message, sender_priv_key, recipient_pub_key)
  }

  pub fn decrypt(bie_cipher_text: &[u8], recipient_priv_key: &PrivateKey, sender_pub_key: Option<PublicKey>) -> Result<Vec<u8>, BSVErrors> {
    ECIES::decrypt_impl(bie_cipher_text, recipient_priv_key, sender_pub_key)
  }
}
