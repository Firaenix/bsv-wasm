#[cfg(test)]
mod bitcoin_signed_message_tests {

    use std::fmt::Error;

    use bsv_wasm::{BSVErrors, PrivateKey, Signature, BSM};

    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn sign_and_verify_message() {
        let priv_key = PrivateKey::from_wif("L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC").unwrap();

        let message = b"Hello Bitcoin!";

        let msg_sig = priv_key.sign_message(message).unwrap();
        assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

        let signature = BSM::sign_message(&priv_key, message).unwrap();

        let verified = BSM::verify_message(message, &signature, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

        assert!(verified, "Message is not verified")
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn rehydrate_signature_and_verify() -> Result<(), BSVErrors> {
        for _ in 0..100 {
            let priv_key = PrivateKey::from_random();

            let message = b"Hello Bitcoin!";

            let msg_sig = priv_key.sign_message(message).unwrap();
            assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

            let signature = BSM::sign_message(&priv_key, message).unwrap();

            let bsm_sig_hex = signature.to_hex();

            let rehydrated_bsm_sig = Signature::from_hex_der(&bsm_sig_hex, true)?;

            let verified = BSM::verify_message(message, &rehydrated_bsm_sig, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

            assert!(verified, "Message is not verified");
        }

        Ok(())
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn rehydrate_compact_signature_and_verify() -> Result<(), BSVErrors> {
        for _ in 0..100 {
            let priv_key = PrivateKey::from_random();

            let message = b"Hello Bitcoin!";

            let msg_sig = priv_key.sign_message(message).unwrap();
            assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

            let signature = BSM::sign_message(&priv_key, message)?;

            let bsm_compact = signature.to_compact_bytes();

            let rehydrated_bsm_sig = Signature::from_compact_bytes(&bsm_compact)?;

            let verified = BSM::verify_message(message, &rehydrated_bsm_sig, &priv_key.to_public_key()?.to_p2pkh_address()?)?;

            assert!(verified, "Message is not verified");
        }

        Ok(())
    }
}
