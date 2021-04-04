use bitcoin_hashes::{Hash, hex::ToHex};
use wasm_bindgen::prelude::*;

pub mod keypair;
pub use keypair::*;

pub mod signature;
pub use signature::*;

pub mod traits;
pub use traits::*;

pub mod errors;
pub use errors::*;

pub mod address;
pub use address::*;

pub mod types;
pub use types::*;


#[wasm_bindgen]
pub fn hash(input: Vec<u8>) -> String {
  bitcoin_hashes::sha256d::Hash::hash(&input).to_hex()
} 