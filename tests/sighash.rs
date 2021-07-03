#[cfg(test)]
mod sighash_tests {
  use bsv_wasm::*;
  extern crate wasm_bindgen_test;
  use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!();

  // This will remain commented out until some way of matching the SIGHASH_SINGLE bug can be achieved (Still use SHA256 for generating K but do not calculate preimage -> hash)
  // #[test]
  // #[wasm_bindgen_test]
  // fn sighash_single_bug() {
  //   let sighash = SigHash::InputsOutput;
  //   let signing_script = Script::from_asm_string("".into()).unwrap();
  //   let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000".into()).unwrap();

  //   let sighash_buf = tx.sighash_preimage(2, sighash, &signing_script, 0).unwrap();

  //   assert_eq!(sighash_buf.to_hex(), "0000000000000000000000000000000000000000000000000000000000000001")
  // }

  #[test]
  #[wasm_bindgen_test]
  fn sighash_inputs_output_SINGLE() { 
    let priv_key = PrivateKey::from_wif("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx".into()).unwrap();
    let sighash = SigHash::InputsOutput;
    let signing_script = Script::from_asm_string("OP_0 OP_RETURN".into()).unwrap();
    let mut tx = Transaction::from_hex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000".into()).unwrap();

    let sighash_buffer = tx.sighash_preimage(sighash, 0, &signing_script, 0).unwrap();
    assert_eq!(sighash_buffer.to_hex(), "010000008bf38a2d3f477a28aba2fe171260ffb0315c7371617ba6e39aea4ed97558c35800000000000000000000000000000000000000000000000000000000000000009e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f0000000002006a0000000000000000ffffffffc7732d98e887792b43e5dae92a159010d22e47d60ed48b88ba7b6c12a3c9e7560000000043000000");

    assert_eq!(tx.sign(&priv_key, sighash, 0, &signing_script, 0).unwrap().to_hex().unwrap(), "30440220798bd19a0bb1fd5e1b3832e46ae69af687d87cbd179a81e60af719382860aee502206d849665f4010a54f40d9ac463629cbd758042745cebf7122818d9bfea2bce7043")
  }
}