use crate::{BSVErrors, ECIESCiphertext, P2PKHAddress, Signature, SigningHash, ECDSA, ECIES};
use elliptic_curve::{sec1::*, subtle::Choice};
use k256::{AffinePoint, Secp256k1};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str};

use crate::PrivateKey;

#[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    point: Vec<u8>,
    is_compressed: bool,
}

impl PublicKey {
    pub(crate) fn from_private_key_impl(priv_key: &PrivateKey) -> PublicKey {
        PublicKey {
            point: priv_key.get_point(),
            is_compressed: priv_key.is_pub_key_compressed,
        }
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        let bytes = self.to_bytes_impl()?;
        Ok(hex::encode(bytes))
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        Ok(self.point.clone())
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<PublicKey, BSVErrors> {
        let point = EncodedPoint::<Secp256k1>::from_bytes(bytes).map_err(|e| BSVErrors::PublicKeyError(e.to_string()))?;
        Ok(PublicKey::from_encoded_point(&point))
    }

    fn from_encoded_point(point: &EncodedPoint<Secp256k1>) -> PublicKey {
        PublicKey {
            point: point.as_bytes().to_vec(),
            is_compressed: point.is_compressed(),
        }
    }

    pub(crate) fn to_decompressed_impl(&self) -> Result<PublicKey, BSVErrors> {
        use elliptic_curve::DecompressPoint;

        let point = EncodedPoint::<Secp256k1>::from_bytes(&self.point).unwrap();

        let decompressed_point: EncodedPoint<Secp256k1> = match point.coordinates() {
            Coordinates::Compressed { x, y_is_odd } => AffinePoint::decompress(x, Choice::from(y_is_odd as u8)).map(|s| s.to_encoded_point(false)).into(),
            Coordinates::Compact { .. } | Coordinates::Identity => None,
            Coordinates::Uncompressed { .. } => Some(point),
        }
        .unwrap();

        Ok(PublicKey::from_encoded_point(&decompressed_point))
    }

    pub(crate) fn to_compressed_impl(&self) -> Result<PublicKey, BSVErrors> {
        let point = EncodedPoint::<Secp256k1>::from_bytes(&self.point).map_err(|e| BSVErrors::PublicKeyError(e.to_string()))?;
        Ok(PublicKey::from_encoded_point(&point.compress()))
    }

    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<PublicKey, BSVErrors> {
        let point_bytes = hex::decode(hex_str)?;
        PublicKey::from_bytes_impl(&point_bytes)
    }

    /**
     * Standard ECDSA Message Verification
     */
    pub(crate) fn verify_message_impl(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, self, signature, SigningHash::Sha256)
    }

    pub(crate) fn to_p2pkh_address_impl(&self) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_pubkey_impl(self)
    }

    /**
     * Encrypt a message to be sent to this public key with the provided private key.
     */
    pub(crate) fn encrypt_message_impl(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, BSVErrors> {
        ECIES::encrypt_impl(message, sender_private_key, self, false)
    }
}

#[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen)]
impl PublicKey {
    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = isValidMessage))]
    pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
        self.verify_message_impl(message, signature).is_ok()
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = isCompressed))]
    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }
}

/**
 * WASM Exported Methods
 */
#[cfg(target_arch = "wasm32")]
#[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen)]
impl PublicKey {
    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: &str) -> Result<PublicKey, wasm_bindgen::JsError> {
       Ok(PublicKey::from_hex_impl(hex_str)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, wasm_bindgen::JsError> {
       Ok(PublicKey::from_bytes_impl(bytes)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
       Ok(PublicKey::to_bytes_impl(&self)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
       Ok(PublicKey::to_hex_impl(&self)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromPrivateKey))]
    pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
        PublicKey::from_private_key_impl(priv_key)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = verifyMessage))]
    pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
       Ok(self.verify_message_impl(message, signature)?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toAddress))]
    pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
       Ok(self.to_p2pkh_address_impl()?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toCompressed))]
    pub fn to_compressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
       Ok(self.to_compressed_impl()?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toDecompressed))]
    pub fn to_decompressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
       Ok(self.to_decompressed_impl()?)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = encryptMessage))]
    pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
       Ok(self.encrypt_message_impl(message, sender_private_key)?)
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PublicKey {
    pub fn from_hex(hex_str: &str) -> Result<PublicKey, BSVErrors> {
        PublicKey::from_hex_impl(hex_str)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, BSVErrors> {
        PublicKey::from_bytes_impl(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        PublicKey::to_bytes_impl(self)
    }

    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        PublicKey::to_hex_impl(self)
    }

    pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
        PublicKey::from_private_key_impl(priv_key)
    }

    pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        self.verify_message_impl(message, signature)
    }

    pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, BSVErrors> {
        self.to_p2pkh_address_impl()
    }

    pub fn to_compressed(&self) -> Result<PublicKey, BSVErrors> {
        self.to_compressed_impl()
    }

    pub fn to_decompressed(&self) -> Result<PublicKey, BSVErrors> {
        self.to_decompressed_impl()
    }

    pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, BSVErrors> {
        self.encrypt_message_impl(message, sender_private_key)
    }
}
