use crate::Hash;
use std::io::Write;

use crate::{P2PKHAddress, PrivateKey, Signature, SigningHash, VarInt, ECDSA};
use anyhow::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsValue};

/**
 * Bitcoin Signed Message
 */
#[wasm_bindgen]
pub struct BSM {}

const MAGIC_BYTES: &[u8] = b"Bitcoin Signed Message:\n";

impl BSM {
    fn prepend_magic_bytes(msg: &[u8]) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![];

        buffer.write_varint(MAGIC_BYTES.len() as u64)?;
        buffer.write_all(MAGIC_BYTES)?;
        buffer.write_varint(msg.len() as u64)?;
        buffer.write_all(msg)?;

        Ok(buffer)
    }

    /**
     * Sign a message with the intention of verifying with this same Address.
     * Used when using Bitcoin Signed Messages ()
     */
    pub(crate) fn sign_impl(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature> {
        let magic_message = BSM::prepend_magic_bytes(message)?;
        // let magic_message = message;
        ECDSA::sign_with_deterministic_k_impl(priv_key, &magic_message, SigningHash::Sha256d, false)
    }

    pub(crate) fn verify_message_impl(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool> {
        let magic_message = BSM::prepend_magic_bytes(message)?;
        // let magic_message = message;

        let public_key = signature.get_public_key(&magic_message, SigningHash::Sha256d)?;
        let verify_p2pkh = P2PKHAddress::from_pubkey_impl(&public_key)?;

        let verify_address = verify_p2pkh.to_address_string_impl()?;
        let address_string = address.to_address_string_impl()?;
        if verify_address != address_string {
            return Err(anyhow!("Provided address ({}) does not match signature address ({})", address_string, verify_address));
        }
        ECDSA::verify_digest_impl(&magic_message, &public_key, signature, SigningHash::Sha256d)?;
        Ok(true)
    }
}

#[wasm_bindgen]
impl BSM {
    /**
     * Sign a message with the intention of verifying with this same Address.
     * Used when using Bitcoin Signed Messages
     *
     * Returns boolean
     */
    #[wasm_bindgen(js_name = isValidMessage)]
    pub fn is_valid_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> bool {
        match BSM::verify_message_impl(message, signature, address) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BSM {
    #[wasm_bindgen(js_name = verifyMessage)]
    pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, JsValue> {
        match BSM::verify_message_impl(message, signature, address) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = signMessage)]
    pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, JsValue> {
        match BSM::sign_impl(priv_key, message) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl BSM {
    pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool> {
        BSM::verify_message_impl(message, signature, address)
    }

    pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature> {
        BSM::sign_impl(priv_key, message)
    }
}
