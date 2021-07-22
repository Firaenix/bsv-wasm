#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod ecies_tests {
  use bsv_wasm::{PrivateKey, ECIES};
  use std::io::Read;
  use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_decrypt_text_ephemeral_private_key() {
    // Recipient & Sender
    let bob_priv_key = PrivateKey::from_random();
    let bob_pub_key = bob_priv_key.get_public_key().unwrap();
    let message = b"Hello, Bitcoin.";

    let encrypted = ECIES::encrypt(message, None, &bob_pub_key).unwrap();

    assert!(encrypted.len() > 0);

    let plaintext = ECIES::decrypt(&encrypted, &bob_priv_key, None).unwrap();

    assert_eq!(plaintext, message);
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_text_specific_private_key() {
    // Sender
    let alice_private_key = PrivateKey::from_random();

    // Recipient
    let bob_priv_key = PrivateKey::from_random();
    let bob_pub_key = bob_priv_key.get_public_key().unwrap();
    let message = b"Hello, Bitcoin.";

    let encrypted = ECIES::encrypt(message, Some(alice_private_key.clone()), &bob_pub_key).unwrap();

    assert!(encrypted.len() > 0);

    let plaintext = ECIES::decrypt(&encrypted, &bob_priv_key, Some(alice_private_key.get_public_key().unwrap())).unwrap();

    assert_eq!(plaintext, message);
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_text_specific_private_key_convenience_method() {
    // Sender
    let alice_private_key = PrivateKey::from_random();
    let alice_pub_key = alice_private_key.get_public_key().unwrap();

    // Recipient
    let bob_priv_key = PrivateKey::from_random();
    let bob_pub_key = bob_priv_key.get_public_key().unwrap();
    let message = b"Hello, Bitcoin.";

    let encrypted = alice_private_key.encrypt_message(message, Some(bob_pub_key)).unwrap();

    assert!(encrypted.len() > 0);

    let plaintext = bob_priv_key.decrypt_message(&encrypted, Some(alice_pub_key)).unwrap();

    assert_eq!(plaintext, message);
  }

  #[test]
  #[wasm_bindgen_test]
  fn encrypt_text_specific_private_key_convenience_method_send_to_self() {
    // Sender
    let alice_private_key = PrivateKey::from_random();
    let alice_pub_key = alice_private_key.get_public_key().unwrap();

    let message = b"Hello, Bitcoin.";

    let encrypted = alice_private_key.encrypt_message(message, None).unwrap();

    assert!(encrypted.len() > 0);

    let plaintext = alice_private_key.decrypt_message(&encrypted, None).unwrap();

    assert_eq!(plaintext, message);
  }
}
