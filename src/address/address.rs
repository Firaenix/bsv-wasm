use crate::PublicKey;
use crate::{AddressErrors, Hash};
use anyhow::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P2PKHAddress {
    pubkey_hash: Vec<u8>,
}

impl P2PKHAddress {
    fn from_pubkey_hash_impl(hash_bytes: Vec<u8>) -> P2PKHAddress {
        P2PKHAddress {
            pubkey_hash: hash_bytes,
        }
    }

    fn from_pubkey_impl(pub_key: &PublicKey) -> Result<P2PKHAddress, AddressErrors> {
        let pub_key_hex = match pub_key.to_hex_impl() {
            Ok(v) => v,
            Err(e) => return Err(AddressErrors::PublicKeyError { error: e }),
        };

        let pub_key_bytes = match hex::decode(&pub_key_hex) {
            Ok(v) => v,
            Err(e) => {
                return Err(AddressErrors::ParseHex {
                    hex: pub_key_hex,
                    error: e,
                })
            }
        };

        let pub_key_hash = Hash::hash_160(&pub_key_bytes);

        Ok(P2PKHAddress::from_pubkey_hash_impl(pub_key_hash.to_bytes()))
    }

    fn to_address_string_impl(&self) -> Result<String, AddressErrors> {
        let mut pub_key_hash_bytes = self.pubkey_hash.clone();

        let mut address_bytes: Vec<u8> = vec![00];
        address_bytes.append(&mut pub_key_hash_bytes);

        let shad_bytes = Hash::sha_256d(&address_bytes).to_bytes();
        let mut checksum_bytes = shad_bytes[0..4].to_vec();

        address_bytes.append(&mut checksum_bytes);

        let address = bs58::encode(address_bytes);

        Ok(address.into_string())
    }

    pub(crate) fn from_p2pkh_string_impl(
        address_string: String,
    ) -> Result<P2PKHAddress, AddressErrors> {
        let decoded = bs58::decode(address_string.clone());

        let address_bytes = match decoded.into_vec() {
            Ok(v) => v,
            Err(e) => {
                return Err(AddressErrors::Base58Decode {
                    error: anyhow!(e),
                    string: address_string,
                })
            }
        };

        // Remove 0x00 from the front and the 4 byte checksum off the end
        let pub_key_hash = address_bytes[1..address_bytes.len() - 4].to_vec();

        Ok(P2PKHAddress {
            pubkey_hash: pub_key_hash,
        })
    }
}

/**
  Shared Methods
*/
#[wasm_bindgen]
impl P2PKHAddress {
    #[wasm_bindgen(js_name = toPubKeyHashBytes)]
    pub fn to_pubkey_hash(&self) -> Vec<u8> {
        self.pubkey_hash.clone()
    }

    #[wasm_bindgen(js_name = toPubKeyHashHex)]
    pub fn to_pubkey_hash_hex(&self) -> String {
        hex::encode(self.pubkey_hash.clone())
    }
}

/**
 * WASM Exported Methods
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl P2PKHAddress {
    #[wasm_bindgen(js_name = fromPubKeyHash)]
    pub fn from_pubkey_hash(hash_bytes: Vec<u8>) -> P2PKHAddress {
        P2PKHAddress::from_pubkey_hash_impl(hash_bytes)
    }
    #[wasm_bindgen(js_name = fromPubKey)]
    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, JsValue> {
        match P2PKHAddress::from_pubkey_impl(pub_key) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
    #[wasm_bindgen(js_name = toString)]
    pub fn to_address_string(&self) -> Result<String, JsValue> {
        match P2PKHAddress::to_address_string_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = fromP2PKHString)]
    pub fn from_p2pkh_string(address_string: String) -> Result<P2PKHAddress, JsValue> {
        match P2PKHAddress::from_p2pkh_string_impl(address_string) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
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
    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, AddressErrors> {
        P2PKHAddress::from_pubkey_impl(pub_key)
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_address_string(&self) -> Result<String, AddressErrors> {
        P2PKHAddress::to_address_string_impl(&self)
    }

    pub fn from_p2pkh_string(address_string: String) -> Result<P2PKHAddress, AddressErrors> {
        P2PKHAddress::from_p2pkh_string_impl(address_string)
    }
}
