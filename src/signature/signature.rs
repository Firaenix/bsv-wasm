use crate::PublicKey;
use wasm_bindgen::{prelude::*, throw_str};
use k256::{Secp256k1, ecdsa::Signature as SecpSignature, Scalar, ecdsa::{VerifyingKey, signature::Verifier}};
use elliptic_curve::sec1::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Signature {
  sig: k256::ecdsa::Signature
}


#[wasm_bindgen]
impl Signature {
  #[wasm_bindgen(js_name = fromDER)]
  pub fn from_der(bytes: Vec<u8>) -> crate::types::Result<Signature> {
    let sig = match SecpSignature::from_der(&bytes) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    Ok(Signature{
      sig
    })
  }

  #[wasm_bindgen(js_name = fromHexDER)]
  pub fn from_hex_der(hex: String) -> crate::types::Result<Signature> {
    let bytes = match hex::decode(hex) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    let sig = match SecpSignature::from_der(&bytes) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    Ok(Signature{
      sig
    })
  }

  #[wasm_bindgen(js_name = toHex)]
  #[wasm_bindgen(js_name = toDER)]
  pub fn to_hex(&self) -> String {
    let bytes = self.sig.to_der();

    hex::encode(bytes)
  }

  pub fn verify(&self, message: Vec<u8>, pub_key: &PublicKey) -> crate::Result<bool> {
    let point = match EncodedPoint::from_bytes(pub_key.to_bytes()?) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };
    let key = match VerifyingKey::from_encoded_point(&point) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    Ok(key.verify(&message, &self.sig).is_ok())
  }

  // #[wasm_bindgen(js_name = fromRS)]
  // pub fn from_r_s(r: Vec<u8>, s: Vec<u8>) -> crate::types::Result<Signature> {
  //   let sig = match SecpSignature::from_scalars(r.into(), s.into()) {
  //     Ok(v) => v,
  //     Err(e) => throw_str(&e.to_string())
  //   };

  //   Ok(Signature{
  //     sig
  //   })
  // }
}