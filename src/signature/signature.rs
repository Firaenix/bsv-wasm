use crate::{get_hash_digest, BSVErrors, PublicKey, PublicKeyErrors, Sha256r, SigningHash, ECDSA};
use anyhow::*;
use digest::Digest;
use ecdsa::signature::{DigestVerifier, Signature as SigTrait};
use elliptic_curve::sec1::*;
use k256::{
    ecdsa::Signature as SecpSignature,
    ecdsa::{recoverable, signature::Verifier, VerifyingKey},
    EncodedPoint, FieldBytes, Scalar,
};
use wasm_bindgen::{convert::OptionIntoWasmAbi, prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub(crate) sig: k256::ecdsa::Signature,
    pub(crate) recovery_i: u8,
}

/**
 * Implementation Methods
 */
impl Signature {
    pub(crate) fn from_der_impl(bytes: Vec<u8>, is_recoverable: bool) -> Result<Signature, BSVErrors> {
        let sig = SecpSignature::from_der(&bytes)?;

        Ok(Signature {
            sig,
            recovery_i: is_recoverable as u8,
        })
    }

    pub(crate) fn from_hex_der_impl(hex: String, is_recoverable: bool) -> Result<Signature, BSVErrors> {
        let bytes = hex::decode(hex)?;
        let sig = SecpSignature::from_der(&bytes)?;

        Ok(Signature {
            sig,
            recovery_i: is_recoverable as u8,
        })
    }

    pub(crate) fn get_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, BSVErrors> {
        let recovery_id_main = match recoverable::Id::new(self.recovery_i) {
            Ok(v) => v,
            Err(e) => {
                return Err(BSVErrors::PublicKeyRecoveryError(
                    format!("Recovery I ({}) is too large, must be 0 or 1 for this library. {}", self.recovery_i, e),
                    None,
                ))
            }
        };

        let recoverable_sig = recoverable::Signature::new(&self.sig, recovery_id_main)?;
        let message_digest = get_hash_digest(hash_algo, message);
        let verify_key = match recoverable_sig.recover_verify_key_from_digest(message_digest) {
            Ok(v) => v,
            Err(e) => {
                return Err(BSVErrors::PublicKeyRecoveryError(format!("Signature Hex: {} Id: {}", self.to_hex(), self.recovery_i), Some(e)));
            }
        };

        let pub_key = PublicKey::from_bytes_impl(&verify_key.to_bytes().to_vec())?;

        Ok(pub_key)
    }

    pub(crate) fn from_compact_impl(compact_bytes: Vec<u8>) -> Result<Signature, BSVErrors> {
        // 27-30: P2PKH uncompressed
        // 31-34: P2PKH compressed
        let i = match compact_bytes[0] - 27 {
            x if x > 4 => x - 4,
            x => x,
        };

        let r = Scalar::from_bytes_reduced(FieldBytes::from_slice(&compact_bytes[1..33]));
        let s = Scalar::from_bytes_reduced(FieldBytes::from_slice(&compact_bytes[33..65]));

        let sig = SecpSignature::from_scalars(r, s)?;

        Ok(Signature { sig, recovery_i: i })
    }
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        let bytes = self.sig.to_der();

        hex::encode(bytes)
    }

    #[wasm_bindgen(js_name = toDER)]
    pub fn to_der_bytes(&self) -> Vec<u8> {
        let bytes = self.sig.to_der();

        bytes.as_bytes().to_vec()
    }

    #[wasm_bindgen(js_name = toCompactBytes)]
    pub fn to_compact_bytes(&self) -> Vec<u8> {
        let mut compact_buf = vec![];

        // Need to handle compression?
        compact_buf.push(self.recovery_i + 27 + 4);

        let r_bytes = &*self.sig.r().to_bytes();
        compact_buf.extend_from_slice(r_bytes);

        let s_bytes = &*self.sig.s().to_bytes();
        compact_buf.extend_from_slice(s_bytes);

        compact_buf
    }

    #[wasm_bindgen(js_name = verifyMessage)]
    pub fn verify_message(&self, message: Vec<u8>, pub_key: &PublicKey) -> bool {
        match ECDSA::verify_digest_impl(&message, pub_key, self, SigningHash::Sha256) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl Signature {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromDER))]
    pub fn from_der(bytes: Vec<u8>, is_recoverable: bool) -> Result<Signature, JsValue> {
        match Signature::from_der_impl(bytes, is_recoverable) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHexDER))]
    pub fn from_hex_der(hex: String, is_recoverable: bool) -> Result<Signature, JsValue> {
        match Signature::from_hex_der_impl(hex, is_recoverable) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = fromCompactBytes)]
    pub fn from_compact_bytes(compact_bytes: Vec<u8>) -> Result<Signature, JsValue> {
        match Signature::from_compact_impl(compact_bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = recoverPublicKey)]
    pub fn recover_public_key(&self, message: Vec<u8>, hash_algo: SigningHash) -> Result<PublicKey, JsValue> {
        match Signature::get_public_key(&self, &message, hash_algo) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl Signature {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_der(bytes: Vec<u8>, is_recoverable: bool) -> Result<Signature, BSVErrors> {
        Signature::from_der_impl(bytes, is_recoverable)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex_der(hex: String, is_recoverable: bool) -> Result<Signature, BSVErrors> {
        Signature::from_hex_der_impl(hex, is_recoverable)
    }

    pub fn from_compact_bytes(compact_bytes: Vec<u8>) -> Result<Signature, BSVErrors> {
        Signature::from_compact_impl(compact_bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn recover_public_key(&self, message: Vec<u8>, hash_algo: SigningHash) -> Result<PublicKey, BSVErrors> {
        Signature::get_public_key(&self, &message, hash_algo)
    }
}
