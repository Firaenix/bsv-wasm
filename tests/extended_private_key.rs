

#[cfg(test)]
mod xpriv_tests {
    use bsv_wasm::{P2PKHAddress, hash::Hash, keypair::*};
    extern crate wasm_bindgen_test;
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
    let derived_key = key.derive(2147483648).unwrap();

    assert_eq!(derived_key.get_private_key().to_wif(true).unwrap(), "L5BmPijJjrKbiUfG4zbiFKNqkvuJ8usooJmzuD7Z8dkRoTThYnAT");
    assert_eq!(derived_key.get_public_key().to_hex().unwrap(), "035a784662a4a20a65bf6aab9ae98a6c068a81c52e4b032c0fb5400c706cfccc56");

    // // m/0'/12'
    let second_derived = derived_key.derive(12 + 2147483648).unwrap();
    assert_eq!(second_derived.get_private_key().to_wif(true).unwrap(), "KxUAqUXuB3Ksh3QwnorUhATf2bNY6CPjD3dv8EeTXdVeQF8RYQpL");
    assert_eq!(second_derived.get_public_key().to_hex().unwrap(), "03c188374826dc4a986adf53b01d1eb5ca4bf37f0c6ceea63cd6e350a56883b369");
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
  #[wasm_bindgen_test]
  fn from_xprv_string_and_invalid_path() {
    let key = ExtendedPrivateKey::from_string("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi").unwrap();

    assert_eq!(key.to_string().unwrap(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

    let derived_key = key.derive_from_path("0'/12'");
    assert_eq!(derived_key.is_err(), true);
  }
}

