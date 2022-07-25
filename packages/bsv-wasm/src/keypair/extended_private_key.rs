use wasm_bindgen::prelude::*;
use bsv::ExtendedPrivateKey as BSVExtendedPrivateKey;

use super::{private_key::PrivateKey, public_key::PublicKey};

#[wasm_bindgen]
pub struct ExtendedPrivateKey(pub(crate) BSVExtendedPrivateKey);

#[wasm_bindgen]
impl ExtendedPrivateKey {
    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPrivateKey))]
    pub fn get_private_key(&self) -> PrivateKey {
        PrivateKey(self.0.get_private_key())
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPublicKey))]
    pub fn get_public_key(&self) -> PublicKey {
        PublicKey(self.0.get_public_key())
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getChainCode))]
    pub fn get_chain_code(&self) -> Vec<u8> {
        self.0.get_chain_code()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getDepth))]
    pub fn get_depth(&self) -> u8 {
        self.0.get_depth()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getParentFingerprint))]
    pub fn get_parent_fingerprint(&self) -> Vec<u8> {
        self.0.get_parent_fingerprint()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getIndex))]
    pub fn get_index(&self) -> u32 {
        self.0.get_index()
    }

    pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(self.0.derive(index)?))
    }

    pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(self.0.derive_from_path(path)?))
    }

    pub fn from_seed(seed: &[u8]) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_seed_impl(seed)?))
    }

    pub fn from_random() -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_random()?))
    }

    pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_string(xprv_string)?))
    }

    pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(self.0.to_string()?)
    }

    pub fn from_mnemonic(mnemonic: &[u8], passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
        Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_mnemonic(mnemonic, passphrase)?))
    }
}
