use crate::{P2PKHAddress, PrivateKey, Signature, SigningHash, ECDSA};
use anyhow::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

/**
 * Bitcoin Signed Message
 */
#[wasm_bindgen]
pub struct BSM {}

impl BSM {
  /**
   * TODO: Sign a message with the intention of verifying with this same Address.
   * Used when using Bitcoin Signed Messages ()
   */
  pub(crate) fn sign_impl(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature> {
    ECDSA::sign_with_deterministic_k_impl(priv_key, message, SigningHash::Sha256d, false)
  }

  pub(crate) fn verify_message_impl(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool> {
    let public_key = signature.get_public_key(message, SigningHash::Sha256d)?;
    let verify_p2pkh = P2PKHAddress::from_pubkey_impl(&public_key)?;

    let verify_address = verify_p2pkh.to_address_string_impl()?;
    let address_string = address.to_address_string_impl()?;
    if verify_address != address_string {
      return Err(anyhow!("Provided address ({}) does not match signature address ({})", verify_address, address_string));
    }

    ECDSA::verify_digest_impl(message, &public_key, signature, SigningHash::Sha256d)?;
    Ok(true)
  }
}

#[wasm_bindgen]
impl BSM {
  /**
   * Sign a message with the intention of verifying with this same Address.
   * Used when using Bitcoin Signed Messages
   *
   * Returns boolean
   */
  #[wasm_bindgen(js_name = isValidMessage)]
  pub fn is_valid_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> bool {
    match BSM::verify_message_impl(message, signature, address) {
      Ok(_) => true,
      Err(_) => false,
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BSM {
  #[wasm_bindgen(js_name = verifyMessage)]
  pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, JsValue> {
    match BSM::verify_message_impl(message, signature, address) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = signMessage)]
  pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, JsValue> {
    match BSM::sign_impl(priv_key, message) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl BSM {
  pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool> {
    BSM::verify_message_impl(message, signature, address)
  }

  pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature> {
    BSM::sign_impl(priv_key, message)
  }
}
