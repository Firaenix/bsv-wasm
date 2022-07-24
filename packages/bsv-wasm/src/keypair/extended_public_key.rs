use bsv::ExtendedPublicKey as BSVExtendedPublicKey;
use wasm_bindgen::prelude::*;

use crate::keypair::{public_key::PublicKey, extended_private_key::ExtendedPrivateKey};

#[wasm_bindgen]
pub struct ExtendedPublicKey(pub(crate) BSVExtendedPublicKey);

#[wasm_bindgen]
impl ExtendedPublicKey {
    pub fn get_public_key(&self) -> PublicKey {
        PublicKey(self.0.get_public_key())
    }

    pub fn from_xpriv(xpriv: &ExtendedPrivateKey) -> Self {
       ExtendedPublicKey(BSVExtendedPublicKey::from_xpriv(&xpriv.0)) 
    }

    pub fn get_chain_code(&self) -> Vec<u8> {
        self.0.get_chain_code()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getDepth))]
    pub fn get_depth(&self) -> u8 {
        self.0.get_depth()
    }

    pub fn get_parent_fingerprint(&self) -> Vec<u8> {
        self.0.get_parent_fingerprint()
    }

    pub fn get_index(&self) -> u32 {
        self.0.get_index()
    }

    pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
        Ok(Self(self.0.derive(index)?))
    }

    pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
        Ok(Self(self.0.derive_from_path(path)?))
    }

    pub fn from_seed(seed: &[u8]) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
        Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_seed(seed)?))
    }

    pub fn from_random() -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
        Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_random()?))
    }

    pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
        Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_string(xpub_string)?))
    }

    pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_string()?)
    }
}


