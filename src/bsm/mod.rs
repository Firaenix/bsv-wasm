use crate::BSVErrors;
use crate::VarIntWriter;
use std::io::Write;

use crate::{P2PKHAddress, PrivateKey, Signature, SigningHash, ECDSA};

/**
 * Bitcoin Signed Message
 */
pub struct BSM {}

const MAGIC_BYTES: &[u8] = b"Bitcoin Signed Message:\n";

impl BSM {
    fn prepend_magic_bytes(msg: &[u8]) -> Result<Vec<u8>, BSVErrors> {
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
    pub(crate) fn sign_impl(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, BSVErrors> {
        let magic_message = BSM::prepend_magic_bytes(message)?;
        // let magic_message = message;
        ECDSA::sign_with_deterministic_k_impl(priv_key, &magic_message, SigningHash::Sha256d, false)
    }

    /**
     * Sign a Bitcoin Signed Message with a specific K value. I hope you know what you're doing!
     */
    pub(crate) fn sign_with_k_impl(priv_key: &PrivateKey, ephemeral_key: &PrivateKey, message: &[u8]) -> Result<Signature, BSVErrors> {
        let magic_message = BSM::prepend_magic_bytes(message)?;
        // let magic_message = message;
        ECDSA::sign_with_k_impl(priv_key, ephemeral_key, &magic_message, SigningHash::Sha256d)
    }

    pub(crate) fn verify_message_impl(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, BSVErrors> {
        let magic_message = BSM::prepend_magic_bytes(message)?;
        // let magic_message = message;

        let public_key = signature.get_public_key(&magic_message, SigningHash::Sha256d)?;
        let verify_p2pkh = P2PKHAddress::from_pubkey_impl(&public_key)?;

        let verify_address = verify_p2pkh.to_string_impl()?;
        let address_string = address.to_string_impl()?;
        if verify_address != address_string {
            return Err(BSVErrors::MessageVerification(format!(
                "Provided address ({}) does not match signature address ({})",
                address_string, verify_address
            )));
        }
        ECDSA::verify_digest_impl(&magic_message, &public_key, signature, SigningHash::Sha256d)?;
        Ok(true)
    }
}

impl BSM {
    /**
     * Sign a message with the intention of verifying with this same Address.
     * Used when using Bitcoin Signed Messages
     *
     * Returns boolean
     */
    pub fn is_valid_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> bool {
        BSM::verify_message_impl(message, signature, address).is_ok()
    }
}

impl BSM {
    pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, BSVErrors> {
        BSM::verify_message_impl(message, signature, address)
    }

    pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, BSVErrors> {
        BSM::sign_impl(priv_key, message)
    }

    pub fn sign_message_with_k(priv_key: &PrivateKey, ephemeral_key: &PrivateKey, message: &[u8]) -> Result<Signature, BSVErrors> {
        BSM::sign_with_k_impl(priv_key, ephemeral_key, message)
    }
}
