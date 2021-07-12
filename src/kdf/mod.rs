pub mod pbkdf2_kdf;
pub use pbkdf2_kdf::*;

use crate::utils::{from_hex, to_hex};
use crate::Hash;
use serde::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KDF {
    hash: Hash,
    #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
    salt: Vec<u8>,
}

#[wasm_bindgen]
impl KDF {
    #[wasm_bindgen(js_name = getHash)]
    pub fn get_hash(&self) -> Hash {
        self.hash.clone()
    }

    #[wasm_bindgen(js_name = getSalt)]
    pub fn get_salt(&self) -> Vec<u8> {
        self.salt.clone()
    }
}
