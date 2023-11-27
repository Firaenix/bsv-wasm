#[cfg(test)]
mod tests {
    use bsv::*;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn import_signature() {
        let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
        let sig = Signature::from_der(&hex::decode(sig_hex).unwrap()).unwrap();
        assert_eq!(sig.to_der_hex(), sig_hex)
    }

    #[test]
    fn import_signature_string() {
        let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
        let sig = Signature::from_hex_der(sig_hex).unwrap();
        assert_eq!(sig.to_der_hex(), sig_hex)
    }

    #[test]
    fn import_signature_with_sighash_string() {
        let sig_hex = "304402205ebadbf09cf9b9be17ee6f588e93f490a2db9ac5966f938255282cca9ca75fa602206c37c1842e1b48a177c195e34579be84826b7ad919cda6d803a5fc1d77551580c3";

        assert!(Signature::from_hex_der(sig_hex).is_ok())
    }

    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn der_signature_test_s_r() {
    //   let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
    //   let sig = Signature::from_hex_der(sig_hex.into()).unwrap();

    //   let verified = sig.verify();

    //   assert_eq!(sig.to_hex(), sig_hex)
    // }

    #[test]
    fn sign_message() {
        let wif = "L5EZftvrYaSudiozVRzTqLcHLNDoVn7H5HSfM9BAN6tMJX8oTWz6";

        let key = PrivateKey::from_wif(wif).unwrap();
        let message = b"Hello";

        let signature = key.sign_message(message).unwrap();
        let pub_key = PublicKey::from_private_key(&key);

        let is_verified = signature.verify_message(message, &pub_key);
        assert!(is_verified);
        assert_eq!(
            signature.to_der_hex(),
            "3045022100fab965a4dd445c990f46689f7acdc6e089128dc2d743457b350032d66336edb7022005f5684cc707b569120ef0442343998c95f6514c751251a91f82b1ec6a92da78".to_lowercase()
        )
    }

    #[test]
    fn recover_pub_key_from_signature_sha256() {
        let key = PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32").unwrap();

        let message = b"Hello";

        let signature = key.sign_message(message).unwrap();
        let pub_key = PublicKey::from_private_key(&key);

        let is_verified = signature.verify_message(message, &pub_key);
        assert!(is_verified);

        let recovered_pub_key = signature.recover_public_key(message, SigningHash::Sha256).unwrap();
        assert_eq!(pub_key.to_hex().unwrap(), recovered_pub_key.to_hex().unwrap());
    }

    #[test]
    fn recover_pub_key_from_signature_sha256d() {
        let key = PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32").unwrap();
        let message = b"Hello";

        let signature = ECDSA::sign_with_deterministic_k(&key, message, SigningHash::Sha256d, false).unwrap();

        let pub_key = PublicKey::from_private_key(&key);
        let recovered_pub_key = signature.recover_public_key(message, SigningHash::Sha256d).unwrap();

        assert_eq!(pub_key.to_hex().unwrap(), recovered_pub_key.to_hex().unwrap());
    }

    #[test]
    fn recover_pub_key_from_signature_sha256d_reverse_k() {
        let key = PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32").unwrap();
        let message = b"Hello";

        let signature = ECDSA::sign_with_deterministic_k(&key, message, SigningHash::Sha256d, true).unwrap();

        let pub_key = PublicKey::from_private_key(&key);
        let recovered_pub_key = signature.recover_public_key(message, SigningHash::Sha256d).unwrap();

        assert_eq!(pub_key.to_hex().unwrap(), recovered_pub_key.to_hex().unwrap());
    }

    #[test]
    fn to_compact_test() {
        let key = PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32").unwrap();

        let message = b"Hello";

        let signature = key.sign_message(message).unwrap();

        let compact_sig = signature.to_compact_bytes(None);
        let uncompacted_sig = Signature::from_compact_bytes(&compact_sig).unwrap();

        assert_eq!(uncompacted_sig.to_compact_bytes(None), signature.to_compact_bytes(None));
        assert_eq!(uncompacted_sig.to_der_bytes(), signature.to_der_bytes());
    }

    #[test]
    fn sign_with_k_test_par() {
        (0..2180i32).into_par_iter().for_each(|_i| {
            let private_key = PrivateKey::from_random();
            let public_key = PublicKey::from_private_key(&private_key);
            let ephemeral_key = PrivateKey::from_random();
            let message = PrivateKey::from_random().to_bytes();
            let signature = ECDSA::sign_with_k(&private_key, &ephemeral_key, &message, SigningHash::Sha256d).unwrap();
            let private_key_recovered = ECDSA::private_key_from_signature_k(&signature, &public_key, &ephemeral_key, &message, SigningHash::Sha256d).unwrap();
            assert!(private_key_recovered.to_bytes() == private_key.to_bytes());
            if _i % 10000 == 0 {
                println!("{}", _i);
            }
        });
    }

    #[test]
    fn sign_with_k_test() {
        // TODO: Handle for extremely low private key/ephemeral key
        let private_key = PrivateKey::from_random();
        // let private_key = PrivateKey::from_wif("5HpHagT65TZzG1PH3CSu63k8DbpvD8s5ip4nEB3kEsreAnchuDf").unwrap();
        let public_key = PublicKey::from_private_key(&private_key);
        let ephemeral_key = PrivateKey::from_random();
        // let ephemeral_key = PrivateKey::from_wif("5HpHagT65TZzG1PH3CSu63k8DbpvD8s5ip4nEB3kEsreAnchuDf").unwrap();
        let message = b"Hello";
        let signature = ECDSA::sign_with_k(&private_key, &ephemeral_key, message, SigningHash::Sha256d).unwrap();
        let private_key_recovered = ECDSA::private_key_from_signature_k(&signature, &public_key, &ephemeral_key, message, SigningHash::Sha256d).unwrap();
        assert!(private_key_recovered.to_bytes() == private_key.to_bytes());
    }
}
