use wasm_bindgen::prelude::*;

mod private_key;
mod public_key;

pub use private_key::*;
pub use public_key::*;


#[wasm_bindgen]
#[derive(Debug)]
pub struct KeyPair {
  public_key: PublicKey,
  private_key: PrivateKey
}