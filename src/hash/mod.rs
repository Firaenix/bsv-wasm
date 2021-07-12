pub mod digest_utils;
pub mod hash160_digest;
pub mod reverse_digest;
pub mod sha256d_digest;
pub mod sha256r_digest;

pub use digest_utils::*;
pub use hash160_digest::*;
pub use reverse_digest::*;
pub use sha256d_digest::*;
pub use sha256r_digest::*;

use crate::utils::{from_hex, to_hex};
use crate::ToHex;
use digest::Digest;
use hmac::crypto_mac::Key;
use hmac::digest::{BlockInput, FixedOutput, Reset, Update};
use hmac::{Hmac, Mac, NewMac};
use ripemd160::Ripemd160;
use serde::*;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

use self::hash160_digest::Hash160;
use self::sha256d_digest::Sha256d;

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] pub(crate) Vec<u8>);

/**
 * Serialisation Functions
 */
#[wasm_bindgen]
impl Hash {
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

/**
 * Hash Functions
 */
#[wasm_bindgen]
impl Hash {
    #[wasm_bindgen(js_name = sha256d)]
    pub fn sha_256d(input: &[u8]) -> Self {
        Hash((&*Sha256d::digest(input)).to_vec())
    }

    #[wasm_bindgen(js_name = sha256)]
    pub fn sha_256(input: &[u8]) -> Self {
        Hash((&*Sha256::digest(input)).to_vec())
    }

    #[wasm_bindgen(js_name = sha1)]
    pub fn sha_1(input: &[u8]) -> Self {
        Hash((&*Sha1::digest(input)).to_vec())
    }

    #[wasm_bindgen(js_name = ripemd160)]
    pub fn ripemd_160(input: &[u8]) -> Self {
        Hash((&*Ripemd160::digest(input)).to_vec())
    }

    #[wasm_bindgen(js_name = hash160)]
    pub fn hash_160(input: &[u8]) -> Self {
        Hash((&*Hash160::digest(input)).to_vec())
    }

    #[wasm_bindgen(js_name = sha512)]
    pub fn sha_512(input: &[u8]) -> Self {
        Hash((&*Sha512::digest(input)).to_vec())
    }
}

/**
 * HMAC Methods
 */
#[wasm_bindgen]
impl Hash {
    // D::BlockSize: ArrayLength<u8>
    fn hmac<T>(input: &[u8], key: &[u8]) -> Hmac<T>
    where
        T: Update + BlockInput + FixedOutput + Reset + Default + Clone,
    {
        // Should fix this unwrap, but really shouldnt error.
        let mut engine = Hmac::<T>::new_from_slice(key).unwrap();
        engine.update(input);
        engine
    }

    #[wasm_bindgen(js_name = sha512Hmac)]
    pub fn sha_512_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Sha512>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }

    #[wasm_bindgen(js_name = sha256Hmac)]
    pub fn sha_256_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Sha256>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }

    #[wasm_bindgen(js_name = sha256dHmac)]
    pub fn sha_256d_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Sha256d>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }

    #[wasm_bindgen(js_name = sha1Hmac)]
    pub fn sha_1_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Sha1>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }

    #[wasm_bindgen(js_name = ripemd160Hmac)]
    pub fn ripemd_160_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Ripemd160>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }

    #[wasm_bindgen(js_name = hash160Hmac)]
    pub fn hash_160_hmac(input: &[u8], key: &[u8]) -> Self {
        let hmac = Hash::hmac::<Hash160>(input, key);

        Self(hmac.finalize().into_bytes().to_vec())
    }
}
