

#[cfg(test)]
mod tests {
  use bsv_rs::*;
  extern crate wasm_bindgen_test;
  use wasm_bindgen_test::*;
  wasm_bindgen_test::wasm_bindgen_test_configure!();

  #[test]
  #[wasm_bindgen_test]
  fn import_signature() {
    let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
    let sig = Signature::from_der(hex::decode(sig_hex).unwrap()).unwrap();
    assert_eq!(sig.to_hex(), sig_hex)
  }

  #[test]
  #[wasm_bindgen_test]
  fn import_signature_string() {
    let sig_hex = "3044022075fc517e541bd54769c080b64397e32161c850f6c1b2b67a5c433affbb3e62770220729e85cc46ffab881065ec07694220e71d4df9b2b8c8fd12c3122cf3a5efbcf2";
    let sig = Signature::from_hex_der(sig_hex.into()).unwrap();
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

    let signature = key.sign_message(message.to_vec()).unwrap();
    let pub_key = PublicKey::from_private_key(&key, true);

    let is_verified = signature.verify(message.to_vec(), &pub_key).unwrap();
    assert_eq!(is_verified, true);
    assert_eq!(signature.to_hex(), "3045022100fab965a4dd445c990f46689f7acdc6e089128dc2d743457b350032d66336edb7022005f5684cc707b569120ef0442343998c95f6514c751251a91f82b1ec6a92da78".to_lowercase())
  }
}

