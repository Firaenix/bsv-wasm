use crate::{BSVErrors, PrivateKey, PublicKey};
use elliptic_curve::ecdh::diffie_hellman;

use wasm_bindgen::prelude::*;

use wasm_bindgen::{throw_str, JsValue};

#[cfg_attr(all(feature = "wasm-bindgen-ecdh"), wasm_bindgen)]
#[derive(Clone)]
pub struct ECDH {}

impl ECDH {
    /**
     * Derives the shared key between a recipients public key and an optional private key.
     */
    pub(crate) fn derive_shared_key_impl(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        let internal_key = k256::PublicKey::from_sec1_bytes(&pub_key.to_bytes_impl()?)?;
        let shared = diffie_hellman(priv_key.secret_key.to_nonzero_scalar(), internal_key.as_affine());
        let bytes = shared.as_bytes();
        Ok(bytes.as_slice().to_vec())
    }
}


#[cfg_attr(all(feature = "wasm-bindgen-ecdh"), wasm_bindgen)]
impl ECDH {
    #[cfg_attr(all(feature = "wasm-bindgen-ecdh"), wasm_bindgen(js_name = deriveSharedKey))]
    pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(ECDH::derive_shared_key_impl(priv_key, pub_key)?)
    }
}

#[cfg(not(feature = "wasm-bindgen-ecdh"))]
impl ECDH {
    pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        ECDH::derive_shared_key_impl(priv_key, pub_key)
    }
}
