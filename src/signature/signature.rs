use crate::{PublicKey, Sha256r, SignatureErrors, SigningHash};
use anyhow::*;
use digest::Digest;
use k256::{
    ecdsa::Signature as SecpSignature,
    ecdsa::{recoverable, signature::Verifier, VerifyingKey},
    EncodedPoint, FieldBytes,
};
use wasm_bindgen::{prelude::*, throw_str};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    sig: k256::ecdsa::Signature,
    is_recoverable: bool,
}

/**
 * Implementation Methods
 */
impl Signature {
    pub(crate) fn from_der_impl(
        bytes: Vec<u8>,
        is_recoverable: bool,
    ) -> Result<Signature, SignatureErrors> {
        let sig = match SecpSignature::from_der(&bytes) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::SecpError { error: e }),
        };

        Ok(Signature {
            sig,
            is_recoverable,
        })
    }

    pub(crate) fn from_hex_der_impl(
        hex: String,
        is_recoverable: bool,
    ) -> Result<Signature, SignatureErrors> {
        let bytes = match hex::decode(hex) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::ParseHex { error: e }),
        };

        let sig = match SecpSignature::from_der(&bytes) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::SecpError { error: e }),
        };

        Ok(Signature {
            sig,
            is_recoverable,
        })
    }

    pub(crate) fn get_public_key(
        &self,
        message: &[u8],
        hash_algo: SigningHash,
    ) -> Result<PublicKey, SignatureErrors> {
        if self.is_recoverable == false {
            return Err(SignatureErrors::DerivePublicKey {
                error: anyhow!("This signature is not recoverable"),
            });
        }

        let recovery_id = match recoverable::Id::new(self.is_recoverable as u8) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::DerivePublicKey { error: anyhow!(e) }),
        };

        let recoverable_sig = match recoverable::Signature::new(&self.sig, recovery_id) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::DerivePublicKey { error: anyhow!(e) }),
        };

        let message_digest = match hash_algo {
            SigningHash::Sha256 => Sha256r::default().chain(message.clone()),
            SigningHash::Sha256d => Sha256r::default().chain(Sha256r::digest(message.clone())),
        };

        let verify_key = match recoverable_sig.recover_verify_key_from_digest(message_digest) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::DerivePublicKey { error: anyhow!(e) }),
        };

        let pub_key = match PublicKey::from_bytes_impl(&verify_key.to_bytes().to_vec(), true) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::DerivePublicKey { error: anyhow!(e) }),
        };

        Ok(pub_key)
    }

    pub(crate) fn to_hex_impl(&self) -> String {
        let bytes = self.sig.to_der();

        hex::encode(bytes)
    }

    pub(crate) fn to_der_bytes_impl(&self) -> Vec<u8> {
        let bytes = self.sig.to_der();

        bytes.as_bytes().to_vec()
    }

    pub(crate) fn verify_impl(
        &self,
        message: Vec<u8>,
        pub_key: &PublicKey,
    ) -> Result<bool, SignatureErrors> {
        let pub_key_bytes = match pub_key.to_bytes_impl() {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::PublicKeyError { error: e }),
        };

        let point = match EncodedPoint::from_bytes(pub_key_bytes) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::InvalidPoint { error: e }),
        };

        let key = match VerifyingKey::from_encoded_point(&point) {
            Ok(v) => v,
            Err(e) => return Err(SignatureErrors::SecpError { error: e }),
        };

        Ok(key.verify(&message, &self.sig).is_ok())
    }
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(js_name = toDER)]
    pub fn to_der_bytes(&self) -> Vec<u8> {
        Signature::to_der_bytes_impl(&self)
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

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        Signature::to_hex_impl(&self)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = verify))]
    pub fn verify(&self, message: Vec<u8>, pub_key: &PublicKey) -> Result<bool, JsValue> {
        match Signature::verify_impl(&self, message, pub_key) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = recoverPublicKey)]
    pub fn recover_public_key(
        &self,
        message: Vec<u8>,
        hash_algo: SigningHash,
    ) -> Result<PublicKey, JsValue> {
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
    pub fn from_der(bytes: Vec<u8>, is_recoverable: bool) -> Result<Signature, SignatureErrors> {
        Signature::from_der_impl(bytes, is_recoverable)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex_der(hex: String, is_recoverable: bool) -> Result<Signature, SignatureErrors> {
        Signature::from_hex_der_impl(hex, is_recoverable)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_hex(&self) -> String {
        Signature::to_hex_impl(&self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn verify(&self, message: Vec<u8>, pub_key: &PublicKey) -> Result<bool, SignatureErrors> {
        Signature::verify_impl(&self, message, pub_key)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn recover_public_key(
        &self,
        message: Vec<u8>,
        hash_algo: SigningHash,
    ) -> Result<PublicKey, SignatureErrors> {
        Signature::get_public_key(&self, &message, hash_algo)
    }
}
