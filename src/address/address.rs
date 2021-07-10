use crate::{AddressErrors, Hash, PublicKeyErrors, Script, ScriptErrors};
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
    fn from_pubkey_hash_impl(hash_bytes: Vec<u8>) -> P2PKHAddress {
        P2PKHAddress { pubkey_hash: hash_bytes }
    }

    fn from_pubkey_impl(pub_key: &PublicKey) -> Result<P2PKHAddress> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let pub_key_hash = Hash::hash_160(&pub_key_bytes);

        Ok(P2PKHAddress::from_pubkey_hash_impl(pub_key_hash.to_bytes()))
    }

    fn to_address_string_impl(&self) -> Result<String> {
        let mut pub_key_hash_bytes = self.pubkey_hash.clone();

        let mut address_bytes: Vec<u8> = vec![00];
        address_bytes.append(&mut pub_key_hash_bytes);

        let shad_bytes = Hash::sha_256d(&address_bytes).to_bytes();
        let mut checksum_bytes = shad_bytes[0..4].to_vec();

        address_bytes.append(&mut checksum_bytes);

        let address = bs58::encode(address_bytes);

        Ok(address.into_string())
    }

    pub(crate) fn from_p2pkh_string_impl(address_string: String) -> Result<P2PKHAddress> {
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
    pub(crate) fn to_tx_out_script_impl(&self) -> Result<Script, ScriptErrors> {
        Script::from_asm_string_impl(format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", self.to_pubkey_hash_hex()))
    }

    /**
     * Produces the unlocking script for a P2PKH address.
     * Should be inserted into a TxIn.
     */
    pub(crate) fn to_tx_in_script_impl(&self, pub_key: &PublicKey, sig: &Signature) -> Result<Script> {
        // Make sure the given Public Key matches this address.
        let verifying_address = P2PKHAddress::from_pubkey_impl(pub_key)?;

        let pub_key_hex = pub_key.to_hex_impl()?;
        match Script::from_asm_string_impl(format!("{} {}", sig.to_hex_impl(), pub_key_hex)) {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!(e)),
        }
    }

    // /**
    //  * TODO: Sign a message with the intention of verifying with this same Address.
    //  * Used when using Bitcoin Signed Messages ()
    //  */
    // pub(crate) fn sign_impl(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature> {
    //     PublicKey::from_private_key(priv_key, compress)
    // }

    // /**
    //  * Sign a message with the intention of verifying with this same Address.
    //  * Used when using Bitcoin Signed Messages ()
    //  */
    // pub(crate) fn verify_impl(message: &[u8], signature: &Signature) -> Result<bool> {}
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

    #[wasm_bindgen(js_name = toTxOutScript)]
    pub fn to_tx_out_script(&self) -> Result<Script, JsValue> {
        match self.to_tx_out_script_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    /**
     * This method is an alias for toTxOutScript
     */
    #[wasm_bindgen(js_name = toLockingScript)]
    pub fn to_locking_script(&self) -> Result<Script, JsValue> {
        self.to_tx_out_script()
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
    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress> {
        P2PKHAddress::from_pubkey_impl(pub_key)
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_address_string(&self) -> Result<String> {
        P2PKHAddress::to_address_string_impl(&self)
    }

    pub fn from_p2pkh_string(address_string: String) -> Result<P2PKHAddress> {
        P2PKHAddress::from_p2pkh_string_impl(address_string)
    }

    pub fn to_tx_out_script(&self) -> Result<Script, ScriptErrors> {
        self.to_tx_out_script_impl()
    }

    pub fn to_locking_script(&self) -> Result<Script, ScriptErrors> {
        self.to_tx_out_script()
    }
}
