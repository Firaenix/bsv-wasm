use crate::PrivateKey;
use crate::{BSVErrors, ECIESCiphertext, P2PKHAddress, Signature, SigningHash, ECDSA, ECIES};
use elliptic_curve::{sec1::*, subtle::Choice};
use k256::{AffinePoint, Secp256k1};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    point: Vec<u8>,
    is_compressed: bool,
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pubkey = self.to_hex_impl().map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        serializer.serialize_str(&pubkey)
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pubkey_string = String::deserialize(deserializer)?;

        let p2pkh = PublicKey::from_hex_impl(&pubkey_string).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(p2pkh)
    }
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

impl PublicKey {
    pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
        self.verify_message_impl(message, signature).is_ok()
    }

    pub fn is_compressed(&self) -> bool {
        self.is_compressed
    }
}

/**
 * Native Exported Methods
 */
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
