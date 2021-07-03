
use crate::sign_custom_preimage;
use crate::Hash;
use crate::PrivateKeyErrors;
use crate::Sha256d;
use crate::ToHex;
use crate::Signature;
use crate::sha256_digest::Sha256;
use anyhow::*;
use digest::FixedOutput;
use digest::FixedOutputDirty;
use k256::ecdsa::digest::Digest;
use k256::ecdsa::{signature::Signer, Signature as SecpSignature, SigningKey};
use k256::ecdsa::signature::DigestSigner;
use k256::{EncodedPoint, SecretKey};
use rand_core::OsRng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PrivateKey {
  secret_key: SecretKey,
}

pub enum KTypes {
  Sha256,
  Sha256r,
}

/**
 * Internal Methods
 */
impl PrivateKey {
  /**
   * SHA256s and then signs the specified message.
   * Secp256k1 signature inputs must be 32 bytes in length, SHA256 is to ensure this.
   */
  pub(crate) fn sign_message_impl(&self, msg: &[u8]) -> Result<Signature, PrivateKeyErrors> {
    let signing_key = match SigningKey::from_bytes(&self.secret_key.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(PrivateKeyErrors::ByteDecode { error: anyhow!(e) }),
    };

    // let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = msg;

    // Note: the signature type must be annotated or otherwise inferrable as
    // `Signer` has many impls of the `Signer` trait (for both regular and
    // recoverable signature types).
    let mut signature: SecpSignature = signing_key.sign(message);
    match Signature::from_der_impl(signature.to_der().as_bytes().to_vec()) {
      Ok(v) => Ok(v),
      Err(e) => Err(PrivateKeyErrors::SignatureError { error: e }),
    }
  }

  pub(crate) fn sign_with_k_impl(&self, preimage: &[u8]) -> Result<Signature, PrivateKeyErrors> {
    let signing_key = match SigningKey::from_bytes(&self.secret_key.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(PrivateKeyErrors::ByteDecode { error: anyhow!(e) }),
    };
    
    // let preimage_hash = Hash::sha_256(preimage.clone()).to_bytes();
    let engine = Sha256::new(true).chain(Sha256::digest(preimage.clone()));
    let non_reverse_engine = Sha256::new(false).chain(Sha256::digest(preimage.clone()));
    let double_hash = Hash::sha_256d(preimage).to_bytes();
    // double_hash.reverse();

    assert_eq!(&signing_key.to_bytes().to_vec(), &self.secret_key.to_bytes().to_vec(), "Signing key and secret key bytes are diff");

    let (custom_sig, is_recoverable) = sign_custom_preimage(&self.secret_key, &double_hash, engine.clone()).unwrap();

    // assert_eq!(ecdsa::signature::Signature::as_bytes(&signature.to_der()).to_hex(), "30450221009c230cdb72228135e3b9e27bcc58d366d89be2ddbc078dccac0b17568e11e41502201ebd424c1bbb9b807770bdc9f7db098ed158b541218ff102ae220ecf044c8df8", "NOPE");
    // let mut thing = vec![];
    // engine.;

    // let digest_256dr = engine.clone().finalize().to_vec();
    // assert_eq!(digest_256dr, hash.to_bytes(), "digests arent equal");

    // secret_scalar.try_sign_recoverable_prehashed(ephemeral_scalar, &k);

    // let signature: SecpSignature = signing_key.sign(&hash.to_bytes());
    // let signature: SecpSignature = signing_key.sign_digest(engine);

    // assert_eq!(signature.to_der().as_bytes(), custom_sig.to_der().as_bytes(), "Sigs arent equal");

    // signature.normalize_s().unwrap();
    match Signature::from_der_impl(custom_sig.to_der().as_bytes().to_vec()) {
      Ok(v) => Ok(v),
      Err(e) => Err(PrivateKeyErrors::SignatureError { error: e }),
    }
  }

