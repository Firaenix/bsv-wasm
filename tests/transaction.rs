#[cfg(test)]
mod transaction_tests {
  use bsv_wasm::*;
  extern crate wasm_bindgen_test;
  use bsv_wasm::TxIn;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn deserialise_transaction_hex() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.get_version(), 1);
    assert_eq!(tx.get_ninputs(), 0x02);
    
    let tx_in_0 = tx.get_input(0).unwrap();
    
    assert_eq!(tx_in_0.get_prev_tx_id(), hex::decode("3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e").unwrap());
    assert_eq!(tx_in_0.get_vout(), 0);
    assert_eq!(tx_in_0.get_script_sig_size(), 0x8c);
    assert_eq!(tx_in_0.get_script_sig().to_bytes().len(), 0x8c);
    assert_eq!(tx_in_0.get_script_sig().to_bytes(), hex::decode("493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2").unwrap());
    assert_eq!(tx_in_0.get_sequence(), 4294967295);

    let tx_in_1 = tx.get_input(1).unwrap();
    assert_eq!(tx_in_1.get_vout(), 2);
    assert_eq!(tx_in_1.get_script_sig_size(), 0x8b);
    assert_eq!(tx_in_1.get_script_sig().to_bytes().len(), 0x8b);
    assert_eq!(tx_in_1.get_script_sig().to_bytes(), hex::decode("48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442").unwrap());
    assert_eq!(tx_in_1.get_sequence(), 4294967295);

    let tx_out_0 = tx.get_output(0).unwrap();
    assert_eq!(tx_out_0.get_satoshis(), 1076000);
    assert_eq!(tx_out_0.get_script_pub_key_size(), 25);
    assert_eq!(tx_out_0.get_script_pub_key(), hex::decode("76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac").unwrap());

    let tx_out_1 = tx.get_output(1).unwrap();
    assert_eq!(tx_out_1.get_satoshis(), 117488);
    assert_eq!(tx_out_1.get_script_pub_key_size(), 25);
    assert_eq!(tx_out_1.get_script_pub_key(), hex::decode("76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac").unwrap());

    assert_eq!(tx.to_json_string().unwrap(), "{\"version\":1,\"inputs\":[{\"prev_tx_id\":\"3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e\",\"vout\":0,\"script_sig\":\"493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2\",\"sequence\":4294967295},{\"prev_tx_id\":\"6f653a93e7ff01c3317ee9eb9b75c85d4881684f8117f73f4765b61a7a5e19a3\",\"vout\":2,\"script_sig\":\"48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442\",\"sequence\":4294967295}],\"outputs\":[{\"value\":1076000,\"script_pub_key\":\"76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac\"},{\"value\":117488,\"script_pub_key\":\"76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac\"}],\"n_locktime\":0}")
  }

  #[test]
  fn deserialise_transaction_hex_malformed() {
    let tx_hex = "FAKE01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let tx = Transaction::from_hex(tx_hex.to_string()); 

    assert_eq!(tx.is_err(), true);
  }

  #[test]
  #[wasm_bindgen_test]
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
  #[wasm_bindgen_test]
  fn add_input_to_transaction() {
    let mut tx = Transaction::new(1, 4);

    assert_eq!(tx.get_n_locktime(), 4);
    assert_eq!(tx.get_version(), 1);
    assert_eq!(tx.get_ninputs(), 0);
    assert_eq!(tx.get_input(0), None);
    assert_eq!(tx.get_noutputs(), 0);
    assert_eq!(tx.get_output(0), None);

    let input = TxIn::new(vec![], 0, vec![], 0);

    tx.add_input(&input);
    assert_eq!(tx.get_ninputs(), 1);
    assert_eq!(tx.get_input(0), Some(input));
  }

  #[test]
  #[wasm_bindgen_test]
  fn add_output_to_transaction() {
    let mut tx = Transaction::new(1, 4);

    assert_eq!(tx.get_n_locktime(), 4);
    assert_eq!(tx.get_version(), 1);
    assert_eq!(tx.get_ninputs(), 0);
    assert_eq!(tx.get_input(0), None);
    assert_eq!(tx.get_noutputs(), 0);
    assert_eq!(tx.get_output(0), None);

    let output = TxOut::new(0, vec![]);

    tx.add_output(&output);
    assert_eq!(tx.get_noutputs(), 1);
    assert_eq!(tx.get_output(0), Some(output));
  }

  #[test]
  #[wasm_bindgen_test]
  fn txin_to_hex() {
    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin = TxIn::from_hex(txin_hex.to_string()).unwrap();

    assert_eq!(txin.to_hex().unwrap(), txin_hex);
  }

  #[test]
  fn txin_to_hex_fail() {
    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txin = TxIn::from_hex(txin_hex.to_string());

    assert_eq!(txin.is_err(), true);
  }

  #[test]
  #[wasm_bindgen_test]
  fn txout_to_hex() {
    let txout_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout = TxOut::from_hex(txout_hex.to_string()).unwrap();

    assert_eq!(txout.to_hex().unwrap(), txout_hex)
  }


  #[test]
  #[wasm_bindgen_test]
  fn transaction_to_hex() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.to_hex().unwrap(), tx_hex)
  }

  #[cfg(not(target_arch = "wasm32"))]
  #[test]
  fn add_txins_to_transaction() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let mut tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.get_ninputs(), 2);

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin1 = TxIn::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin2 = TxIn::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin3 = TxIn::from_hex(txin_hex.to_string()).unwrap();

    tx.add_inputs(vec![txin1, txin2, txin3]);

    assert_eq!(tx.get_ninputs(), 5);
  }

  #[cfg(target_arch = "wasm32")]
  #[wasm_bindgen_test]
  fn add_txins_to_transaction_wasm() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let mut tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.get_ninputs(), 2);

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin1 = TxIn::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin2 = TxIn::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "7967a5185e907a25225574544c31f7b059c1a191d65b53dcc1554d339c4f9efc010000006a47304402206a2eb16b7b92051d0fa38c133e67684ed064effada1d7f925c842da401d4f22702201f196b10e6e4b4a9fff948e5c5d71ec5da53e90529c8dbd122bff2b1d21dc8a90121039b7bcd0824b9a9164f7ba098408e63e5b7e3cf90835cceb19868f54f8961a825ffffffff";
    let txin3 = TxIn::from_hex(txin_hex.to_string()).unwrap();


    let boxed_vals = Box::from(vec![JsValue::from(txin1.clone()), JsValue::from(txin2.clone()), JsValue::from(txin3.clone())]);
    tx.add_inputs(boxed_vals);

    assert_eq!(tx.get_ninputs(), 5);

    assert_eq!(tx.get_input(4).unwrap().get_outpoint_hex(), txin3.get_outpoint_hex());
  }

  #[cfg(not(target_arch = "wasm32"))]
  #[test]
  fn add_txouts_to_transaction() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let mut tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.get_noutputs(), 2);

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout1 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout2 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout3 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    tx.add_outputs(vec![txout1, txout2, txout3]);

    assert_eq!(tx.get_noutputs(), 5);
  }

  #[cfg(target_arch = "wasm32")]
  #[wasm_bindgen_test]
  fn add_txouts_to_transaction() {
    let tx_hex = "01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let mut tx = Transaction::from_hex(tx_hex.to_string()).unwrap();

    assert_eq!(tx.get_noutputs(), 2);

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout1 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout2 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    let txin_hex = "4baf2100000000001976a914db4d1141d0048b1ed15839d0b7a4c488cd368b0e88ac";
    let txout3 = TxOut::from_hex(txin_hex.to_string()).unwrap();

    let boxed_vals = Box::from(vec![JsValue::from(txout1.clone()), JsValue::from(txout2.clone()), JsValue::from(txout3.clone())]);
    tx.add_outputs(boxed_vals);

    assert_eq!(tx.get_noutputs(), 5);
  }
}