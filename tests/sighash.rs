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

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "30440220798bd19a0bb1fd5e1b3832e46ae69af687d87cbd179a81e60af719382860aee502206d849665f4010a54f40d9ac463629cbd758042745cebf7122818d9bfea2bce7043"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig));
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

        let sig = tx.sign(&priv_key, SigHash::try_from(sighash).unwrap(), 0, &signing_script, 0).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "3045022100ff2ff02e9b30a6c8079ee90d24ce1e231a5e861fa7517ae8389aeddb9b7c499102202c7deb7a0a5f08dcd1179cc03098b45454d8f704589dd600632a17851a8daddf82"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig));
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

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "304402200a154c40134341fba55cc0d30753de8a72559f196492976dd702d40babc53f8502200bd904c0b73ea8bffd2b12d381450879f25105be85710dd056aa872afb897cd102"
        );
        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig));
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

        let sig = tx.sign(&priv_key, SigHash::try_from(sighash).unwrap(), 0, &signing_script, 0).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "30450221008caaf83578ffa42caa22bf15b54e7bbf51645566f41e60de57e771c57752e1cb02204e22fe4a88d98c601e9b52ff05a8d955a24baaaae197c532ed2391149885822483"
        );
        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig));
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

        let sig = tx.sign(&priv_key, sighash, 0, &signing_script, 0).unwrap();
        assert_eq!(
            sig.to_hex().unwrap(),
            "3045022100e8789a42d6a124434a52b583404f5478be526a325a54e1ebd438a48eb30d007a02206aa8339ecaaf217ef2d14677504d71b89fc4a3ede2003facae1f2d62d95f9c7a03"
        );

        assert!(tx.verify(&PublicKey::from_private_key(&priv_key), &sig));
    }
}
