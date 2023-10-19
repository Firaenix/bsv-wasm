use crate::hash::Hash;
use bsv::KDF as BSVKDF;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PBKDF2Hashes {
    SHA1,
    SHA256,
    SHA512,
}

impl From<PBKDF2Hashes> for bsv::PBKDF2Hashes {
    fn from(item: PBKDF2Hashes) -> Self {
        match item {
            PBKDF2Hashes::SHA1 => bsv::PBKDF2Hashes::SHA1,
            PBKDF2Hashes::SHA256 => bsv::PBKDF2Hashes::SHA256,
            PBKDF2Hashes::SHA512 => bsv::PBKDF2Hashes::SHA512,
        }
    }
}

#[wasm_bindgen]
pub struct KDF(BSVKDF);

impl From<BSVKDF> for KDF {
    fn from(v: BSVKDF) -> KDF {
        KDF(v)
    }
}

#[wasm_bindgen]
impl KDF {
    pub fn get_hash(&self) -> Hash {
        Hash(self.0.get_hash())
    }

    pub fn get_salt(&self) -> Vec<u8> {
        self.0.get_salt()
    }

    /**
     * Implementation of PBKDF2 - when None is specified for salt, a random salt will be generated
     */
    pub fn pbkdf2(password: &[u8], salt: Option<Vec<u8>>, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> KDF {
        KDF(BSVKDF::pbkdf2(password, salt, hash_algo.into(), rounds, output_length))
    }
}
