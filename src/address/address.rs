use std::borrow::Borrow;

use crate::PublicKey;
use anyhow::private::kind::TraitKind;
use bitcoin_hashes::{Hash, hex::{FromHex, ToHex}};
use js_sys::Error;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct P2PKHAddress {
  pubkey_hash: Vec<u8>
}

#[wasm_bindgen]
impl P2PKHAddress {
  #[wasm_bindgen(js_name=fromPubKeyHash)]
  pub fn from_pubkey_hash(hash_bytes: Vec<u8>) -> P2PKHAddress {
    P2PKHAddress{
      pubkey_hash: hash_bytes
    }
  }

  #[wasm_bindgen(js_name=fromPubKey)]
  pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, JsValue> { 
    let pub_key_hex = match pub_key.to_hex() {
      Ok(v) => v,
      Err(e) => return Err(e)
    };

    let pub_key_bytes = match hex::decode(pub_key_hex) {
      Ok(v) => v,
      Err(e) => throw_str(&e.to_string())
    };

    let pub_key_hash = bitcoin_hashes::hash160::Hash::hash(&pub_key_bytes);
    
    Ok(P2PKHAddress::from_pubkey_hash(pub_key_hash.to_vec()))
  }

  #[wasm_bindgen(js_name=toString)]
  pub fn to_address_string(&self) -> Result<String, JsValue> {
    let mut pub_key_hash_bytes = self.pubkey_hash.clone();
    
    let mut address_bytes: Vec<u8> = vec![00];
    address_bytes.append(&mut pub_key_hash_bytes);

    let shad_bytes = bitcoin_hashes::sha256d::Hash::hash(&address_bytes);
    let mut checksum_bytes = shad_bytes[0..4].to_vec();

    address_bytes.append(&mut checksum_bytes);

    let address = bs58::encode(address_bytes);

    Ok(address.into_string())
  }
}