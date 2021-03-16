use std::borrow::Borrow;

use crate::PublicKey;
use anyhow::private::kind::TraitKind;
use bitcoin_hashes::hex::FromHex;
use wasm_bindgen::prelude::*;

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

  pub fn to_address_string(&self) -> String {
    let pub_key_hex = self.public_key.to_hex();
    let output = bitcoin_hashes::hash160::Hash::from_hex(pub_key_hex.borrow());

    return match output {
      Ok(e) => String::from("Ye"),
      Err(e) => String::from(e.to_string())
    };
  }
}