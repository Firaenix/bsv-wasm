use crate::AddressErrors;
use std::borrow::Borrow;

use anyhow::private::kind::TraitKind;
use bitcoin_hashes::{Hash, hex::{FromHex, ToHex}};
use js_sys::Error;
use wasm_bindgen::{prelude::*, throw_str};
use wasm_bindgen::JsValue;
use crate::PublicKey;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct P2PKHAddress {
  pubkey_hash: Vec<u8>
}

impl P2PKHAddress {
  
  fn from_pubkey_hash_impl(hash_bytes: Vec<u8>) -> P2PKHAddress {
    P2PKHAddress{
      pubkey_hash: hash_bytes
    }
  }

  
  fn from_pubkey_impl(pub_key: &PublicKey) ->  Result<P2PKHAddress, AddressErrors> { 
    let pub_key_hex = match pub_key.to_hex_impl() {
      Ok(v) => v,
      Err(e) => return Err(AddressErrors::PublicKeyError{ error: e })
    };

    let pub_key_bytes = match hex::decode(&pub_key_hex) {
      Ok(v) => v,
      Err(e) => return Err(AddressErrors::ParseHex{ hex: pub_key_hex, error: e })
    };

    let pub_key_hash = bitcoin_hashes::hash160::Hash::hash(&pub_key_bytes);
    
    Ok(P2PKHAddress::from_pubkey_hash_impl(pub_key_hash.to_vec()))
  }

  
  fn to_address_string_impl(&self) ->  Result<String, AddressErrors> {
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

/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl P2PKHAddress {

  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromPubKeyHash))]
  pub fn from_pubkey_hash(hash_bytes: Vec<u8>) -> P2PKHAddress {
    P2PKHAddress::from_pubkey_hash_impl(hash_bytes)
  }
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromPubKey))]
  pub fn from_pubkey(pub_key: &PublicKey) ->  Result<P2PKHAddress, JsValue> {
    match P2PKHAddress::from_pubkey_impl(pub_key) {
      Ok(v) => Ok(v),
        Err(e) => throw_str(&e.to_string())
    }
  }
  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
  pub fn to_address_string(&self) ->  Result<String, JsValue> {
    match P2PKHAddress::to_address_string_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string())
    }
  }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl P2PKHAddress {
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_pubkey_hash(hash_bytes: Vec<u8>) -> P2PKHAddress {
    P2PKHAddress::from_pubkey_hash_impl(hash_bytes)
  }
  #[cfg(not(target_arch = "wasm32"))]
  pub fn from_pubkey(pub_key: &PublicKey) ->  Result<P2PKHAddress, AddressErrors> {
    P2PKHAddress::from_pubkey_impl(pub_key)
  }
  #[cfg(not(target_arch = "wasm32"))]
  pub fn to_address_string(&self) ->  Result<String, AddressErrors> {
    P2PKHAddress::to_address_string_impl(&self)
  }

}