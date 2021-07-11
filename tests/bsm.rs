#[cfg(test)]
mod bitcoin_signed_message_tests {
    use anyhow::*;
    use bsv_wasm::{encryption::AESAlgorithms, hash::Hash, AES};
    use bsv_wasm::{PrivateKey, BSM, ECDSA};
    use rand_core::{OsRng, RngCore};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn sign_and_verify_message() {
        let priv_key = PrivateKey::from_wif("L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC".into()).unwrap();

        let message = b"Hello Bitcoin!";

        let msg_sig = priv_key.sign_message(message).unwrap();
        assert_eq!(msg_sig.verify_message(message.to_vec(), &priv_key.get_public_key().unwrap()), true, "Normal messages match");

        let signature = BSM::sign_message(&priv_key, message).unwrap();

        let verified = BSM::verify_message(message, &signature, &priv_key.get_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

        assert_eq!(verified, true, "Message is not verified")
    }
}
