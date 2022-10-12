use crate::BSVErrors;
use crate::ECIESCiphertext;
use crate::ECDSA;
use crate::ECIES;
use crate::{Hash, PublicKey, SigningHash};
use crate::{Signature, ToHex};
use elliptic_curve::sec1::ToEncodedPoint;
use k256::SecretKey;
use rand_core::OsRng;

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
    pub(crate) fn sign_message_impl(&self, msg: &[u8]) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_deterministic_k_impl(self, msg, SigningHash::Sha256, false)
    }

    pub(crate) fn to_wif_impl(&self) -> Result<String, BSVErrors> {
        // 1. Get Private Key hex
        let priv_key_hex = self.to_hex();

        // 2. Add 0x80 in front + 0x01 to end if compressed pub key

        let padded_hex = match self.is_pub_key_compressed {
            true => format!("80{}01", priv_key_hex),
            false => format!("80{}", priv_key_hex),
        };

        // 3. SHA256d
        let bytes = hex::decode(padded_hex.clone())?;

        let shad_hex = Hash::sha_256d(&bytes).to_bytes();

        // 4. Take first 4 bytes as checksum
        let checksum = shad_hex.to_vec()[0..4].to_hex();

        // 5. Add checksum to end of padded private key
        let extended_key = format!("{}{}", padded_hex, checksum);

        // 6 Base58 Result
        let extended_key_bytes = hex::decode(extended_key)?;

        Ok(bs58::encode(extended_key_bytes).into_string())
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<PrivateKey, BSVErrors> {
        let secret_key = SecretKey::from_be_bytes(bytes)?;

        Ok(PrivateKey {
            secret_key,
            is_pub_key_compressed: true,
        })
    }

    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<PrivateKey, BSVErrors> {
        let bytes = hex::decode(hex_str)?;

        Self::from_bytes_impl(&bytes)
    }

    pub(crate) fn from_wif_impl(wif_string: &str) -> Result<PrivateKey, BSVErrors> {
        // 1. Decode from Base58
        let wif_bytes = bs58::decode(wif_string).into_vec()?;
        let wif_without_checksum = wif_bytes[0..wif_bytes.len() - 4].to_vec();

        // 2. Check the Checksum
        let checksum = wif_bytes[wif_bytes.len() - 4..].to_hex();
        let check_hash = Hash::sha_256d(&wif_without_checksum).to_bytes();
        let check_string = check_hash.to_vec()[0..4].to_hex();

        if check_string.ne(&checksum) {
            return Err(BSVErrors::FromWIF("Checksum does not match!".into()));
        }

        // Private Key is 32 bytes + prefix is 33 bytes, if 34 bytes and ends with 01, compressed is true
        fn is_compressed(unchecksum: &[u8]) -> bool {
            if unchecksum.len() < 34 {
                return false;
            }

            match unchecksum.last() {
                Some(last_byte) => last_byte.eq(&1),
                None => false,
            }
        }

        let is_compressed_pub_key = is_compressed(&wif_without_checksum);
        // 3. Check if compressed public key, return private key string
        let private_key_hex = match is_compressed_pub_key {
            true => wif_without_checksum[1..wif_without_checksum.len() - 1].to_hex(),
            false => wif_without_checksum[1..].to_hex(),
        };

        Ok(PrivateKey::from_hex_impl(&private_key_hex)?.compress_public_key(is_compressed_pub_key))
    }

    pub(crate) fn to_public_key_impl(&self) -> Result<PublicKey, BSVErrors> {
        let pub_key = PublicKey::from_private_key_impl(self);

        if !self.is_pub_key_compressed {
            return pub_key.to_decompressed_impl();
        }

        Ok(pub_key)
    }

    /**
     * Encrypt a message to the public key of this private key.
     */
    pub(crate) fn encrypt_message_impl(&self, message: &[u8]) -> Result<ECIESCiphertext, BSVErrors> {
        ECIES::encrypt_impl(message, self, &self.to_public_key_impl()?, false)
    }

    /**
     * Decrypt a message that was sent to the public key corresponding to this private key.
     */
    pub(crate) fn decrypt_message_impl(&self, ciphertext: &ECIESCiphertext, sender_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        ECIES::decrypt_impl(ciphertext, self, sender_pub_key)
    }
}

impl PrivateKey {
    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.secret_key.to_be_bytes().to_vec()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        let secret_key_bytes = self.to_bytes();
        hex::encode(secret_key_bytes)
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromRandom))]
    pub fn from_random() -> PrivateKey {
        let secret_key = k256::SecretKey::random(&mut OsRng);

        PrivateKey {
            secret_key,
            is_pub_key_compressed: true,
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPoint))]
    /**
     * Finds the Public Key Point.
     */
    pub fn get_point(&self) -> Vec<u8> {
        self.secret_key.public_key().as_affine().to_encoded_point(self.is_pub_key_compressed).as_bytes().into()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = compressPublicKey))]
    pub fn compress_public_key(&self, should_compress: bool) -> PrivateKey {
        let mut priv_key = self.clone();
        priv_key.is_pub_key_compressed = should_compress;
        priv_key
    }
}

/**
 * Native Exported Methods
 */
impl PrivateKey {
    pub fn to_wif(&self) -> Result<String, BSVErrors> {
        PrivateKey::to_wif_impl(self)
    }

    pub fn from_wif(wif_string: &str) -> Result<PrivateKey, BSVErrors> {
        PrivateKey::from_wif_impl(wif_string)
    }

    pub fn from_hex(hex_str: &str) -> Result<PrivateKey, BSVErrors> {
        PrivateKey::from_hex_impl(hex_str)
    }

    /**
     * Standard ECDSA Message Signing using SHA256 as the digestg
     */
    pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, BSVErrors> {
        PrivateKey::sign_message_impl(self, msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey, BSVErrors> {
        Self::from_bytes_impl(bytes)
    }

    pub fn to_public_key(&self) -> Result<PublicKey, BSVErrors> {
        self.to_public_key_impl()
    }

    /**
     * Encrypt a message to the public key of this private key.
     */
    pub fn encrypt_message(&self, message: &[u8]) -> Result<ECIESCiphertext, BSVErrors> {
        self.encrypt_message_impl(message)
    }

    /**
     * Decrypt a message that was sent to the public key corresponding to this private key.
     */
    pub fn decrypt_message(&self, ciphertext: &ECIESCiphertext, sender_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        self.decrypt_message_impl(ciphertext, sender_pub_key)
    }
}
