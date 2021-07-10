#[cfg(test)]
mod tests {
    use bsv_wasm::*;
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn import_signature() {
        let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
        let sig = Signature::from_der(hex::decode(sig_hex).unwrap(), false).unwrap();
        assert_eq!(sig.to_hex(), sig_hex)
    }

    #[test]
    #[wasm_bindgen_test]
    fn import_signature_string() {
        let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
        let sig = Signature::from_hex_der(sig_hex.into(), false).unwrap();
        assert_eq!(sig.to_hex(), sig_hex)
    }

    // #[test]
    // #[wasm_bindgen_test]
    // fn der_signature_test_s_r() {
    //   let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
    //   let sig = Signature::from_hex_der(sig_hex.into()).unwrap();

    //   let verified = sig.verify();

    //   assert_eq!(sig.to_hex(), sig_hex)
    // }

    #[test]
    #[wasm_bindgen_test]
    fn sign_message() {
        let wif = "L5EZftvrYaSudiozVRzTqLcHLNDoVn7H5HSfM9BAN6tMJX8oTWz6";

        let key = PrivateKey::from_wif(wif.into()).unwrap();
        let message = b"Hello";

        let signature = key.sign_message(message).unwrap();
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&key, true);

        let is_verified = signature
            .verify(message.to_vec(), &pub_key, SigningHash::Sha256)
            .unwrap();
        assert_eq!(is_verified, true);
        assert_eq!(signature.to_hex(), "3045022100fab965a4dd445c990f46689f7acdc6e089128dc2d743457b350032d66336edb7022005f5684cc707b569120ef0442343998c95f6514c751251a91f82b1ec6a92da78".to_lowercase())
    }

    #[test]
    #[wasm_bindgen_test]
    fn recover_pub_key_from_signature_sha256d_reverse_k() {
        let key =
            PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32".into())
                .unwrap();

        let message = b"Hello";

        let signature = key
            .sign_with_k(message, SigningHash::Sha256d, true)
            .unwrap();
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&key, true);

        let is_verified = signature
            .verify(message.to_vec(), &pub_key, SigningHash::Sha256d)
            .unwrap();
        assert_eq!(is_verified, true);

        let recovered_pub_key = signature
            .recover_public_key(message.to_vec(), SigningHash::Sha256d)
            .unwrap();
        assert_eq!(
            pub_key.to_hex().unwrap(),
            recovered_pub_key.to_hex().unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn recover_pub_key_from_signature_sha256d() {
        let key =
            PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32".into())
                .unwrap();

        let message = b"Hello";

        let signature = key
            .sign_with_k(message, SigningHash::Sha256d, false)
            .unwrap();
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&key, true);

        let is_verified = signature
            .verify(message.to_vec(), &pub_key, SigningHash::Sha256d)
            .unwrap();
        assert_eq!(is_verified, true);

        let recovered_pub_key = signature
            .recover_public_key(message.to_vec(), SigningHash::Sha256d)
            .unwrap();
        assert_eq!(
            pub_key.to_hex().unwrap(),
            recovered_pub_key.to_hex().unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn recover_pub_key_from_signature_sha256() {
        let key =
            PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32".into())
                .unwrap();

        let message = b"Hello";

        let signature = key
            .sign_with_k(message, SigningHash::Sha256, false)
            .unwrap();
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&key, true);

        let is_verified = signature
            .verify(message.to_vec(), &pub_key, SigningHash::Sha256)
            .unwrap();
        assert_eq!(is_verified, true);

        let recovered_pub_key = signature
            .recover_public_key(message.to_vec(), SigningHash::Sha256)
            .unwrap();
        assert_eq!(
            pub_key.to_hex().unwrap(),
            recovered_pub_key.to_hex().unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn recover_pub_key_from_signature_sha256_reverse_k() {
        let key =
            PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32".into())
                .unwrap();

        let message = b"Hello";

        let signature = key.sign_with_k(message, SigningHash::Sha256, true).unwrap();
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&key, true);

        let is_verified = signature
            .verify(message.to_vec(), &pub_key, SigningHash::Sha256)
            .unwrap();
        assert_eq!(is_verified, true);

        let recovered_pub_key = signature
            .recover_public_key(message.to_vec(), SigningHash::Sha256)
            .unwrap();
        assert_eq!(
            pub_key.to_hex().unwrap(),
            recovered_pub_key.to_hex().unwrap()
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn to_compact_test() {
        let key =
            PrivateKey::from_wif("L4rGfRz3Q994Xns9wWti75K2CjxrCuzCqUAwN6yW7ia9nj4SDG32".into())
                .unwrap();

        let message = b"Hello";

        let signature = key
            .sign_with_k(message, SigningHash::Sha256, false)
            .unwrap();

        let compact_sig = signature.to_compact_bytes();
        let uncompacted_sig = Signature::from_compact_bytes(compact_sig).unwrap();

        assert_eq!(
            uncompacted_sig.to_compact_bytes(),
            signature.to_compact_bytes()
        );
        assert_eq!(uncompacted_sig.to_der_bytes(), signature.to_der_bytes());
    }
}
