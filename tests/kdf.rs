#[cfg(test)]
mod kdf_tests {
    extern crate wasm_bindgen_test;
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

    let kdf = KDF::pbkdf2(&password, Some(salt.into()), bsv_wasm::PBKDF2Hashes::SHA256, rounds, 32).unwrap();

    // validated against twetch/sycamore-pro and https://neurotechnics.com/tools/pbkdf2-test
    assert_eq!(kdf.get_hash().to_hex(), "ffb5bb1b78211b1d275f32c4ba426f0875e80640fbf313eac06ba6e79225b237");
  }
}
