use crate::{ExtendedPrivateKeyErrors, HARDENED_KEY_OFFSET, KDF, XPRIV_VERSION_BYTE};
use std::{io::{Cursor, Read, Write}, ops::{Add}, vec};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use k256::{Scalar, SecretKey};

use anyhow::*;
use getrandom::*;

use wasm_bindgen::{prelude::*, throw_str};

use crate::{hash::Hash, PrivateKey, PublicKey};

#[wasm_bindgen]
pub struct ExtendedPrivateKey {
  private_key: PrivateKey,
  public_key: PublicKey,
  chain_code: Vec<u8>,
  depth: u8,
  index: u32,
  parent_fingerprint: Vec<u8>,
}

impl ExtendedPrivateKey {
  pub fn new(
    private_key: &PrivateKey,
    chain_code: &[u8],
    depth: &u8,
    index: &u32,
    parent_fingerprint: Option<&[u8]>,
  ) -> Self {
    let fingerprint = match parent_fingerprint {
      Some(v) => v,
      None => &[0, 0, 0, 0],
    };

    ExtendedPrivateKey {
      private_key: private_key.clone(),
      public_key: PublicKey::from_private_key_impl(private_key, true),
      chain_code: chain_code.to_vec(),
      depth: *depth,
      index: *index,
      parent_fingerprint: fingerprint.to_vec(),
    }
  }

  pub fn to_string_impl(&self) -> Result<String> {
    let mut buffer: Vec<u8> =  vec![];

    buffer.write_u32::<BigEndian>(XPRIV_VERSION_BYTE)
      .and_then(|_| buffer.write_u8(self.depth))
      .and_then(|_| buffer.write(&self.parent_fingerprint))
      .and_then(|_| buffer.write_u32::<BigEndian>(self.index))
      .and_then(|_| buffer.write(&self.chain_code))
      .and_then(|_| buffer.write_u8(0))
      .and_then(|_| buffer.write(&self.private_key.to_bytes()))?;

    let checksum = &Hash::sha_256d(&buffer).to_bytes()[0..4];
    buffer.write(checksum)?;

    Ok(bs58::encode(buffer).into_string())
  }

  pub fn from_mnemonic_and_passphrase_impl(mnemonic: Vec<u8>, passphrase: Option<Vec<u8>>) -> Result<Self, ExtendedPrivateKeyErrors> {
    let fixed_phrase = match passphrase {
      Some(v) => v,
      None => b"mnemonic".to_vec()
    };

    let seed =  KDF::pbkdf2(mnemonic, Some(fixed_phrase), crate::PBKDF2Hashes::SHA512, 2048, 64);
    let seed_bytes = seed.get_hash().to_bytes();
    Self::from_seed_impl(seed_bytes)
  }

  pub fn from_string_impl(xprv_string: &str) -> Result<Self> {
    let mut cursor = Cursor::new(bs58::decode(xprv_string).into_vec()?);

    // Skip the first 4 bytes "xprv"
    cursor.set_position(4);

    let depth = cursor.read_u8()?;
    let mut parent_fingerprint = vec![0; 4];
    cursor.read_exact(&mut parent_fingerprint)?;
    let index = cursor.read_u32::<BigEndian>()?;

    let mut chain_code = vec![0; 32];
    cursor.read_exact(&mut chain_code)?;

    // Skip appended 0 byte on private key
    cursor.set_position(cursor.position() + 1);

    let mut private_key_bytes = vec![0; 32];
    cursor.read_exact(&mut private_key_bytes)?;
    let private_key = match PrivateKey::from_bytes_impl(&private_key_bytes) {
      Ok(v) => v,
      Err(e) => return Err(anyhow!(e)),
    };
    let public_key = PublicKey::from_private_key_impl(&private_key, true);

    let mut checksum = vec![0; 4];
    cursor.read_exact(&mut checksum)?;

    Ok(ExtendedPrivateKey {
      private_key,
      public_key,
      chain_code,
      depth,
      index,
      parent_fingerprint,
    })
  }

