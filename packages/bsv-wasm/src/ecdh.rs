use bsv::ECDH as BSVECDH;
use wasm_bindgen::prelude::*;

use crate::keypair::{private_key::PrivateKey, public_key::PublicKey};

#[wasm_bindgen]
pub struct ECDH;

#[wasm_bindgen]
impl ECDH {
    pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(BSVECDH::derive_shared_key(&priv_key.0, &pub_key.0)?)
    }
}
