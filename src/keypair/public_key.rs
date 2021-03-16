use wasm_bindgen::prelude::*;
use k256::PublicKey as PubKey;


#[wasm_bindgen]
#[derive(Debug)]
pub struct PublicKey {
  pub_key: PubKey
}

#[wasm_bindgen]
impl PublicKey {
}