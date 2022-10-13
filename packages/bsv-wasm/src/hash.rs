use bsv::Hash as BSVHash;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Hash(pub(crate) BSVHash);

impl From<BSVHash> for Hash {
    fn from(v: BSVHash) -> Hash {
        Hash(v)
    }
}

impl From<Hash> for BSVHash {
    fn from(v: Hash) -> BSVHash {
        v.0
    }
}

/**
 * Serialisation Functions
 */
#[wasm_bindgen]
impl Hash {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
    pub fn sha_256d(input: &[u8]) -> Self {
        Hash(BSVHash::sha_256d(input))
    }

    pub fn sha_256(input: &[u8]) -> Self {
        Hash(BSVHash::sha_256(input))
    }

    pub fn sha_1(input: &[u8]) -> Self {
        Hash(BSVHash::sha_1(input))
    }

    pub fn ripemd_160(input: &[u8]) -> Self {
        Hash(BSVHash::ripemd_160(input))
    }

    pub fn hash_160(input: &[u8]) -> Self {
        Hash(BSVHash::hash_160(input))
    }

    pub fn sha_512(input: &[u8]) -> Self {
        Hash(BSVHash::sha_512(input))
    }

    pub fn sha_512_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::sha_256d_hmac(input, key))
    }

    pub fn sha_256_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::sha_256d_hmac(input, key))
    }

    pub fn sha_256d_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::sha_256d_hmac(input, key))
    }

    pub fn sha_1_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::sha_1_hmac(input, key))
    }

    pub fn ripemd_160_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::ripemd_160_hmac(input, key))
    }

    pub fn hash_160_hmac(input: &[u8], key: &[u8]) -> Self {
        Self(BSVHash::hash_160_hmac(input, key))
    }
}
