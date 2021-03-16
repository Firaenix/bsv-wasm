use crate::keypair::public_key::PublicKey;
use std::borrow::Borrow;

use wasm_bindgen::prelude::*;
use k256::SecretKey;
use rand_core::OsRng;


#[wasm_bindgen]
#[derive(Debug)]
pub struct PrivateKey {
  secret_key: SecretKey
}

#[wasm_bindgen]
impl PrivateKey {
  #[wasm_bindgen(js_name = fromRandom)]
  pub fn from_random() -> PrivateKey {
    let secret_key = k256::SecretKey::random(&mut OsRng);

    PrivateKey{
      secret_key
    }
  }

  #[wasm_bindgen(js_name = toHex)]
  pub fn to_hex(&self) -> String {
      let secret_key_bytes = self.secret_key.to_bytes().to_vec();
      hex::encode(secret_key_bytes)
  }
  
  #[wasm_bindgen(js_name = fromHex)]
  pub fn from_hex(hex_str: String) -> Option<PrivateKey> {
    let bytes = match hex::decode(hex_str) {
      Ok(bytes) => bytes,
      Err(e) => return None
    };
 
    let secret_key = match SecretKey::from_bytes(bytes) {
      Ok(key) => key,
      Err(e) => return None
    };

    Some(PrivateKey{
      secret_key
    })
  }

  pub fn get_public_key(&self) -> PublicKey {
    let pub_key = self.secret_key.public_key();
    PublicKey{
      pub_key
    }
  }
}