  pub(crate) fn to_wif_impl(&self, compressed: bool) -> Result<String, PrivateKeyErrors> {
    // 1. Get Private Key hex
    let priv_key_hex = self.to_hex();

    // 2. Add 0x80 in front + 0x01 to end if compressed pub key
    let padded_hex = match compressed {
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
    let extended_key_bytes = match hex::decode(extended_key) {
      Ok(v) => v,
      Err(e) => return Err(PrivateKeyErrors::ByteDecode { error: anyhow!(e) }),
    };

    Ok(bs58::encode(extended_key_bytes).into_string())
  }

  pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<PrivateKey, PrivateKeyErrors> {
    let secret_key = match SecretKey::from_bytes(bytes) {
      Ok(key) => key,
      Err(e) => return Err(PrivateKeyErrors::SecretKey { error: anyhow!(e) }),
    };

    Ok(PrivateKey { secret_key })
  }

  pub(crate) fn from_hex_impl(hex_str: String) -> Result<PrivateKey, PrivateKeyErrors> {
    let bytes = match hex::decode(hex_str) {
      Ok(bytes) => bytes,
      Err(e) => return Err(PrivateKeyErrors::ByteDecode { error: anyhow!(e) }),
    };

    Self::from_bytes_impl(&bytes)
  }

  pub(crate) fn from_wif_impl(wif_string: String) -> Result<PrivateKey, PrivateKeyErrors> {
    // 1. Decode from Base58
    let wif_bytes = match bs58::decode(wif_string.clone()).into_vec() {
      Ok(v) => v,
      Err(e) => {
        return Err(PrivateKeyErrors::Base58Decode {
          string: wif_string,
          error: anyhow!(e),
        })
      }
    };

    let wif_without_checksum = wif_bytes[0..wif_bytes.len() - 4].to_vec();

    // 2. Check the Checksum
    let checksum = wif_bytes[wif_bytes.len() - 4..].to_hex();
    let check_hash = Hash::sha_256d(&wif_without_checksum).to_bytes();
    let check_string = check_hash.to_vec()[0..4].to_hex();

    if check_string.ne(&checksum) {
      throw_str("Checksum does not match! Invalid WIF");
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

    // 3. Check if compressed public key, return private key string

    let private_key_hex = match is_compressed(&wif_without_checksum) {
      true => wif_without_checksum[1..wif_without_checksum.len() - 1].to_hex(),
      false => wif_without_checksum[1..].to_hex(),
    };

    PrivateKey::from_hex_impl(private_key_hex.into())
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

    PrivateKey { secret_key }
  }

  #[wasm_bindgen(js_name = getPoint)]
  pub fn get_point(&self, compressed: bool) -> Vec<u8> {
    EncodedPoint::from_secret_key(&self.secret_key, compressed)
      .as_bytes()
      .into()
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


  #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = sign))]
  /**
   * SHA256s and then signs the specified message.
   * Secp256k1 signature inputs must be 32 bytes in length, SHA256 is to ensure this.
   */
  pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, JsValue> {
    match PrivateKey::sign_message_impl(&self, msg) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toWIF)]
  pub fn to_wif(&self, compressed: bool) -> Result<String, JsValue> {
    match PrivateKey::to_wif_impl(&self, compressed) {
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
}

/**
 * Native Exported Methods
 */
#[cfg(not(target_arch = "wasm32"))]
impl PrivateKey {
  pub fn to_wif(&self, compressed: bool) -> Result<String, PrivateKeyErrors> {
    PrivateKey::to_wif_impl(&self, compressed)
  }

  pub fn from_wif(wif_string: String) -> Result<PrivateKey, PrivateKeyErrors> {
    PrivateKey::from_wif_impl(wif_string)
  }

  pub fn from_hex(hex_str: String) -> Result<PrivateKey, PrivateKeyErrors> {
    PrivateKey::from_hex_impl(hex_str)
  }

  /**
   * SHA256s and then signs the specified message.
   * Secp256k1 signature inputs must be 32 bytes in length, SHA256 is to ensure this.
   */
  pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, PrivateKeyErrors> {
    PrivateKey::sign_message_impl(&self, msg)
  }

  pub fn sign_preimage(&self, preimage: &[u8]) -> Result<Signature, PrivateKeyErrors> {
    PrivateKey::sign_preimage_impl(&self, preimage)
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey, PrivateKeyErrors> {
    Self::from_bytes_impl(bytes)
  }
}
