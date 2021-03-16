use wasm_bindgen::prelude::*;
use k256::ecdsa::signature::Signature;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}

#[wasm_bindgen]
pub fn sig() -> Vec<u8> {
  use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    SecretKey,
};
  use rand_core::OsRng; // requires 'getrandom' feature

  // Signing
  let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
  let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

  // Note: the signature type must be annotated or otherwise inferrable as
  // `Signer` has many impls of the `Signer` trait (for both regular and
  // recoverable signature types).
  let signature: Signature = signing_key.sign(message);

  // Verification
  use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};

  let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
  assert!(verify_key.verify(message, &signature).is_ok());

  signature.as_bytes().to_vec()
}