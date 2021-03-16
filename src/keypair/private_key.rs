use crate::keypair::public_key::PublicKey;
use std::{borrow::Borrow, io::Read, str::FromStr};
use bitcoin_hashes::{Hash, hex::{FromHex, ToHex}};
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

  pub fn to_wif(&self) -> String {
    // 1. Get Private Key hex
    let priv_key_hex = self.to_hex();

    // 2. Add 0x80 in front + 0x01 to end if compressed pub key
    let padded_hex = format!("80{}", priv_key_hex);

    // 3. SHA256d
    let bytes =  match hex::decode(padded_hex.clone()) {
      Ok(v) => v,
      Err(e) => wasm_bindgen::throw_str(&e.to_string())
    };

    let shad_hex = bitcoin_hashes::sha256d::Hash::hash(&bytes);

    // 4. Take first 4 bytes as checksum
    let checksum = shad_hex.to_vec()[0..4].to_hex();

    // 5. Add checksum to end of padded private key
    let extended_key = format!("{}{}", padded_hex, checksum);

    // 6 Base58 Result
    let extended_key_bytes = match hex::decode(extended_key) {
      Ok(v) => v,
      Err(e) => wasm_bindgen::throw_str(&e.to_string())
    };

    bs58::encode(extended_key_bytes).into_string()
  }
}