  pub fn from_random_impl() -> Result<Self, ExtendedPrivateKeyErrors> {
    let mut seed = vec![0; 64];
    match getrandom(&mut seed)  {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPrivateKeyErrors::RandomnessGenerationError{ error: anyhow!(e) }),
    };

    Self::from_seed_impl(seed)
  }

  pub fn from_seed_impl(seed: Vec<u8>) -> Result<Self, ExtendedPrivateKeyErrors> {
    let seed_hmac = Hash::sha_512_hmac(&seed, b"Bitcoin seed");

    let seed_bytes = seed_hmac.to_bytes();
    let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    let private_key_bytes = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError {
        error: anyhow!("Could not get 32 bytes for private key"),
      })
    };
    let chain_code = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError {
        error: anyhow!("Could not get 32 bytes for chain code"),
      })
    };

    let priv_key = match PrivateKey::from_bytes_impl(private_key_bytes) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPrivateKeyError { error: e }),
    };

    let pub_key = PublicKey::from_private_key_impl(&priv_key, true);

    Ok(Self {
      private_key: priv_key.clone(),
      public_key: pub_key.clone(),
      chain_code: chain_code.to_vec(),
      depth: 0,
      index: 0,
      parent_fingerprint: [0, 0, 0, 0].to_vec(),
    })
  }

  pub fn derive_impl(&self, index: u32) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    if self.depth >= 255 {
      return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!("Cannot derive from depth of more than 255") });
    }

    let is_hardened = index >= HARDENED_KEY_OFFSET;

    let key_data = match is_hardened {
      true => {
        let mut bytes: Vec<u8> = vec![];

        bytes.push(0x0);
        bytes.extend_from_slice(&self.private_key.clone().to_bytes());
        bytes.extend_from_slice(&index.clone().to_be_bytes());
        bytes
      }
      false => {
        let mut bytes: Vec<u8> = vec![];

        let pub_key_bytes = &match self.public_key.clone().to_bytes_impl() {
          Ok(v) => v,
          Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPublicKeyError { error: e }),
        };

        bytes.extend_from_slice(&pub_key_bytes);
        bytes.extend_from_slice(&index.clone().to_be_bytes());
        bytes
      }
    };

    let pub_key_bytes = &match self.public_key.clone().to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPublicKeyError { error: e }),
    };
    let hash = Hash::hash_160(&pub_key_bytes);
    let fingerprint = &hash.to_bytes()[0..4];

    let hmac = Hash::sha_512_hmac(&key_data, &self.chain_code.clone());
    let seed_bytes = hmac.to_bytes();

    let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    // let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    let private_key_bytes = match seed_chunks.next() {
      Some(b) => b,
      None => {
        return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError {
          error: anyhow!("Could not get 32 bytes for private key"),
        })
      }
    };
    let child_chain_code = match seed_chunks.next() {
      Some(b) => b,
      None => {
        return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError {
          error: anyhow!("Could not get 32 bytes for chain code"),
        })
      }
    };

    let parent_scalar = match SecretKey::from_bytes(self.private_key.clone().to_bytes().as_slice()) {
      Ok(v) => v.to_secret_scalar().clone(),
      Err(e) => return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!(e) })
    };

    let il_scalar = match SecretKey::from_bytes(private_key_bytes) {
      Ok(il) => Scalar::from_bytes_reduced(&il.to_secret_scalar().to_bytes()),
      Err(e) => return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!(e) })
    };

    // child_private_key = il + parent_key % n
    let derived_private_key = parent_scalar.add(il_scalar);

    let child_private_key = match PrivateKey::from_bytes_impl(&derived_private_key.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPrivateKeyError { error: e }),
    };

    let child_chain_code_bytes = child_chain_code.to_vec();
    let child_pub_key = PublicKey::from_private_key_impl(&child_private_key, true);

    Ok(ExtendedPrivateKey {
      chain_code: child_chain_code_bytes,
      private_key: child_private_key,
      public_key: child_pub_key,
      depth: self.depth + 1,
      index,
      parent_fingerprint: fingerprint.to_vec(),
    })
  }

  pub fn derive_from_path_impl(&self, path: &str) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    if path.to_ascii_lowercase().starts_with('m') == false {
      return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!("Path did not begin with 'm'") });
    }

    let children = path[1..].split('/').filter(|x| -> bool { *x != "" });
    let child_indices = children.map(Self::parse_str_to_idx).collect::<Result<Vec<u32>, ExtendedPrivateKeyErrors>>()?; 

    if child_indices.len() <= 0 {
      return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!(format!("No path was provided. Please provide a string of the form m/0. Given path: {}", path)) });
    }

    let mut xpriv = self.derive_impl(child_indices[0])?;
    for index in child_indices[1..].iter() {
      xpriv = xpriv.derive_impl(*index)?;
    }

    return Ok(xpriv);
  }

  fn parse_str_to_idx(x: &str) -> Result<u32, ExtendedPrivateKeyErrors> {
    let is_hardened = x.ends_with("'") || x.to_lowercase().ends_with("h");
    let index_str = x
      .trim_end_matches("'")
      .trim_end_matches("h")
      .trim_end_matches("H");

    let index = match u32::from_str_radix(index_str, 10) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!(e) })
    };

    if index >= HARDENED_KEY_OFFSET {
      return Err(ExtendedPrivateKeyErrors::DerivationError{ error: anyhow!(format!("Indicies may not be greater than {}", HARDENED_KEY_OFFSET-1)) });
    }

    Ok(match is_hardened {
      true => index + HARDENED_KEY_OFFSET,
      false => index
    })
  }
}

