use crate::{get_hash_digest, BSVErrors, PublicKey, Sha256r, SigHash, SigningHash, ECDSA};
use digest::Digest;
use ecdsa::signature::{DigestVerifier, Signature as SigTrait};
use elliptic_curve::bigint::UInt;
use elliptic_curve::sec1::*;
use elliptic_curve::{bigint::ByteArray, ops::Reduce};
use k256::Secp256k1;
use k256::{
    ecdsa::Signature as SecpSignature,
    ecdsa::{recoverable, signature::Verifier, VerifyingKey},
    EncodedPoint, FieldBytes, Scalar,
};
use num_traits::{FromPrimitive, ToPrimitive};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{convert::OptionIntoWasmAbi, prelude::*, throw_str};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryInfo {
    is_y_odd: bool,
    is_x_reduced: bool,
    is_pubkey_compressed: bool,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl RecoveryInfo {
    pub fn new(is_y_odd: bool, is_x_reduced: bool, is_pubkey_compressed: bool) -> RecoveryInfo {
        RecoveryInfo {
            is_y_odd,
            is_x_reduced,
            is_pubkey_compressed,
        }
    }

    pub fn from_byte(recovery_byte: u8, is_pubkey_compressed: bool) -> RecoveryInfo {
        RecoveryInfo {
            is_x_reduced: (recovery_byte & 0b10) != 0,
            is_y_odd: (recovery_byte & 1) != 0,
            is_pubkey_compressed,
        }
    }

    pub fn default() -> RecoveryInfo {
        RecoveryInfo {
            is_y_odd: false,
            is_x_reduced: false,
            is_pubkey_compressed: false,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub(crate) sig: k256::ecdsa::Signature,
    pub(crate) recovery: Option<RecoveryInfo>,
}

/**
 * Implementation Methods
 */
impl Signature {
    pub(crate) fn from_der_impl(bytes: &[u8]) -> Result<Signature, BSVErrors> {
        let sighash_stripped_bytes = match bytes.last().and_then(|v| SigHash::from_u8(*v)) {
            Some(_v) => bytes[0..bytes.len() - 1].to_vec(),
            _ => bytes.to_vec(),
        };

        let sig = SecpSignature::from_der(&sighash_stripped_bytes)?;

        Ok(Signature { sig, recovery: None })
    }

    pub(crate) fn from_hex_der_impl(hex: &str) -> Result<Signature, BSVErrors> {
        let bytes = hex::decode(hex)?;
        Signature::from_der_impl(&bytes)
    }

    pub(crate) fn get_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, BSVErrors> {
        let recovery = match &self.recovery {
            Some(v) => v,
            None => {
                return Err(BSVErrors::PublicKeyRecoveryError(
                    "No recovery info is provided in this signature, unable to recover private key. Use compact byte serialisation instead.".into(),
                    ecdsa::Error::new(),
                ))
            }
        };

        let id = ecdsa::RecoveryId::new(recovery.is_y_odd, recovery.is_x_reduced);
        let k256_recovery = id.try_into().map_err(|e| BSVErrors::PublicKeyRecoveryError("".into(), e))?;

        let recoverable_sig = recoverable::Signature::new(&self.sig, k256_recovery)?;
        let message_digest = get_hash_digest(hash_algo, message);
        let verify_key = match recoverable_sig.recover_verify_key_from_digest(message_digest) {
            Ok(v) => v,
            Err(e) => {
                return Err(BSVErrors::PublicKeyRecoveryError(format!("Signature Hex: {} Id: {:?}", self.to_hex(), recovery), e));
            }
        };

        let pub_key = PublicKey::from_bytes_impl(&verify_key.to_bytes().to_vec())?;

        Ok(pub_key)
    }

    pub(crate) fn from_compact_impl(compact_bytes: &[u8]) -> Result<Signature, BSVErrors> {
        // 27-30: P2PKH uncompressed
        // 31-34: P2PKH compressed
        let (recovery, is_compressed) = match (compact_bytes[0] - 27) as i8 - 4 {
            x if x < 0 => (x + 4, false),
            x => (x, true),
        };

        // TODO: Check Recovery Endianness so we can recover x and y info.
        if recovery > 3 {
            return Err(BSVErrors::SignatureError("Cannot have recovery byte that is larger than 3."));
        }

        let r = *FieldBytes::from_slice(&compact_bytes[1..33]);
        let s = *FieldBytes::from_slice(&compact_bytes[33..65]);

        let sig = SecpSignature::from_scalars(r, s)?;

        Ok(Signature {
            sig,
            recovery: Some(RecoveryInfo::from_byte(recovery as u8, is_compressed)),
        })
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Signature {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        let bytes = self.sig.to_der();

        hex::encode(bytes)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toDER))]
    pub fn to_der_bytes(&self) -> Vec<u8> {
        let bytes = self.sig.to_der();

        bytes.as_bytes().to_vec()
    }

    /// NOTE: Provide recovery info if the current signature object doesnt contain it.
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toCompactBytes))]
    pub fn to_compact_bytes(&self, recovery_info: Option<RecoveryInfo>) -> Vec<u8> {
        // TODO: Test Compact Bytes length vs DER only
        let RecoveryInfo {
            is_y_odd,
            is_x_reduced,
            is_pubkey_compressed,
        } = recovery_info.or(self.recovery.clone()).unwrap_or(RecoveryInfo::default());

        let mut recovery = ((is_x_reduced as u8) << 1 | (is_y_odd as u8)) + 27 + 4;

        if !is_pubkey_compressed {
            recovery = recovery - 4
        }

        let mut compact_buf = vec![recovery];
        let r_bytes = &*self.sig.r().to_bytes();
        compact_buf.extend_from_slice(r_bytes);

        let s_bytes = &*self.sig.s().to_bytes();
        compact_buf.extend_from_slice(s_bytes);

        compact_buf
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = verifyMessage))]
    pub fn verify_message(&self, message: &[u8], pub_key: &PublicKey) -> bool {
        ECDSA::verify_digest_impl(message, pub_key, self, SigningHash::Sha256).is_ok()
    }
}

/**
 * WASM Exported Methods
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
impl Signature {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromDER))]
    pub fn from_der(bytes: &[u8], is_recoverable: bool) -> Result<Signature, JsValue> {
        Signature::from_der_impl(bytes, is_recoverable).map_err(|e| throw_str(&e.to_string()))
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHexDER))]
    pub fn from_hex_der(hex: &str, is_recoverable: bool) -> Result<Signature, JsValue> {
        match Signature::from_hex_der_impl(hex, is_recoverable) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromCompactBytes))]
    pub fn from_compact_bytes(compact_bytes: &[u8]) -> Result<Signature, JsValue> {
        match Signature::from_compact_impl(compact_bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = recoverPublicKey))]
    pub fn recover_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, JsValue> {
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
    pub fn from_der(bytes: &[u8]) -> Result<Signature, BSVErrors> {
        Signature::from_der_impl(bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex_der(hex: &str) -> Result<Signature, BSVErrors> {
        Signature::from_hex_der_impl(hex)
    }

    pub fn from_compact_bytes(compact_bytes: &[u8]) -> Result<Signature, BSVErrors> {
        Signature::from_compact_impl(compact_bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn recover_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, BSVErrors> {
        Signature::get_public_key(self, message, hash_algo)
    }
}
