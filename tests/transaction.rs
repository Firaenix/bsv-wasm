#[cfg(test)]
mod tests {
  use bsv_rs::*;
  extern crate wasm_bindgen_test;
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
    assert_eq!(tx_in_0.get_script_sig().len(), 0x8c);
    assert_eq!(tx_in_0.get_script_sig(), hex::decode("493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2").unwrap());
    assert_eq!(tx_in_0.get_sequence(), 4294967295);

    let tx_in_1 = tx.get_input(1).unwrap();
    assert_eq!(tx_in_1.get_vout(), 2);
    assert_eq!(tx_in_1.get_script_sig_size(), 0x8b);
    assert_eq!(tx_in_1.get_script_sig().len(), 0x8b);
    assert_eq!(tx_in_1.get_script_sig(), hex::decode("48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442").unwrap());
    assert_eq!(tx_in_1.get_sequence(), 4294967295);

    let tx_out_0 = tx.get_output(0).unwrap();
    assert_eq!(tx_out_0.get_satoshi_value(), 1076000);
    assert_eq!(tx_out_0.get_script_pub_key_size(), 25);
    assert_eq!(tx_out_0.get_script_pub_key(), hex::decode("76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac").unwrap());

    let tx_out_1 = tx.get_output(1).unwrap();
    assert_eq!(tx_out_1.get_satoshi_value(), 117488);
    assert_eq!(tx_out_1.get_script_pub_key_size(), 25);
    assert_eq!(tx_out_1.get_script_pub_key(), hex::decode("76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac").unwrap());

    assert_eq!(tx.to_json().unwrap(), "{\"version\":1,\"n_inputs\":2,\"inputs\":[{\"prev_tx_id\":\"3f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e\",\"vout\":0,\"script_sig_size\":140,\"script_sig\":\"493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2\",\"sequence\":4294967295},{\"prev_tx_id\":\"6f653a93e7ff01c3317ee9eb9b75c85d4881684f8117f73f4765b61a7a5e19a3\",\"vout\":2,\"script_sig_size\":139,\"script_sig\":\"48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442\",\"sequence\":4294967295}],\"n_outputs\":2,\"outputs\":[{\"value\":1076000,\"script_pub_key_size\":25,\"script_pub_key\":\"76a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88ac\"},{\"value\":117488,\"script_pub_key_size\":25,\"script_pub_key\":\"76a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac\"}],\"n_locktime\":0}")
  }

  #[test]
  fn deserialise_transaction_hex_malformed() {
    let tx_hex = "FAKE01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000";
    let tx = Transaction::from_hex(tx_hex.to_string()); 

    assert_eq!(tx.is_err(), true);
  } 
}