

#[cfg(test)]
mod xpriv_tests {
    use bsv_wasm::{hash::Hash, keypair::*};
    extern crate wasm_bindgen_test;
    use rand_core::OsRng;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();


  #[test]
  #[wasm_bindgen_test]
  fn xpriv_parts_to_xpriv_string() {
    // let seed = "4a6b71c77d6d37b5b95b10659632e64341b985fadb37f55668b4de3f404473b47ec36bfebd30d9195ae4580f73c09dd492acab731eafd7f236e41e4818531368";
    let parent_chain_code = hex::decode("05aae71d7c080474efaab01fa79e96f4c6cfe243237780b0df4bc36106228e31").unwrap();
    let private_key = PrivateKey::from_hex("39f329fedba2a68e2a804fcd9aeea4104ace9080212a52ce8b52c1fb89850c72".to_string()).unwrap();
    let parent_public_key = "0252c616d91a2488c1fd1f0f172e98f7d1f6e51f8f389b2f8d632a8b490d5f6da9";
    let parent_fingerprint = &Hash::hash_160(&hex::decode(parent_public_key).unwrap()).to_bytes()[0..4];

    let key = ExtendedPrivateKey::new(&private_key, parent_chain_code.as_slice(), &1, &0, Some(parent_fingerprint));
    
    assert_eq!(key.to_string().unwrap(), "xprv9tuogRdb5YTgcL3P8Waj7REqDuQx4sXcodQaWTtEVFEp6yRKh1CjrWfXChnhgHeLDuXxo2auDZegMiVMGGxwxcrb2PmiGyCngLxvLeGsZRq")
  }

