#[cfg(test)]
mod bitcoin_signed_message_tests {
    use bsv::{PrivateKey, Signature, BSM};

    #[test]
    fn sign_and_verify_message() {
        let priv_key = PrivateKey::from_wif("L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC").unwrap();

        let message = b"Hello Bitcoin!";

        let msg_sig = priv_key.sign_message(message).unwrap();
        let is_std_verified = msg_sig.verify_message(message, &priv_key.to_public_key().unwrap());
        assert!(is_std_verified, "Normal messages match");

        let signature = BSM::sign_message(&priv_key, message).unwrap();

        let verified = BSM::verify_message(message, &signature, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

        assert!(verified, "Message is not verified")
    }

    #[test]
    fn rehydrate_signature_and_verify() {
        for _ in 0..100 {
            let priv_key = PrivateKey::from_random();

            let message = b"Hello Bitcoin!";

            let msg_sig = priv_key.sign_message(message).unwrap();
            assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

            let signature = BSM::sign_message(&priv_key, message).unwrap();

            let bsm_sig_hex = signature.to_compact_bytes(None);

            let rehydrated_bsm_sig = Signature::from_compact_bytes(&bsm_sig_hex).unwrap();

            let verified = BSM::verify_message(message, &rehydrated_bsm_sig, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

            assert!(verified, "Message is not verified");
        }
    }

    #[test]
    fn rehydrate_compact_signature_and_verify() {
        for _ in 0..100 {
            let priv_key = PrivateKey::from_random();

            let message = b"Hello Bitcoin!";

            let msg_sig = priv_key.sign_message(message).unwrap();
            assert!(msg_sig.verify_message(message, &priv_key.to_public_key().unwrap()), "Normal messages match");

            let signature = BSM::sign_message(&priv_key, message).unwrap();

            let bsm_compact = signature.to_compact_bytes(None);

            let rehydrated_bsm_sig = Signature::from_compact_bytes(&bsm_compact).unwrap();

            let verified = BSM::verify_message(message, &rehydrated_bsm_sig, &priv_key.to_public_key().unwrap().to_p2pkh_address().unwrap()).unwrap();

            assert!(verified, "Message is not verified");
        }
    }
}
