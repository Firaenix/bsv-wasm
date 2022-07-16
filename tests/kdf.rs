#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod kdf_tests {
    use bsv_wasm::{hash::Hash, KDF};
    use pbkdf2::{
        password_hash::{Ident, PasswordHasher, Salt, SaltString},
        Params, Pbkdf2,
    };
    use std::str::from_utf8;
    
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn pbkdf2_sha256_hash_test() {
        let password = "stronk-password".as_bytes();
        let salt = "snails".as_bytes();
        let rounds: u32 = 10000;

        let kdf = KDF::pbkdf2(password, Some(salt.into()), bsv_wasm::PBKDF2Hashes::SHA256, rounds, 32);

        // validated against twetch/sycamore-pro and https://neurotechnics.com/tools/pbkdf2-test
        assert_eq!(kdf.get_hash().to_hex(), "ffb5bb1b78211b1d275f32c4ba426f0875e80640fbf313eac06ba6e79225b237");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn pbkdf2_sha256_hash_test_2() {
        let password = "stronk-password".as_bytes();
        let salt = "1ae0ee429ffca864413b59edd5612c1a86b097411280a6dfa376d91c6eba5a20".as_bytes(); // sha256 of debug@twetch.com
        let rounds: u32 = 10000;

        let kdf = KDF::pbkdf2(password, Some(salt.into()), bsv_wasm::PBKDF2Hashes::SHA256, rounds, 32);

        // validated against twetch/sycamore-pro and https://neurotechnics.com/tools/pbkdf2-test
        assert_eq!(kdf.get_hash().to_hex(), "f064d740b65941152755829e2b48578b259bc9bfc8c3af7b0d93a5ca677f259d");
    }
}