  #[test]
  #[wasm_bindgen_test]
  fn from_xprv_string() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K2rdSf96bvxvYtHYjf2899A7M7S3Ka2jASLK6P3hs7Bg9snGVsArqAA2awhc26e5kqKDquKSkpZ6hXymjpCcUj1tRi17L4Bg").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K2rdSf96bvxvYtHYjf2899A7M7S3Ka2jASLK6P3hs7Bg9snGVsArqAA2awhc26e5kqKDquKSkpZ6hXymjpCcUj1tRi17L4Bg")
  }

  #[test]
  #[wasm_bindgen_test]
  fn from_xprv_string_and_derive() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    // m/0/0
    let derived_key = key.derive(0).unwrap().derive(0).unwrap();

    assert_eq!(derived_key.get_private_key().to_wif(true).unwrap(), "L5LxK8WV9wNDemaYBtpEURWi3sHmGsEHpSGmSfahQrreTYKukp9W");
    assert_eq!(derived_key.get_public_key().to_hex().unwrap(), "02756de182c5dd4b717ea87e693006da62dbb3cddaa4a5cad2ed1f5bbab755f0f5");

    // m/0/0/12
    let second_derived = derived_key.derive(12).unwrap();
    assert_eq!(second_derived.get_private_key().to_wif(true).unwrap(), "KwX8mbobXJQ89SzPXHJ8fLGZ7ya6GyqAMLEZ8Cs2QWSm6GSAvQVg");
    assert_eq!(second_derived.get_public_key().to_hex().unwrap(), "02c815eb6b999ae9ac4edba5ca1b9a57723b4fa749afe3d3994462f0c4f2efc7dd");
  }

  #[test]
  #[wasm_bindgen_test]
  fn from_xprv_string_and_derive_hardened() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    // m/0'
    let derived_key = key.derive(HARDENED_KEY_OFFSET).unwrap();
    assert_eq!(derived_key.to_string().unwrap(), "xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7");

    // assert_eq!(derived_key.get_private_key().to_wif(true).unwrap(), "L5BmPijJjrKbiUfG4zbiFKNqkvuJ8usooJmzuD7Z8dkRoTThYnAT");
    // assert_eq!(derived_key.get_public_key().to_hex().unwrap(), "035a784662a4a20a65bf6aab9ae98a6c068a81c52e4b032c0fb5400c706cfccc56");

    // m/0'/12'
    let second_derived = derived_key.derive(12 + HARDENED_KEY_OFFSET).unwrap();
    assert_eq!(second_derived.to_string().unwrap(), "xprv9wTYmMFmpgaLi3HHQHhv5tzwSbXyrwdbm6PiYgTrgu1D931Q81Doi4RhbEoxbLvBYs7f7Foq4tTU8UxTHy6yD4TYgrm8ttEiWLiwSm66akB");
    // assert_eq!(second_derived.get_private_key().to_wif(true).unwrap(), "KxUAqUXuB3Ksh3QwnorUhATf2bNY6CPjD3dv8EeTXdVeQF8RYQpL");
    // assert_eq!(second_derived.get_public_key().to_hex().unwrap(), "03c188374826dc4a986adf53b01d1eb5ca4bf37f0c6ceea63cd6e350a56883b369");
  }

  #[test]
  #[wasm_bindgen_test]
  fn from_xprv_string_and_derive_from_path() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    // m/0'
    let derived_key = key.derive_from_path("m/0'/12'").unwrap();
    assert_eq!(derived_key.get_private_key().to_wif(true).unwrap(), "KxUAqUXuB3Ksh3QwnorUhATf2bNY6CPjD3dv8EeTXdVeQF8RYQpL");
    assert_eq!(derived_key.get_public_key().to_hex().unwrap(), "03c188374826dc4a986adf53b01d1eb5ca4bf37f0c6ceea63cd6e350a56883b369");
  }

  #[test]
  fn from_xprv_string_and_invalid_path() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    assert_eq!(key.derive_from_path("0'/12'").is_err(), true);
    assert_eq!(key.derive_from_path("m/0'/2222222222'").is_err(), true);
  }

  #[test]
  fn from_xprv_string_and_path_only_contains_m() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    assert_eq!(key.derive_from_path("m").is_err(), true);
    assert_eq!(key.derive_from_path("m/").is_err(), true);
  }

  #[test]
  #[wasm_bindgen_test]
  fn seed_to_xprv() {
    let bytes = hex::decode("c3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355c").unwrap();
    let key = ExtendedPrivateKey::from_seed(bytes).unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3UHKxrxdUicfntW2v9RRm2YGc1Fvs44cNMLfrXERKaSzziw6qVMcN6EFiT2uEYDNgSxopNFEGBjBZkHXRWQkTe8ePpAviwT");
  }

  #[test]
  #[wasm_bindgen_test]
  fn non_standard__long_seed_to_xprv() {
    let bytes = hex::decode("c3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355cc3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355cc3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355cc3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355cc3cbf33f1b8f404fec9c6779b2b89d9fa08d0ecfc2c66bf679cbdbe6b8630fdd849911514fd01fef00d26dbb9290a4ea311765b8a5ed004a85af7253f0b1355c").unwrap();
    let key = ExtendedPrivateKey::from_seed(bytes).unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K2iZ72grdBYHPdmnrwYwmUHQkSpW6hnVCcYKVMr2V5A7AYDerUumeMVaZ98FAY5viaMkJ6a1bU3s5aHYbyErWotxAPBMQx4m");
  }

  #[test]
  #[wasm_bindgen_test]
  fn non_standard__massive_seed_to_xpriv() {
    let mut seed = vec![0; 4096];
    getrandom::getrandom(&mut seed).unwrap();
    let key = ExtendedPrivateKey::from_seed(seed).unwrap();

    let new_key = ExtendedPrivateKey::from_string(&key.to_string().unwrap()).unwrap();
    assert_eq!(new_key.to_string().unwrap(), key.to_string().unwrap());
  }

  #[test]
  fn non_standard__short_seed_to_xprv() {
    let bytes = hex::decode("deadbeef").unwrap();
    let key = ExtendedPrivateKey::from_seed(bytes).unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3W67FQwsDcvjkiQ6NJaKxBL4bSpATvZ1DtNrJQAfJYZX3Dscv5GFu6oaWTsn5DDD335wopZqLs1QYCGMLyF4c8xcc12URrY");
  }

  #[test]
  fn from_mnemonic_xprv() {
    let mnemonic = "vapor cabbage jacket unveil permit web live pyramid husband final plug metal";
    let key = ExtendedPrivateKey::from_mnemonic(mnemonic.as_bytes().to_vec(), None).unwrap();
    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3kV5ByEVyeoaC6TbWS9T3UrQamHwMgpbTghuLXUfiSgeK1TRr1K9xWVcJKdtQawEM1RGwAfCzwPHJXSCEzTSze7ZnduyQaU");
  }
}

