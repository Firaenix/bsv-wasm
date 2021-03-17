use std::borrow::Borrow;

use crate::PublicKey;
use anyhow::private::kind::TraitKind;
use bitcoin_hashes::{Hash, hex::FromHex};
use js_sys::Error;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct P2PKHAddress {
  public_key: PublicKey
}

#[wasm_bindgen]
impl P2PKHAddress {
  #[wasm_bindgen(constructor)]
  pub fn new(pub_key: PublicKey) -> P2PKHAddress {
    P2PKHAddress{
      public_key: pub_key
    }
  }

  pub fn to_address_string(&self) -> Result<String, JsValue> {
    let pub_key_hex = self.public_key.to_hex();
    let bytes =  match hex::decode(pub_key_hex.clone()) {
      Ok(v) => v,
      Err(e) => wasm_bindgen::throw_str(&e.to_string())
    };

    let output = bitcoin_hashes::hash160::Hash::hash(&bytes);

    Ok(output.to_string())
  }
}