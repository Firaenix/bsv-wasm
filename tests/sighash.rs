#[cfg(test)]
mod sighash_tests {
    use std::convert::{TryFrom, TryInto};

    use bsv::*;

    #[test]
    fn sighash_inputs_output_single() {
        let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx").unwrap();
        let sighash = SigHash::InputsOutput;
        let signing_script = Script::from_asm_string("OP_0 OP_RETURN").unwrap();
        let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000").unwrap();

        let sighash_buffer = tx.sighash_preimage(sighash, 0, &signing_script, 0).unwrap();
        assert_eq!(sighash_buffer.to_hex(), "010000008bf38a2d3f477a28aba2fe171260ffb0315c7371617ba6e39aea4ed97558c35800000000000000000000000000000000000000000000000000000000000000009e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006a0000000000000000ffffffffc7732d98e887792b43e5dae92a159010d22e47d60ed48b88ba7b6c12a3c9e7560000000043000000", "Sighash preimages did not match");

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0, false).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "3044022039b24bbc11cebd00c62a656cb2e04f740cfdbb43c8eb3fbd900f35937f7bdaf402205377f652da661086d08565a5656bb39fb366b551e918a69d71496a2c711225c443"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig, false));
    }

    #[test]
    fn sighash_none_anyonecanpay_no_fork_id() {
        let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx").unwrap();
        let sighash = SigHash::NONE | SigHash::ANYONECANPAY;
        let signing_script = Script::from_asm_string("OP_0 OP_RETURN").unwrap();
        let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000").unwrap();

        let sighash_buffer = tx.sighash_preimage(sighash.try_into().unwrap(), 0, &signing_script, 0).unwrap();
        assert_eq!(
            sighash_buffer.to_hex(),
            "01000000019e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006affffffff000000000082000000"
        );

        let sig = tx.sign(&priv_key, SigHash::try_from(sighash).unwrap(), 0, &signing_script, 0, false).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "30440220508cd9b741d1c5af04849ecff9690e9d0490c4d8472c17883034305b6103f486022066451831cb7b07353c62ae50abd5a34b033bee8e1f1bcd591aca3304b39cbf2682"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig, false));
    }

    #[test]
    fn sighash_none_anyonecanpay_fork_id() {
        let signing_script = Script::from_asm_string("OP_CHECKSIG").unwrap();
        let mut tx = Transaction::new(1, 0);
        tx.add_input(&TxIn::new(
            &[1u8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            0,
            &signing_script,
            None,
        ));
        // This is used to set the hashcache and make sure it clears properly on alternative signing types
        let _sighash_buffer_all = tx.sighash_preimage(SigHash::InputsOutputs, 0, &signing_script, 1337).unwrap();

        let sighash_buffer = tx.sighash_preimage(SigHash::Input, 0, &signing_script, 1337).unwrap();

        assert_eq!(
            sighash_buffer.to_hex(),
            "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001010101010101010101010101010101010101010101010101010101010101010000000001ac3905000000000000ffffffff000000000000000000000000000000000000000000000000000000000000000000000000c2000000"
        );
    }

    #[test]
    fn sighash_none_no_fork_id() {
        let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx").unwrap();
        let sighash = SigHash::NONE;
        let signing_script = Script::from_asm_string("OP_0 OP_RETURN").unwrap();
        let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000").unwrap();

        let sighash_buffer = tx.sighash_preimage(sighash, 0, &signing_script, 0).unwrap();
        assert_eq!(sighash_buffer.to_hex(), "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006affffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000000000000000000000000002000000");

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0, false).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "30450221008bd853b7fb69731b843fd431da88f94ca93bb1484a7ca7e9ee667398d13c614902205d0454a335c3632104500eef948a5f7e86043ececa9f1dc594c5b36bfa446d1d02"
        );
        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig, false));
    }

    #[test]
    fn sighash_single_anyonecanpay_no_fork_id() {
        let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx").unwrap();
        let sighash = SigHash::SINGLE | SigHash::ANYONECANPAY;
        let signing_script = Script::from_asm_string("OP_0 OP_RETURN").unwrap();
        let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000").unwrap();

        let sighash_buffer = tx.sighash_preimage(sighash.try_into().unwrap(), 0, &signing_script, 0).unwrap();
        let desired_sighash =
            "01000000019e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006affffffff01206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac0000000083000000";
        let correct_tx = Transaction::from_hex(desired_sighash).unwrap();
        assert_eq!(Transaction::from_bytes(&sighash_buffer).unwrap(), correct_tx);
        assert_eq!(sighash_buffer.to_hex(), desired_sighash);

        let sig = tx.sign(&priv_key, SigHash::try_from(sighash).unwrap(), 0, &signing_script, 0, false).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "3045022100fb05590a5ce4fd0ff39149a1d473f5395f23b466f2ab3a14a99c7ee6a8a14af2022009f4d25c1c8ad80cf250e02af1211c8345694772fd1db4bafcdf32be4748a21b83"
        );
        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig, false));
    }

    #[test]
    fn sighash_single_no_fork_id() {
        let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx").unwrap();
        let sighash = SigHash::SINGLE;
        let signing_script = Script::from_asm_string("OP_0 OP_RETURN").unwrap();
        let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000").unwrap();

        let sighash_buffer = tx.sighash_preimage(sighash, 0, &signing_script, 0).unwrap();
        let desired_sighash = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006affffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f02000000000000000001206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac0000000003000000";
        let correct_tx = Transaction::from_hex(desired_sighash).unwrap();
        assert_eq!(Transaction::from_bytes(&sighash_buffer).unwrap(), correct_tx);
        assert_eq!(sighash_buffer.to_hex(), desired_sighash);

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0, false).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "3045022100d76b13177c292d81c0847c366d3dfbbaa07567dd7fbaf78b3d524afb4b99626d02204ac6476d957cb846e0fe9a6a8223add3d76e439282562b80c7b84b3fa32a2b2203"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig, false));
    }
}
