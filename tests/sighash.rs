#[cfg(test)]
mod sighash_tests {
  use bsv_wasm::*;
  extern crate wasm_bindgen_test;
  use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn sighash_single_bug() {
    let sighash = SigHash::InputsOutput;
    let signing_script = Script::from_asm_string("".into()).unwrap();
    let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000".into()).unwrap();

    let sighash_buf = tx.sighash(2, sighash, &signing_script, 0).unwrap();

    assert_eq!(sighash_buf.to_hex(), "0000000000000000000000000000000000000000000000000000000000000001")
  }

  #[test]
  #[wasm_bindgen_test]
  fn sighash_inputs_output_SINGLE() { 
    let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx".into()).unwrap();
    let sighash = SigHash::InputsOutput;
    let signing_script = Script::from_asm_string("OP_0 OP_RETURN".into()).unwrap();
    let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000".into()).unwrap();

    // let sighash_buf = tx.sighash_preimage(0, SigHash::SINGLE | SigHashFORKID, &signing_script, 0).unwrap();

    // If dont reverse before msg_scalar 30450221009c230cdb72228135e3b9e27bcc58d366d89be2ddbc078dccac0b17568e11e41502201ebd424c1bbb9b807770bdc9f7db098ed158b541218ff102ae220ecf044c8df843

    assert_eq!(tx.sign(&priv_key, sighash, 0, &signing_script, 0).unwrap().to_hex(), "30450221009c230cdb72228135e3b9e27bcc58d366d89be2ddbc078dccac0b17568e11e4150220325b711f4f6fd51e62a9a313cce9ba6c6ae2df1af9dcf3899fff66819e1482c743")
  }
}