use std::{io::{Cursor, Read, Write}, ops::{Add, Rem}, vec};

use bitcoin_hashes::hex::ToHex;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use elliptic_curve::generic_array::typenum::private::IsGreaterOrEqualPrivate;
use k256::{NonZeroScalar, Secp256k1, ecdsa::SigningKey, Scalar, SecretKey};
use primitive_types::{U256, U512};

use anyhow::*;
use getrandom::*;
use snafu::*;
use wasm_bindgen::{prelude::*, throw_str};

use crate::{hash::Hash, PrivateKey, PrivateKeyErrors, PublicKey, PublicKeyErrors};

pub const HARDENED_KEY_OFFSET: u32 = 0x80000000;

#[derive(Debug, Snafu)]
pub enum ExtendedPrivateKeyErrors {
  #[snafu(display("Could not generate randomness: {}", error))]
  RandomnessGenerationError { error: anyhow::Error },
  #[snafu(display("Could not calculate private key bytes from seed: {}", error))]
  InvalidSeedHmacError { error: anyhow::Error },
  #[snafu(display("Could not calculate private key: {}", error))]
  InvalidPrivateKeyError { error: PrivateKeyErrors },
  #[snafu(display("Could not calculate public key: {}", error))]
  InvalidPublicKeyError { error: PublicKeyErrors },
  #[snafu(display("Could not serialise xpriv: {}", error))]
  SerialisationError { error: anyhow::Error },

  #[snafu(display("Could not derive xpriv: {}", error))]
  DerivationError { error: anyhow::Error },
}

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
      public_key: PublicKey::from_private_key(private_key, true),
      chain_code: chain_code.to_vec(),
      depth: *depth,
      index: *index,
      parent_fingerprint: fingerprint.to_vec(),
    }
  }

  pub fn to_string_impl(&self) -> Result<String, ExtendedPrivateKeyErrors> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    match cursor.write_u32::<BigEndian>(0x0488ade4)
      .and_then(|_| cursor.write_u8(self.depth))
      .and_then(|_| cursor.write(&self.parent_fingerprint))
      .and_then(|_| cursor.write_u32::<BigEndian>(self.index))
      .and_then(|_| cursor.write(&self.chain_code))
      .and_then(|_| cursor.write_u8(0))
      .and_then(|_| cursor.write(&self.private_key.to_bytes())) {
        Ok(_) => (),
        Err(e) => return Err(ExtendedPrivateKeyErrors::SerialisationError{ error: anyhow!(e) })
      };

    let mut serialised = Vec::new();
    cursor.set_position(0);
    match cursor.read_to_end(&mut serialised) {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPrivateKeyErrors::SerialisationError{ error: anyhow!(e) })
    };

    let checksum = &Hash::sha_256d(&serialised).to_bytes()[0..4];
    match cursor.write(checksum) {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPrivateKeyErrors::SerialisationError{ error: anyhow!(e) })
    };

    serialised = Vec::new();
    cursor.set_position(0);
    match cursor.read_to_end(&mut serialised) {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPrivateKeyErrors::SerialisationError{ error: anyhow!(e) })
    };

    Ok(bs58::encode(serialised).into_string())
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
    let public_key = PublicKey::from_private_key(&private_key, true);

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

    let pub_key = PublicKey::from_private_key(&priv_key, true);

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

    let parent_private_key = SecretKey::from_bytes(self.private_key.clone().to_bytes().as_slice()).unwrap();

    let il = SecretKey::from_bytes(private_key_bytes).unwrap();
    let sclal: Scalar = Scalar::from_bytes_reduced(&il.secret_scalar().to_bytes());

    // child_private_key = il + parent_key % n
    let derived_private_key = parent_private_key.secret_scalar().add(sclal);

    let child_private_key = match PrivateKey::from_bytes_impl(&derived_private_key.to_bytes()) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPrivateKeyError { error: e }),
    };

    let child_chain_code_bytes = child_chain_code.to_vec();
    let child_pub_key = PublicKey::from_private_key(&child_private_key, true);

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
  pub fn get_private_key(&self) -> PrivateKey {
    self.private_key.clone()
  }

  pub fn get_public_key(&self) -> PublicKey {
    self.public_key.clone()
  }

  pub fn get_chain_code(&self) -> Vec<u8> {
    self.chain_code.clone()
  }

  pub fn get_depth(&self) -> u8 {
    self.depth.clone()
  }

  pub fn get_parent_fingerprint(&self) -> Vec<u8> {
    self.parent_fingerprint.clone()
  }

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
  pub fn to_string(&self) -> Result<String, ExtendedPrivateKeyErrors> {
    Self::to_string_impl(&self)
  }
}
