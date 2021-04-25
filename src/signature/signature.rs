use crate::{PublicKey, SignatureErrors};
use wasm_bindgen::{prelude::*, throw_str};
use k256::{Secp256k1, ecdsa::Signature as SecpSignature, Scalar, ecdsa::{VerifyingKey, signature::Verifier}};
use elliptic_curve::sec1::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
  sig: k256::ecdsa::Signature
}


/**
 * Implementation Methods
 */
impl Signature {
  pub(crate) fn from_der_impl(bytes: Vec<u8>) -> Result<Signature, SignatureErrors> {
    let sig = match SecpSignature::from_der(&bytes) {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::SecpError{ error: e })
    };

    Ok(Signature{
      sig
    })
  }

  
  pub(crate) fn from_hex_der_impl(hex: String) -> Result<Signature, SignatureErrors> {
    let bytes = match hex::decode(hex) {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::ParseHex{ error: e })
    };

    let sig = match SecpSignature::from_der(&bytes) {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::SecpError{ error: e })
    };

    Ok(Signature{
      sig
    })
  }

  
  pub(crate) fn to_hex_impl(&self) -> String {
    let bytes = self.sig.to_der();

    hex::encode(bytes)
  }

  pub(crate) fn to_der_bytes_impl(&self) -> Vec<u8> {
    let bytes = self.sig.to_der();

    bytes.as_bytes().to_vec()
  }

  pub(crate) fn verify_impl(&self, message: Vec<u8>, pub_key: &PublicKey) -> Result<bool, SignatureErrors> {
    let pub_key_bytes = match pub_key.to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::PublicKeyError{ error: e })
    };

    let point = match EncodedPoint::from_bytes(pub_key_bytes) {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::InvalidPoint{ error: e })
    };

    let key = match VerifyingKey::from_encoded_point(&point) {
      Ok(v) => v,
      Err(e) => return Err(SignatureErrors::SecpError{ error: e })
    };

    Ok(key.verify(&message, &self.sig).is_ok())
  }
}


/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl Signature {
  
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromDER))]
  pub fn from_der(bytes: Vec<u8>) -> Result<Signature, JsValue> {
    match Signature::from_der_impl(bytes) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHexDER))]
  pub fn from_hex_der(hex: String) -> Result<Signature, JsValue> {
    match Signature::from_hex_der_impl(hex) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
  pub fn to_hex(&self) -> String {
    Signature::to_hex_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toDER))]
  pub fn to_der_bytes(&self) -> Vec<u8> {
    Signature::to_der_bytes_impl(&self)
  }

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = verify))]
  pub fn verify(&self, message: Vec<u8>, pub_key: &PublicKey) -> Result<bool, JsValue> {
    match Signature::verify_impl(&self, message, pub_key) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}


/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl Signature {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_der(bytes: Vec<u8>) -> Result<Signature, SignatureErrors> {
    Signature::from_der_impl(bytes)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_hex_der(hex: String) -> Result<Signature, SignatureErrors> {
    Signature::from_hex_der_impl(hex)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_hex(&self) -> String {
    Signature::to_hex_impl(&self)
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub fn verify(&self, message: Vec<u8>, pub_key: &PublicKey) -> Result<bool, SignatureErrors> {
    Signature::verify_impl(&self, message, pub_key)
  }
}