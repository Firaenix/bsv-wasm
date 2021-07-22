use k256::ecdh::{*, self, };
use rand_core::OsRng;

use crate::{AES, BSVErrors, Hash, PrivateKey, PublicKey};

pub struct ECIES {}


pub struct ECDH {}

impl ECDH {
  /**
   * Derives the shared key between a recipients public key and an optional private key.
   */ 
  pub fn derive_shared_key(priv_key: Option<&PrivateKey>, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    let internal_key = k256::PublicKey::from_sec1_bytes(&pub_key.to_bytes()?)?;
    
    let shared = match priv_key {
      Some(p) => elliptic_curve::ecdh::diffie_hellman(p.secret_key.to_secret_scalar(), internal_key.as_affine()),
      None => ecdh::EphemeralSecret::random(&mut OsRng).diffie_hellman(&internal_key)
    };

    let bytes = shared.as_bytes();
    Ok(bytes.as_slice().to_vec())
  }
}

pub struct CipherKeys {
  pub iv: Vec<u8>,
  pub ke: Vec<u8>,
  pub km: Vec<u8>
}

impl ECIES {
  pub fn encrypt(message: &[u8], priv_key: Option<&PrivateKey>, pub_key: &PublicKey) -> Result<Vec<u8>, BSVErrors> {
    let shared_key = ECDH::derive_shared_key(priv_key, pub_key)?;
    let cipher = ECIES::derive_cipher_keys(&shared_key)?;

    let cipher_text = AES::encrypt_impl(&cipher.ke, &cipher.iv, message, crate::AESAlgorithms::AES256_CBC)?;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend_from_slice(b"BIE");
    if let Some(pk) = priv_key {
      let r_buf = pk.get_public_key()?.to_compressed()?.to_bytes()?;
      buffer.extend_from_slice(&r_buf);
    } 
    buffer.extend_from_slice(&cipher_text);

    let hmac = Hash::sha_256_hmac(&buffer, &cipher.km).to_bytes();
    buffer.extend_from_slice(&hmac);

    Ok(buffer)
  }

  pub fn decrypt(cipher_text: &[u8]) -> Result<Vec<u8>, BSVErrors> {
    if &cipher_text[0..=3] != b"BIE" {
      return Err(BSVErrors::DecryptionError("Cipher text did not start with BIE".into()))
    }


    Ok(vec![])
  }

  pub fn derive_cipher_keys(shared_key_bytes: &[u8]) -> Result<CipherKeys, BSVErrors> {
    let hash = Hash::sha_256(shared_key_bytes).to_bytes();

    Ok(CipherKeys {
      iv: hash[0..16].into(),
      ke: hash[16..32].into(),
      km: hash[32..64].into()
    })
  }
}