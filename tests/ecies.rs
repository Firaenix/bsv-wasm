#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod ecies_tests {
    use bsv_wasm::{PrivateKey, ECIES};
    use std::io::Read;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    // Send to other party without encoding public key
    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_to_other_party_and_exclude_pub_key() {
        // Sender
        let alice_private_key = PrivateKey::from_random();

        // Recipient
        let bob_priv_key = PrivateKey::from_random();
        let bob_pub_key = bob_priv_key.get_public_key().unwrap();
        let message = b"Hello, Bitcoin.";

        let encrypted = ECIES::encrypt(message, &alice_private_key, &bob_pub_key, true).unwrap();

        let plaintext = ECIES::decrypt(&encrypted, &bob_priv_key, &alice_private_key.get_public_key().unwrap()).unwrap();

        assert_eq!(plaintext, message);
    }

    // Send to self without encoding public key
    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_to_self_and_exclude_pub_key() {
        // Sender
        let alice_private_key = PrivateKey::from_random();
        let alice_pub_key = alice_private_key.get_public_key().unwrap();

        let message = b"Hello, Bitcoin.";

        let encrypted = ECIES::encrypt(message, &alice_private_key, &alice_pub_key, true).unwrap();

        let plaintext = ECIES::decrypt(&encrypted, &alice_private_key, &alice_pub_key).unwrap();

        assert_eq!(plaintext, message);
    }

    // Send to self tests
    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_specific_private_key_send_to_self() {
        // Sender
        let alice_private_key = PrivateKey::from_random();
        let alice_pub_key = alice_private_key.get_public_key().unwrap();

        let message = b"Hello, Bitcoin.";

        let encrypted = ECIES::encrypt(message, &alice_private_key, &alice_pub_key, false).unwrap();

        let plaintext = ECIES::decrypt(&encrypted, &alice_private_key, &alice_pub_key).unwrap();

        assert_eq!(plaintext, message);
    }

    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_specific_private_key_convenience_method_send_to_self() {
        // Sender
        let alice_private_key = PrivateKey::from_random();
        let _alice_pub_key = alice_private_key.get_public_key().unwrap();

        let message = b"Hello, Bitcoin.";

        let encrypted = alice_private_key.encrypt_message(message).unwrap();

        let plaintext = alice_private_key.decrypt_message(&encrypted, &alice_private_key.get_public_key().unwrap()).unwrap();

        assert_eq!(plaintext, message);
    }

    // // Send to other party tests
    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_specific_private_key() {
        // Sender
        let alice_private_key = PrivateKey::from_random();

        // Recipient
        let bob_priv_key = PrivateKey::from_random();
        let bob_pub_key = bob_priv_key.get_public_key().unwrap();
        let message = b"Hello, Bitcoin.";

        let encrypted = ECIES::encrypt(message, &alice_private_key, &bob_pub_key, false).unwrap();

        let plaintext = ECIES::decrypt(&encrypted, &bob_priv_key, &alice_private_key.get_public_key().unwrap()).unwrap();

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

        let encrypted = bob_pub_key.encrypt_message(message, &alice_private_key).unwrap();

        let plaintext = bob_priv_key.decrypt_message(&encrypted, &alice_pub_key).unwrap();

        assert_eq!(plaintext, message);
    }

    // Send to other party with ephemeral(anonymous) private key
    #[test]
    #[wasm_bindgen_test]
    fn encrypt_decrypt_text_ephemeral_private_key() {
        // Recipient with Anonymous sender
        let bob_priv_key = PrivateKey::from_random();
        let bob_pub_key = bob_priv_key.get_public_key().unwrap();
        let message = b"Hello, Bitcoin.";

        let encrypted = ECIES::encrypt_with_ephemeral_private_key(message, &bob_pub_key, false).unwrap();

        let plaintext = ECIES::decrypt(&encrypted, &bob_priv_key, &encrypted.extract_public_key().unwrap()).unwrap();

        assert_eq!(plaintext, message);
    }
}
