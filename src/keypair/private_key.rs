use crate::get_hash_digest;
use crate::{sha256r_digest::Sha256r, ECDSA};
use crate::{Hash, PublicKey, SigningHash};
use crate::{Signature, ToHex};
use anyhow::*;
use k256::ecdsa::digest::Digest;
use k256::ecdsa::recoverable;
use k256::{EncodedPoint, SecretKey};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub(crate) secret_key: SecretKey,
    pub(crate) is_pub_key_compressed: bool,
}

/**
 * Internal Methods
 */
impl PrivateKey {
    /**
     * Standard ECDSA Message Signing
     */
    pub(crate) fn sign_message_impl(&self, msg: &[u8]) -> Result<Signature> {
        ECDSA::sign_with_deterministic_k_impl(self, msg, SigningHash::Sha256, false)
    }

    pub(crate) fn to_wif_impl(&self) -> Result<String> {
        // 1. Get Private Key hex
        let priv_key_hex = self.to_hex();

        // 2. Add 0x80 in front + 0x01 to end if compressed pub key

        let padded_hex = match self.is_pub_key_compressed {
            true => format!("80{}01", priv_key_hex),
            false => format!("80{}", priv_key_hex),
        };

        // 3. SHA256d
        let bytes = match hex::decode(padded_hex.clone()) {
            Ok(v) => v,
            Err(e) => wasm_bindgen::throw_str(&e.to_string()),
        };

        let shad_hex = Hash::sha_256d(&bytes).to_bytes();

        // 4. Take first 4 bytes as checksum
        let checksum = shad_hex.to_vec()[0..4].to_hex();

        // 5. Add checksum to end of padded private key
        let extended_key = format!("{}{}", padded_hex, checksum);

        // 6 Base58 Result
        let extended_key_bytes = hex::decode(extended_key)?;

        Ok(bs58::encode(extended_key_bytes).into_string())
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<PrivateKey> {
        let secret_key = SecretKey::from_bytes(bytes)?;

        Ok(PrivateKey {
            secret_key,
            is_pub_key_compressed: true,
        })
    }

    pub(crate) fn from_hex_impl(hex_str: String) -> Result<PrivateKey> {
        let bytes = hex::decode(hex_str)?;

        Ok(Self::from_bytes_impl(&bytes)?)
    }

    pub(crate) fn from_wif_impl(wif_string: String) -> Result<PrivateKey> {
        // 1. Decode from Base58
        let wif_bytes = bs58::decode(wif_string.clone()).into_vec()?;
        let wif_without_checksum = wif_bytes[0..wif_bytes.len() - 4].to_vec();

        // 2. Check the Checksum
        let checksum = wif_bytes[wif_bytes.len() - 4..].to_hex();
        let check_hash = Hash::sha_256d(&wif_without_checksum).to_bytes();
        let check_string = check_hash.to_vec()[0..4].to_hex();

        if check_string.ne(&checksum) {
            return Err(anyhow!("Checksum does not match! Invalid WIF"));
        }

        // Private Key is 32 bytes + prefix is 33 bytes, if 34 bytes and ends with 01, compressed is true
        fn is_compressed(unchecksum: &Vec<u8>) -> bool {
            if unchecksum.len() < 34 {
                return false;
            }

            match unchecksum.last() {
                Some(last_byte) => last_byte.eq(&01),
                None => false,
            }
        }

        let is_compressed_pub_key = is_compressed(&wif_without_checksum);
        // 3. Check if compressed public key, return private key string
        let private_key_hex = match is_compressed_pub_key {
            true => wif_without_checksum[1..wif_without_checksum.len() - 1].to_hex(),
            false => wif_without_checksum[1..].to_hex(),
        };

        Ok(PrivateKey::from_hex_impl(private_key_hex.into())?.compress_public_key(is_compressed_pub_key))
    }

    pub(crate) fn get_public_key_impl(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_private_key_impl(&self);

        if !self.is_pub_key_compressed {
            return Ok(pub_key.to_decompressed_impl()?);
        }

        return Ok(pub_key);
    }
}

#[wasm_bindgen]
impl PrivateKey {
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.secret_key.to_bytes().to_vec()
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        let secret_key_bytes = self.to_bytes();
        hex::encode(secret_key_bytes)
    }

    #[wasm_bindgen(js_name = fromRandom)]
    pub fn from_random() -> PrivateKey {
        let secret_key = k256::SecretKey::random(&mut OsRng);

        PrivateKey {
            secret_key,
            is_pub_key_compressed: true,
        }
    }

    #[wasm_bindgen(js_name = getPoint)]
    /**
     * Finds the Public Key Point.
     * Always returns the compressed point.
     * To get the decompressed point: PublicKey::from_bytes(point).to_decompressed()
     */
    pub fn get_point(&self) -> Vec<u8> {
        EncodedPoint::from_secret_key(&self.secret_key, true).as_bytes().into()
    }

    #[wasm_bindgen(js_name = compressPublicKey)]
    pub fn compress_public_key(&self, should_compress: bool) -> PrivateKey {
        let mut priv_key = self.clone();
        priv_key.is_pub_key_compressed = should_compress;
        priv_key
    }
}

/**
 * WASM Exported Methods
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PrivateKey {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromWIF))]
    pub fn from_wif(wif_string: String) -> Result<PrivateKey, JsValue> {
        match PrivateKey::from_wif_impl(wif_string) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: String) -> Result<PrivateKey, JsValue> {
        match PrivateKey::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = signMessage))]
    /**
     * Standard ECDSA Message Signing using SHA256 as the digestg
     */
    pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, JsValue> {
        match PrivateKey::sign_message_impl(&self, msg) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toWIF)]
    pub fn to_wif(&self) -> Result<String, JsValue> {
        match PrivateKey::to_wif_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey, JsValue> {
        match Self::from_bytes_impl(bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = getPublicKey)]
    pub fn get_public_key(&self) -> Result<PublicKey, JsValue> {
        match self.get_public_key_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PrivateKey {
    pub fn to_wif(&self) -> Result<String> {
        PrivateKey::to_wif_impl(&self)
    }

    pub fn from_wif(wif_string: String) -> Result<PrivateKey> {
        PrivateKey::from_wif_impl(wif_string)
    }

    pub fn from_hex(hex_str: String) -> Result<PrivateKey> {
        PrivateKey::from_hex_impl(hex_str)
    }

    /**
     * Standard ECDSA Message Signing using SHA256 as the digestg
     */
    pub fn sign_message(&self, msg: &[u8]) -> Result<Signature> {
        PrivateKey::sign_message_impl(&self, msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey> {
        Self::from_bytes_impl(bytes)
    }

    pub fn get_public_key(&self) -> Result<PublicKey> {
        self.get_public_key_impl()
    }
}
