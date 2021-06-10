#[cfg(test)]
mod aes_tests {
    use bsv_wasm::{AES, hash::Hash};
    use rand_core::{OsRng, RngCore};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();


  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_128() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..16];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt_128(&key, &iv, message).unwrap();
    
    assert_eq!(encrypted, [92, 179, 8, 184, 112, 133, 174, 231, 150, 247, 104, 190, 12, 208, 117, 210]);

    let decrypted = AES::decrypt_128(&key, &iv, &encrypted).unwrap();
    assert_eq!(decrypted, message)
    // assert_eq!(pub_key.to_string().unwrap(), "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu");
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_aes_256() {
    let key = &Hash::sha_256(hex::encode("Key Please").as_bytes()).to_bytes()[0..32];
    let iv = &Hash::sha_256(hex::encode("IV Please").as_bytes()).to_bytes()[0..16];

    let message = b"Hello world!";

    let encrypted = AES::encrypt_256(&key, &iv, message).unwrap();
    
    assert_eq!(encrypted, [39, 197, 167, 70, 112, 186, 22, 36, 107, 166, 185, 246, 5, 88, 8, 119]);

    let decrypted = AES::decrypt_256(&key, &iv, &encrypted).unwrap();
    assert_eq!(decrypted, message);
  }
}