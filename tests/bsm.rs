#[cfg(test)]
mod bitcoin_signed_message_tests {

    use bsv_wasm::{PrivateKey, BSM};

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn sign_and_verify_message() {
        let priv_key = PrivateKey::from_wif("L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC").unwrap();

        let message = b"Hello Bitcoin!";

        let msg_sig = priv_key.sign_message(message).unwrap();
        assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

        let signature = BSM::sign_message(&priv_key, message).unwrap();

        let verified = BSM::verify_message(message, &signature, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

        assert!(verified, "Message is not verified")
    }
}
