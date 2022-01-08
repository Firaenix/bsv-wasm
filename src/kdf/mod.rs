pub mod pbkdf2_kdf;
pub use pbkdf2_kdf::*;

use crate::utils::{from_hex, to_hex};
use crate::Hash;
use serde::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-kdf"), wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KDF {
    hash: Hash,
    #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
    salt: Vec<u8>,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-kdf"), wasm_bindgen)]
impl KDF {
    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-kdf"), wasm_bindgen(js_name = getHash))]
    pub fn get_hash(&self) -> Hash {
        self.hash.clone()
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-kdf"), wasm_bindgen(js_name = getSalt))]
    pub fn get_salt(&self) -> Vec<u8> {
        self.salt.clone()
    }
}
