use crate::hash::Hash;
use bsv::PBKDF2Hashes as BSVPBKDF2Hashes;
use bsv::KDF as BSVKDF;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum PBKDF2Hashes {
    SHA1,
    SHA256,
    SHA512,
}

impl Into<BSVPBKDF2Hashes> for PBKDF2Hashes {
    fn into(self) -> bsv::PBKDF2Hashes {
        match self {
            PBKDF2Hashes::SHA1 => BSVPBKDF2Hashes::SHA1,
            PBKDF2Hashes::SHA256 => BSVPBKDF2Hashes::SHA256,
            PBKDF2Hashes::SHA512 => BSVPBKDF2Hashes::SHA512,
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
