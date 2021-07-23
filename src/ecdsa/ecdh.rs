use crate::{BSVErrors, PrivateKey, PublicKey};
use elliptic_curve::ecdh::{self, diffie_hellman};
use elliptic_curve::sec1::ToEncodedPoint;
use elliptic_curve::Scalar;

use rand_core::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

pub struct ECDH {}

impl ECDH {
  /**
   * Derives the shared key between a recipients public key and an optional private key.
   */
  pub(crate) fn derive_shared_key_impl(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    let internal_key = k256::PublicKey::from_sec1_bytes(&pub_key.to_bytes_impl()?)?;
    let shared = diffie_hellman(priv_key.secret_key.to_secret_scalar(), internal_key.as_affine());
    let bytes = shared.as_bytes();
    Ok(bytes.as_slice().to_vec())
  }
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECDH {
  #[wasm_bindgen(js_name = deriveSharedKey)]
  pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, JsValue> {
    match ECDH::derive_shared_key_impl(priv_key, pub_key) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDH {
  pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    ECDH::derive_shared_key_impl(priv_key, pub_key)
  }
}