#[wasm_bindgen]
impl ExtendedPrivateKey {
  #[wasm_bindgen(js_name = getPrivateKey)]
  pub fn get_private_key(&self) -> PrivateKey {
    self.private_key.clone()
  }

  #[wasm_bindgen(js_name = getPublicKey)]
  pub fn get_public_key(&self) -> PublicKey {
    self.public_key.clone()
  }

  #[wasm_bindgen(js_name = getChainCode)]
  pub fn get_chain_code(&self) -> Vec<u8> {
    self.chain_code.clone()
  }

  #[wasm_bindgen(js_name = getDepth)]
  pub fn get_depth(&self) -> u8 {
    self.depth.clone()
  }

  #[wasm_bindgen(js_name = getParentFingerprint)]
  pub fn get_parent_fingerprint(&self) -> Vec<u8> {
    self.parent_fingerprint.clone()
  }

  #[wasm_bindgen(js_name = getIndex)]
  pub fn get_index(&self) -> u32 {
    self.index.clone()
  }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ExtendedPrivateKey {
  #[wasm_bindgen(js_name = deriveChild)]
  pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, JsValue> {
    match Self::derive_impl(&self, index) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = derive)]
  pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, JsValue> {
    match Self::derive_from_path_impl(&self, path) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromSeed)]
  pub fn from_seed(seed: Vec<u8>) -> Result<ExtendedPrivateKey, JsValue> {
    match Self::from_seed_impl(seed) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromRandom)]
  pub fn from_random() -> Result<ExtendedPrivateKey, JsValue> {
    match Self::from_random_impl() {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromString)]
  pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey, JsValue> {
    match Self::from_string_impl(xprv_string) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = toString)]
  pub fn to_string(&self) -> Result<String, JsValue> {
    match Self::to_string_impl(&self) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromMnemonic)]
  pub fn from_mnemonic(mnemonic: Vec<u8>, passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, JsValue> {
    match Self::from_mnemonic_and_passphrase_impl(mnemonic, passphrase) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl ExtendedPrivateKey {
  pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    Self::derive_impl(&self, index)
  }

  pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    Self::derive_from_path_impl(&self, path)
  }

  pub fn from_seed(seed: Vec<u8>) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    Self::from_seed_impl(seed)
  }

  pub fn from_random() -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    Self::from_random_impl()
  }

  pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey> {
    Self::from_string_impl(xprv_string)
  }

  pub fn to_string(&self) -> Result<String> {
    Self::to_string_impl(&self)
  }

  pub fn from_mnemonic(mnemonic: Vec<u8>, passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    Self::from_mnemonic_and_passphrase_impl(mnemonic, passphrase)
  }
}
