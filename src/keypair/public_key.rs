use std::borrow::Borrow;

use wasm_bindgen::prelude::*;
use k256::PublicKey as PubKey;
use elliptic_curve::sec1::*;


#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct PublicKey {
  pub(crate) pub_key: PubKey
}

#[wasm_bindgen]
impl PublicKey {
  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
    hex::encode(self.pub_key.to_encoded_point(true).to_bytes())
  }
}