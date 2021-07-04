#[cfg(test)]
mod kdf_tests {
    extern crate wasm_bindgen_test;
    use std::str::from_utf8;

    use bitcoin_hashes::hex::ToHex;
    use bsv_wasm::{KDF, hash::Hash};
    use pbkdf2::{Params, Pbkdf2, password_hash::{Ident, PasswordHasher, Salt, SaltString}};
    use wasm_bindgen_test::*;
    use anyhow::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn pbkdf2_sha256_hash_test() {
    let password = "stronk-password";
    let salt = "snails";
    let rounds: u32 = 10000;

    let kdf = KDF::pbkdf2(password.as_bytes(), Some(salt.as_bytes()), bsv_wasm::PBKDF2Hashes::SHA256, rounds, 32).unwrap();

    // validated against twetch/sycamore-pro and https://neurotechnics.com/tools/pbkdf2-test
    assert_eq!(kdf.get_hash().to_hex(), "ffb5bb1b78211b1d275f32c4ba426f0875e80640fbf313eac06ba6e79225b237");
  }

  #[test]
  #[wasm_bindgen_test]
  fn pbkdf2_sha256_hash_test_2() {
    let password = "stronk-password";
    let salt = Hash::sha_256("debug@twetch.com".as_bytes()).to_bytes(); // "1ae0ee429ffca864413b59edd5612c1a86b097411280a6dfa376d91c6eba5a20"; // sha256 of debug@twetch.com
    let rounds: u32 = 10000;

    let kdf = KDF::pbkdf2(password.as_bytes(), Some(&salt), bsv_wasm::PBKDF2Hashes::SHA256, rounds, 32).unwrap();

    // validated against twetch/sycamore-pro and https://neurotechnics.com/tools/pbkdf2-test
    assert_eq!(kdf.get_hash().to_hex(), "f064d740b65941152755829e2b48578b259bc9bfc8c3af7b0d93a5ca677f259d");
  }
}
