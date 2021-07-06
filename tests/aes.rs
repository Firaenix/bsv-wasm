#[cfg(test)]
mod aes_tests {
    use std::io::Read;

    use bsv_wasm::{AES, encryption::AESAlgorithms, hash::Hash};
    use rand_core::{OsRng, RngCore};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_128_cbc() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..16];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt(&key, &iv, message, AESAlgorithms::AES128_CBC).unwrap();
    
    assert_eq!(encrypted, [92, 179, 8, 184, 112, 133, 174, 231, 150, 247, 104, 190, 12, 208, 117, 210]);

    let decrypted = AES::decrypt(&key, &iv, &encrypted, AESAlgorithms::AES128_CBC).unwrap();
    assert_eq!(decrypted, message)
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_256_cbc() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..32];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt(&key, &iv, message, AESAlgorithms::AES256_CBC).unwrap();
    
    assert_eq!(encrypted, [39, 197, 167, 70, 112, 186, 22, 36, 107, 166, 185, 246, 5, 88, 8, 119]);

    let decrypted = AES::decrypt(&key, &iv, &encrypted, AESAlgorithms::AES256_CBC).unwrap();
    assert_eq!(decrypted, message)
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_128_ctr() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..16];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt(&key, &iv, message, AESAlgorithms::AES128_CTR).unwrap();
    
    assert_eq!(encrypted, [225, 100, 196, 107, 4, 133, 109, 174, 65, 202, 145, 203], "Encrypted message doesnt match");

    let decrypted = AES::decrypt(&key, &iv, &encrypted, AESAlgorithms::AES128_CTR).unwrap();
    assert_eq!(decrypted, message)
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_256_ctr() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..32];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt(&key, &iv, message, AESAlgorithms::AES256_CTR).unwrap();
    
    assert_eq!(encrypted, [32, 173, 59, 177, 23, 255, 144, 247, 130, 230, 95, 215], "Encrypted message doesnt match");

    let decrypted = AES::decrypt(&key, &iv, &encrypted, AESAlgorithms::AES256_CTR).unwrap();
    assert_eq!(decrypted, message)
  }

}
