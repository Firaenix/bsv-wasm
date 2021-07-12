use crate::{BSVErrors, Hash, Script, BSM};
use crate::{PrivateKey, PublicKey, Signature};
use anyhow::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P2PKHAddress {
    pubkey_hash: Vec<u8>,
}

impl P2PKHAddress {
    pub(crate) fn from_pubkey_hash_impl(hash_bytes: Vec<u8>) -> P2PKHAddress {
        P2PKHAddress { pubkey_hash: hash_bytes }
    }

    pub(crate) fn from_pubkey_impl(pub_key: &PublicKey) -> Result<P2PKHAddress, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let pub_key_hash = Hash::hash_160(&pub_key_bytes);

        Ok(P2PKHAddress::from_pubkey_hash_impl(pub_key_hash.to_bytes()))
    }

    pub(crate) fn to_address_string_impl(&self) -> Result<String, BSVErrors> {
        let mut pub_key_hash_bytes = self.pubkey_hash.clone();

        let mut address_bytes: Vec<u8> = vec![00];
        address_bytes.append(&mut pub_key_hash_bytes);

        let shad_bytes = Hash::sha_256d(&address_bytes).to_bytes();
        let mut checksum_bytes = shad_bytes[0..4].to_vec();

        address_bytes.append(&mut checksum_bytes);

        let address = bs58::encode(address_bytes);

        Ok(address.into_string())
    }

    pub(crate) fn from_p2pkh_string_impl(address_string: String) -> Result<P2PKHAddress, BSVErrors> {
        let decoded = bs58::decode(address_string.clone());
        let address_bytes = decoded.into_vec()?;

        // Remove 0x00 from the front and the 4 byte checksum off the end
        let pub_key_hash = address_bytes[1..address_bytes.len() - 4].to_vec();

        Ok(P2PKHAddress { pubkey_hash: pub_key_hash })
    }

    /**
     * Produces the locking script for a P2PKH address.
     * Should be inserted into a new TxOut.
     */
    pub(crate) fn to_locking_script_impl(&self) -> Result<Script, BSVErrors> {
        Script::from_asm_string_impl(format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", self.to_pubkey_hash_hex()))
    }

    /**
     * Produces the unlocking script for a P2PKH address.
     * Should be inserted into a TxIn.
     */
    pub(crate) fn to_unlocking_script_impl(&self, pub_key: &PublicKey, sig: &Signature) -> Result<Script, BSVErrors> {
        // Make sure the given Public Key matches this address.
        let verifying_address = P2PKHAddress::from_pubkey_impl(pub_key)?;

        if verifying_address != *self {
            return Err(BSVErrors::GenerateScript(format!("Given public key does not correspond to this address")));
        }

        let pub_key_hex = pub_key.to_hex_impl()?;
        let script = Script::from_asm_string_impl(format!("{} {}", sig.to_hex(), pub_key_hex))?;

        Ok(script)
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

    /**
     * Check if message is signed by this Address.
     *
     * Returns a boolean
     */
    #[wasm_bindgen(js_name = isValidBitcoinMessage)]
    pub fn is_valid_bitcoin_message(&self, message: &[u8], signature: &Signature) -> bool {
        match BSM::verify_message_impl(message, signature, self) {
            Ok(_) => true,
            Err(_) => false,
        }
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

    #[wasm_bindgen(js_name = toLockingScript)]
    pub fn get_locking_script(&self) -> Result<Script, JsValue> {
        match self.to_locking_script_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toUnlockingScript)]
    pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &Signature) -> Result<Script, JsValue> {
        match self.to_unlocking_script_impl(pub_key, sig) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    /**
     * Verify if message is signed by this Address.
     *
     * Throws an error if invalid.
     */
    #[wasm_bindgen(js_name = verifyBitcoinMessage)]
    pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, JsValue> {
        match BSM::verify_message_impl(message, signature, self) {
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
    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_pubkey_impl(pub_key)
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_address_string(&self) -> Result<String, BSVErrors> {
        P2PKHAddress::to_address_string_impl(&self)
    }

    pub fn from_p2pkh_string(address_string: String) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_p2pkh_string_impl(address_string)
    }

    pub fn get_locking_script(&self) -> Result<Script, BSVErrors> {
        self.to_locking_script_impl()
    }

    pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &Signature) -> Result<Script, BSVErrors> {
        self.to_unlocking_script_impl(pub_key, sig)
    }

    /**
     * Verify if message is signed by this Address.
     *
     * Returns a Result
     */
    pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        BSM::verify_message_impl(message, signature, self)
    }
}
