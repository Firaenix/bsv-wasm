use crate::{hash::Hash, KDF};
use hmac::Hmac;
use pbkdf2::{
    password_hash::{Ident, PasswordHasher, Salt, SaltString},
    pbkdf2, Params, Pbkdf2,
};
use rand_core::OsRng;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{throw_str, JsValue};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, Copy)]
pub enum PBKDF2Hashes {
    SHA1,
    SHA256,
    SHA512,
}

impl KDF {
    /**
     *
     */
    pub fn pbkdf2_impl(password: &[u8], salt: &[u8], hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> KDF {
        let pbkdf2_fn = match hash_algo {
            PBKDF2Hashes::SHA1 => pbkdf2::<Hmac<Sha1>>,
            PBKDF2Hashes::SHA256 => pbkdf2::<Hmac<Sha256>>,
            PBKDF2Hashes::SHA512 => pbkdf2::<Hmac<Sha512>>,
        };
        let mut result = vec![0; output_length];
        pbkdf2_fn(password, salt, rounds, &mut result);

        KDF {
            hash: Hash(result),
            salt: salt.to_vec(),
        }
    }

    pub fn pbkdf2_random_salt_impl(password: &[u8], hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> KDF {
        let salt = SaltString::generate(&mut OsRng);
        KDF::pbkdf2_impl(password, salt.as_bytes(), hash_algo, rounds, output_length)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl KDF {
    /**
     * Implementation of PBKDF2 - when None is specified for salt, a random salt will be generated
     */
    pub fn pbkdf2(password: &[u8], salt: Option<Vec<u8>>, hash_algo: PBKDF2Hashes, rounds: u32, output_length: usize) -> KDF {
        match salt {
            Some(s) => KDF::pbkdf2_impl(password, &s, hash_algo, rounds, output_length),
            None => KDF::pbkdf2_random_salt_impl(password, hash_algo, rounds, output_length),
        }
    }
}
