use crate::{BSVErrors, Hash, PrivateKey, PublicKey, AES};
use elliptic_curve::sec1::ToEncodedPoint;
use k256::PublicKey as K256PublicKey;

pub mod ecies_ciphertext;
pub use ecies_ciphertext::*;

/**
 * Electrum compatible ECIES implementation.
 * Comparable to Ecies.electrumEncrypt in BSV.JS
 */
#[derive(Clone)]
pub struct ECIES {}

#[derive(Clone)]
pub struct CipherKeys {
    pub(crate) iv: Vec<u8>,
    pub(crate) ke: Vec<u8>,
    pub(crate) km: Vec<u8>,
}

impl CipherKeys {
    pub fn get_iv(&self) -> Vec<u8> {
        self.iv.clone()
    }

    pub fn get_ke(&self) -> Vec<u8> {
        self.ke.clone()
    }

    pub fn get_km(&self) -> Vec<u8> {
        self.km.clone()
    }
}

impl ECIES {
    pub(crate) fn encrypt_impl(message: &[u8], private_key: &PrivateKey, recipient_pub_key: &PublicKey, exclude_pub_key: bool) -> Result<ECIESCiphertext, BSVErrors> {
        let cipher = ECIES::derive_cipher_keys_impl(private_key, recipient_pub_key)?;
        let cipher_text = AES::encrypt_impl(&cipher.ke, &cipher.iv, message, crate::AESAlgorithms::AES128_CBC)?;

        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(b"BIE1");

        let r_buf = match exclude_pub_key {
            true => None,
            false => {
                let pub_key = private_key.to_public_key_impl()?.to_compressed_impl()?.to_bytes_impl()?;
                buffer.extend_from_slice(&pub_key);
                Some(pub_key)
            }
        };
        buffer.extend_from_slice(&cipher_text);

        let hmac = Hash::sha_256_hmac(&buffer, &cipher.km).to_bytes();

        Ok(ECIESCiphertext {
            ciphertext_bytes: cipher_text,
            public_key_bytes: r_buf,
            hmac_bytes: hmac,
            keys: Some(cipher),
        })
    }

    /**
     * Encrypt with a randomly generate private key.
     * This is intended to be used if you want to anonymously send a party an encrypted message.
     */
    pub(crate) fn encrypt_with_ephemeral_private_key_impl(message: &[u8], recipient_pub_key: &PublicKey) -> Result<ECIESCiphertext, BSVErrors> {
        let private_key = PrivateKey::from_random();
        ECIES::encrypt_impl(message, &private_key, recipient_pub_key, false)
    }

    pub(crate) fn decrypt_impl(ciphertext: &ECIESCiphertext, recipient_priv_key: &PrivateKey, sender_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        let cipher_keys = ECIES::derive_cipher_keys_impl(recipient_priv_key, sender_pub_key)?;

        let hmac = &ciphertext.hmac_bytes;

        let mut preimage = b"BIE1".to_vec();
        if let Some(pk) = &ciphertext.public_key_bytes {
            preimage.extend_from_slice(pk);
        }
        preimage.extend_from_slice(&ciphertext.ciphertext_bytes);

        let verify_hmac = Hash::sha_256_hmac(&preimage, &cipher_keys.km);

        if hmac != &verify_hmac.to_bytes() {
            return Err(BSVErrors::ECIESError("Invalid Checksum".into()));
        }

        let plain_text = AES::decrypt_impl(&cipher_keys.ke, &cipher_keys.iv, &ciphertext.ciphertext_bytes, crate::AESAlgorithms::AES128_CBC)?;
        Ok(plain_text)
    }

    pub(crate) fn derive_cipher_keys_impl(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<CipherKeys, BSVErrors> {
        let private_scalar = *priv_key.secret_key.to_nonzero_scalar();
        let pub_key_point = K256PublicKey::from_sec1_bytes(&pub_key.to_bytes_impl()?)?.to_projective();

        let shared_point = pub_key_point * private_scalar;
        let shared_pub = K256PublicKey::from_affine(shared_point.to_affine())?;

        let shared_mod_point = shared_pub.to_encoded_point(true);
        let hash = Hash::sha_512(shared_mod_point.as_bytes()).to_bytes();

        Ok(CipherKeys {
            iv: hash[0..16].into(),
            ke: hash[16..32].into(),
            km: hash[32..64].into(),
        })
    }
}




impl ECIES {
    pub fn encrypt(message: &[u8], sender_priv_key: &PrivateKey, recipient_pub_key: &PublicKey, exclude_pub_key: bool) -> Result<ECIESCiphertext, BSVErrors> {
        ECIES::encrypt_impl(message, sender_priv_key, recipient_pub_key, exclude_pub_key)
    }

    /**
     * Encrypt with a randomly generate private key.
     * This is intended to be used if you want to anonymously send a party an encrypted message.
     */
    pub fn encrypt_with_ephemeral_private_key(message: &[u8], recipient_pub_key: &PublicKey) -> Result<ECIESCiphertext, BSVErrors> {
        ECIES::encrypt_with_ephemeral_private_key_impl(message, recipient_pub_key)
    }

    pub fn decrypt(ciphertext: &ECIESCiphertext, recipient_priv_key: &PrivateKey, sender_pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
        ECIES::decrypt_impl(ciphertext, recipient_priv_key, sender_pub_key)
    }

    pub fn derive_cipher_keys(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<CipherKeys, BSVErrors> {
        ECIES::derive_cipher_keys_impl(priv_key, pub_key)
    }
}
