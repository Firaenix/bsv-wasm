#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod transaction_tests {
    use bsv_wasm::*;
    extern crate wasm_bindgen_test;
    use bsv_wasm::TxIn;
    
    use wasm_bindgen::JsValue;
    
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialise_and_serialise_transaction_hex() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.get_size().unwrap(), 439);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0x02);

        let tx_in_0 = tx.get_input(0).unwrap();

        assert_eq!(tx_in_0.get_prev_tx_id(None), hex::decode("3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e").unwrap());
        assert_eq!(tx_in_0.get_vout(), 0);
        assert_eq!(tx_in_0.get_unlocking_script_size(), 0x8c);
        assert_eq!(tx_in_0.get_unlocking_script().to_bytes().len(), 0x8c);
        assert_eq!(tx_in_0.get_unlocking_script().to_bytes(), hex::decode("493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2").unwrap());
        assert_eq!(tx_in_0.get_sequence(), 4294967295);

        let tx_in_1 = tx.get_input(1).unwrap();
        assert_eq!(tx_in_1.get_vout(), 2);
        assert_eq!(tx_in_1.get_unlocking_script_size(), 0x8b);
        assert_eq!(tx_in_1.get_unlocking_script().to_bytes().len(), 0x8b);
        assert_eq!(tx_in_1.get_unlocking_script().to_bytes(), hex::decode("48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442").unwrap());
        assert_eq!(tx_in_1.get_sequence(), 4294967295);

        let tx_out_0 = tx.get_output(0).unwrap();
        assert_eq!(tx_out_0.get_satoshis(), 1076000);
        assert_eq!(tx_out_0.get_script_pub_key_size(), 25);
        assert_eq!(tx_out_0.get_script_pub_key(), Script::from_hex("76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac").unwrap());

        let tx_out_1 = tx.get_output(1).unwrap();
        assert_eq!(tx_out_1.get_satoshis(), 117488);
        assert_eq!(tx_out_1.get_script_pub_key_size(), 25);
        assert_eq!(tx_out_1.get_script_pub_key(), Script::from_hex("76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac").unwrap());

        let json_string = tx.to_json_string().unwrap();
        assert_eq!(json_string, "{\"version\":1,\"inputs\":[{\"prev_tx_id\":\"9e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f\",\"vout\":0,\"unlocking_script\":[\"3046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f2701\",\"04510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2\"],\"sequence\":4294967295},{\"prev_tx_id\":\"a3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f\",\"vout\":2,\"unlocking_script\":[\"304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c501\",\"042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442\"],\"sequence\":4294967295}],\"outputs\":[{\"value\":1076000,\"script_pub_key\":[\"OP_DUP\",\"OP_HASH160\",\"20bb5c3bfaef0231dc05190e7f1c8e22e098991e\",\"OP_EQUALVERIFY\",\"OP_CHECKSIG\"]},{\"value\":117488,\"script_pub_key\":[\"OP_DUP\",\"OP_HASH160\",\"9e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c\",\"OP_EQUALVERIFY\",\"OP_CHECKSIG\"]}],\"n_locktime\":0}");

        let rehydrated_tx = Transaction::from_json_string(&json_string).unwrap();

        assert_eq!(rehydrated_tx, tx, "Rehydrated JSON Tx doesnt match original Tx")
    }

    #[test]
    fn deserialise_transaction_hex_malformed() {
        let tx_hex = "FAKE01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex);

        assert!(tx.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialise_coinbase_hex() {
        let tx_hex = "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff63038d361604747a77610840000000230000004e2f686f77206c6f6e672063616e207468697320626520746573742074657374206170706172656e746c7920707265747479206c6f6e67206f6b20776f772031323334353637383930313220f09fa68d2f0000000001c817a804000000001976a91454b34b1ba228ba1d75dca5a40a114dc0f13a268788ac00000000";
        let tx = Transaction::from_hex(tx_hex).unwrap();
        assert!(tx.is_coinbase());
        assert_eq!(tx.to_hex().unwrap(), "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff63038d361604747a77610840000000230000004e2f686f77206c6f6e672063616e207468697320626520746573742074657374206170706172656e746c7920707265747479206c6f6e67206f6b20776f772031323334353637383930313220f09fa68d2f0000000001c817a804000000001976a91454b34b1ba228ba1d75dca5a40a114dc0f13a268788ac00000000");
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn deserialise_twetch_hex() {
        let expected_hex = "0200000000010000000000000000fdbf01006a2231394878696756345179427633744870515663554551797131707a5a56646f41757402676d0a746578742f706c61696e04746578741f7477657463685f7477746578745f313634373533323430393832322e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c02676d07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652464373935653432622d323462342d343938372d386562382d353665343733316637636237017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f454344534122314a4d6f6456736e376d7248643273646447645742566d324d4d70395173385651464c584834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d00000000".to_string();
        let script = Script::from_asm_string("0 OP_RETURN 31394878696756345179427633744870515663554551797131707a5a56646f417574 676d 746578742f706c61696e 74657874 7477657463685f7477746578745f313634373533323430393832322e747874 7c 3150755161374b36324d694b43747373534c4b79316b683536575755374d74555235 534554 7477646174615f6a736f6e 6e756c6c 75726c 676d 636f6d6d656e74 6e756c6c 6d625f75736572 6e756c6c 7265706c79 6e756c6c 74797065 706f7374 74696d657374616d70 6e756c6c 617070 747765746368 696e766f696365 64373935653432622d323462342d343938372d386562382d353665343733316637636237 7c 313550636948473232534e4c514a584d6f5355615756693757537163376843667661 424954434f494e5f4543445341 314a4d6f6456736e376d7248643273646447645742566d324d4d7039517338565146 4834384e5336544d7977496c6a527662615849743041546b4246514b2b5431444b2f504f562b496f59326d53616c6576526b3562644232424876654d5a736a64422f4b3333577071484d5037717a79493958626d332b773d").unwrap();
        let mut tx = Transaction::new(2, 0);
        let output = TxOut::new(0, &script);
        tx.add_output(&output);
        assert_eq!(expected_hex, tx.to_hex().unwrap());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn new_transaction() {
        let tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_input_to_transaction() {
        let mut tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);

        let input = TxIn::new(&[], 0, &Script::default(), None);

        tx.add_input(&input);
        assert_eq!(tx.get_ninputs(), 1);
        assert_eq!(tx.get_input(0), Some(input));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn add_output_to_transaction() {
        let mut tx = Transaction::new(1, 4);

        assert_eq!(tx.get_n_locktime(), 4);
        assert_eq!(tx.get_version(), 1);
        assert_eq!(tx.get_ninputs(), 0);
        assert_eq!(tx.get_input(0), None);
        assert_eq!(tx.get_noutputs(), 0);
        assert_eq!(tx.get_output(0), None);

        let output = TxOut::new(0, &Script::default());

        tx.add_output(&output);
        assert_eq!(tx.get_noutputs(), 1);
        assert_eq!(tx.get_output(0), Some(output));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txin_to_hex() {
        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin = TxIn::from_hex(txin_hex).unwrap();

        assert_eq!(txin.to_hex().unwrap(), txin_hex);
    }

    #[test]
    fn txin_to_hex_fail() {
        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txin = TxIn::from_hex(txin_hex);

        assert!(txin.is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txout_to_hex() {
        let txout_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout = TxOut::from_hex(txout_hex).unwrap();

        assert_eq!(txout.to_hex().unwrap(), txout_hex)
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn transaction_to_hex() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn add_txins_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_ninputs(), 2);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin1 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin1.to_hex().unwrap(), txin_hex);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin2 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin2.to_hex().unwrap(), txin_hex);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin3 = TxIn::from_hex(txin_hex).unwrap();
        assert_eq!(txin3.to_hex().unwrap(), txin_hex);

        tx.add_inputs(vec![txin1, txin2, txin3]);

        assert_eq!(tx.get_ninputs(), 5);
    }

    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(target_arch = "wasm32")]
    fn add_txins_to_transaction_wasm() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_ninputs(), 2);

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin1 = TxIn::from_hex(txin_hex).unwrap();

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin2 = TxIn::from_hex(txin_hex).unwrap();

        let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
        let txin3 = TxIn::from_hex(txin_hex).unwrap();

        let boxed_vals = Box::from(vec![JsValue::from(txin1.clone()), JsValue::from(txin2.clone()), JsValue::from(txin3.clone())]);
        tx.add_inputs(boxed_vals);

        assert_eq!(tx.get_ninputs(), 5);

        assert_eq!(tx.get_input(4).unwrap().get_outpoint_hex(None), txin3.get_outpoint_hex(None));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn add_txouts_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_noutputs(), 2);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout1 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout1.to_hex().unwrap(), txin_hex);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout2 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout2.to_hex().unwrap(), txin_hex);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout3 = TxOut::from_hex(txin_hex).unwrap();
        assert_eq!(txout3.to_hex().unwrap(), txin_hex);

        tx.add_outputs(vec![txout1, txout2, txout3]);

        assert_eq!(tx.get_noutputs(), 5);
    }

    
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(target_arch = "wasm32")]
    fn add_txouts_to_transaction() {
        let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
        let mut tx = Transaction::from_hex(tx_hex).unwrap();

        assert_eq!(tx.to_hex().unwrap(), tx_hex);

        assert_eq!(tx.get_noutputs(), 2);

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout1 = TxOut::from_hex(txin_hex).unwrap();

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout2 = TxOut::from_hex(txin_hex).unwrap();

        let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
        let txout3 = TxOut::from_hex(txin_hex).unwrap();

        let boxed_vals = Box::from(vec![JsValue::from(txout1.clone()), JsValue::from(txout2.clone()), JsValue::from(txout3.clone())]);
        tx.add_outputs(boxed_vals);

        assert_eq!(tx.get_noutputs(), 5);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let mut txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_1.set_satoshis(500);
        tx.add_input(&txin_1);
        let mut txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_2.set_satoshis(500);
        tx.add_input(&txin_2);
        let mut txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_3.set_satoshis(2);
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), Some(1002));
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis_single_none_satoshis_returns_none() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let mut txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_1.set_satoshis(500);
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let mut txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        txin_3.set_satoshis(2);
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_txin_satoshis_all_none_satoshis_returns_none() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        assert_eq!(tx.satoshis_in(), None);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_output_satoshis() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(
            5000,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(
            400,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));
        tx.add_output(&TxOut::new(
            9999999,
            &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap(),
        ));

        assert_eq!(tx.satoshis_out(), 5000 + 400 + 9999999)
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn get_total_output_satoshis_all_zero_returns_zero() {
        // Arrange
        let mut tx = Transaction::new(1, 0);
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));
        tx.add_output(&TxOut::new(0, &P2PKHAddress::from_string("16Rcy7RYM3xkPEJr4tvUtL485Fuobi8S7o").unwrap().get_locking_script().unwrap()));

        assert_eq!(tx.satoshis_out(), 0)
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn get_outpoints() {
        let mut tx = Transaction::new(1, 0);

        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            2,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        let outpoints = tx.get_outpoints();

        assert_eq!(&outpoints[0].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f00000000");
        assert_eq!(&outpoints[1].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f01000000");
        assert_eq!(&outpoints[2].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f02000000");
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg(target_arch = "wasm32")]
    fn get_outpoints_wasm() {
        let mut tx = Transaction::new(1, 0);

        let txin_1 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            0,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_1);
        let txin_2 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            1,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_2);
        let txin_3 = TxIn::new(
            &hex::decode("4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a").unwrap(),
            2,
            &Script::default(),
            Some(u32::MAX),
        );
        tx.add_input(&txin_3);

        let outpoints = tx.get_outpoints().unwrap();

        let outpoint_slice: Vec<Vec<u8>> = serde_wasm_bindgen::from_value(outpoints).unwrap();

        assert_eq!(&outpoint_slice[0].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f00000000");
        assert_eq!(&outpoint_slice[1].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f01000000");
        assert_eq!(&outpoint_slice[2].to_hex(), "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f02000000");
    }

    // For future validation
    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn txin_error_when_given_invalid_txid() {
    //     let txin_1 = TxIn::new(
    //         &hex::decode("30333933366363303566333461616365383537616666663461643330656465353364646539366465313436303334363033303136356364313436613834356133").unwrap(),
    //         1,
    //         &Script::from_hex("76a9147a1e5b4edd76b81c816ecba65f61c78afb79bdb888ac").unwrap(),
    //         Some(0xffffffff),
    //     );
    //     assert!(txin_1.is_err(), "TxIn should error when passed an invalid txid")
    // }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn txin_from_outpoint() {
        let outpoint = "9057a1b008f17a6e1b1b39522072598ecf0d73b256c6b2c34e98257d72ce3c7907000000";

        // 793cce727d25984ec3b2c656b2730dcf8e59722052391b1b6e7af108b0a15790 <-- we need this
        // 0100000001793cce727d25984ec3b2c656b2730dcf8e59722052391b1b6e7af108b0a15790070000006a47304402204628fa202f16798ef858baba566f34d96434c61241460ee453cab8193ce87a0102206fd052780b9a8efe4d5fa6d7a2f969220e14aeb7a10b1392b5fda11417f32699412102e6bb51b303ca9cb8e805fa3d104cb030e0ad6872678f5ddb2e3a14188c571f33ffffffff0223020000000000001976a914a0b2113032da4c2c0e22960f12045cecca837c7088ac2f020000000000001976a9146fb1a8e42086219215c45a5e9cb1d94d8fbd845388ac00000000

        let mut tx = Transaction::default();
        let txin = TxIn::from_outpoint_bytes(&hex::decode(outpoint).unwrap()).unwrap();
        tx.add_input(&txin);

        assert_eq!(&txin.get_outpoint_hex(Some(true)), outpoint)
    }

    #[test]
    fn txin_from_outpoint_slice_too_short_should_error() {
        let txin = TxIn::from_outpoint_bytes(&[]);

        assert!(txin.is_err(), "An Outpoint must be precisely 36 bytes long")
    }
}
