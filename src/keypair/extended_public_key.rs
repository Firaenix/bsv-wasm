use bitcoin_hashes::hex::ToHex;
use k256::{AffinePoint, EncodedPoint, NonZeroScalar, ProjectivePoint, PublicKey as K256PublicKey, Scalar, ScalarBytes, Secp256k1, SecretKey};
use elliptic_curve::{sec1::{FromEncodedPoint, ToEncodedPoint}};

use crate::{HARDENED_KEY_OFFSET, PrivateKey, XPUB_VERSION_BYTE};
use std::{io::{Cursor, Read, Write}, ops::{Add, Mul}};

use crate::{ExtendedPrivateKey, ExtendedPublicKeyErrors, PublicKey, hash::Hash};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use wasm_bindgen::{prelude::*, throw_str};
use anyhow::*;
use getrandom::*;

#[wasm_bindgen]
pub struct ExtendedPublicKey {
  public_key: PublicKey,
  chain_code: Vec<u8>,
  depth: u8,
  index: u32,
  parent_fingerprint: Vec<u8>,
}

impl ExtendedPublicKey {
  pub fn new(
    public_key: &PublicKey,
    chain_code: &[u8],
    depth: &u8,
    index: &u32,
    parent_fingerprint: Option<&[u8]>,
  ) -> Self {
    let fingerprint = match parent_fingerprint {
      Some(v) => v,
      None => &[0, 0, 0, 0],
    };

    ExtendedPublicKey {
      public_key: public_key.clone(),
      chain_code: chain_code.to_vec(),
      depth: *depth,
      index: *index,
      parent_fingerprint: fingerprint.to_vec(),
    }
  }

  pub fn to_string_impl(&self) -> Result<String> {
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    cursor.write_u32::<BigEndian>(XPUB_VERSION_BYTE)
      .and_then(|_| cursor.write_u8(self.depth))
      .and_then(|_| cursor.write(&self.parent_fingerprint))
      .and_then(|_| cursor.write_u32::<BigEndian>(self.index))
      .and_then(|_| cursor.write(&self.chain_code))?;

    let pub_key_bytes  = match self.public_key.to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(anyhow!(e))
    };
    cursor.write(&pub_key_bytes)?;

    let mut serialised = Vec::new();
    cursor.set_position(0);
    cursor.read_to_end(&mut serialised)?;

    let checksum = &Hash::sha_256d(&serialised).to_bytes()[0..4];
    cursor.write(checksum)?;

    serialised = Vec::new();
    cursor.set_position(0);
    cursor.read_to_end(&mut serialised)?;

    Ok(bs58::encode(serialised).into_string())
  }

  pub fn from_string_impl(xpub_string: &str) -> Result<Self> {
    let mut cursor = Cursor::new(bs58::decode(xpub_string).into_vec()?);

    // Skip the first 4 bytes "xprv"
    cursor.set_position(4);

    let depth = cursor.read_u8()?;
    let mut parent_fingerprint = vec![0; 4];
    cursor.read_exact(&mut parent_fingerprint)?;
    let index = cursor.read_u32::<BigEndian>()?;

    let mut chain_code = vec![0; 32];
    cursor.read_exact(&mut chain_code)?;

    let mut pub_key_bytes = vec![0; 33];
    cursor.read_exact(&mut pub_key_bytes)?;
    let public_key = match PublicKey::from_bytes_impl(&pub_key_bytes, true) {
      Ok(v) => v,
      Err(e) => return Err(anyhow!(e)),
    };

    let mut checksum = vec![0; 4];
    cursor.read_exact(&mut checksum)?;

    Ok(ExtendedPublicKey {
      public_key,
      chain_code,
      depth,
      index,
      parent_fingerprint,
    })
  }

  pub fn from_random_impl() -> Result<Self, ExtendedPublicKeyErrors> {
    let mut seed = vec![0; 64];
    match getrandom(&mut seed)  {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPublicKeyErrors::RandomnessGenerationError{ error: anyhow!(e) }),
    };

    Self::from_seed_impl(seed)
  }

  pub fn from_xpriv(xpriv: &ExtendedPrivateKey) -> Self {
    Self {
      public_key: xpriv.get_public_key(),
      chain_code: xpriv.get_chain_code(),
      depth: xpriv.get_depth(),
      index: xpriv.get_index(),
      parent_fingerprint: xpriv.get_parent_fingerprint(),
    }
  }

  pub fn from_seed_impl(seed: Vec<u8>) -> Result<Self, ExtendedPublicKeyErrors> {
    let xpriv = match ExtendedPrivateKey::from_seed_impl(seed)  {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::InvalidSeedHmacError{ error: anyhow!(e) })
    };

    Ok(Self::from_xpriv(&xpriv))
  }

  pub fn derive_impl(&self, index: u32) -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    if index >= HARDENED_KEY_OFFSET {
      return Err(ExtendedPublicKeyErrors::DerivationError{ error: anyhow!("Cannot generate a hardened xpub, choose an index between 0 and {}.", HARDENED_KEY_OFFSET-1) })
    }

    let mut key_data: Vec<u8> = vec![];
    let pub_key_bytes = &match self.public_key.clone().to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::InvalidPublicKeyError { error: e }),
    };
    key_data.extend_from_slice(&pub_key_bytes);
    key_data.extend_from_slice(&index.clone().to_be_bytes());
    
    let pub_key_bytes = &match self.public_key.clone().to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::InvalidPublicKeyError { error: e }),
    };
    let hash = Hash::hash_160(&pub_key_bytes);
    let fingerprint = &hash.to_bytes()[0..4];

    let hmac = Hash::sha_512_hmac(&key_data, &self.chain_code.clone());
    let seed_bytes = hmac.to_bytes();

    let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    // let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    let child_public_key_bytes = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPublicKeyErrors::InvalidSeedHmacError {
        error: anyhow!("Could not get 32 bytes for private key"),
      })
    };
    let child_chain_code = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPublicKeyErrors::InvalidSeedHmacError {
        error: anyhow!("Could not get 32 bytes for chain code"),
      })
    };

    let parent_pub_key_bytes = match self.public_key.to_bytes_impl() {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::InvalidPublicKeyError { error: e }), 
    };
    let parent_pub_key_point = match K256PublicKey::from_sec1_bytes(&parent_pub_key_bytes) {
      Ok(x) => x.to_projective(),
      Err(e) => return Err(ExtendedPublicKeyErrors::PublicKeyPointError { error: anyhow!(e) })
    };

    // Pass child_public_key_bytes to secretkey because both Private + Public use same scalar, just need to multiply by it and add the new point
    let il_scalar = match SecretKey::from_bytes(child_public_key_bytes) {
      Ok(il) => Scalar::from_bytes_reduced(&il.to_secret_scalar().to_bytes()),
      Err(e) => return Err(ExtendedPublicKeyErrors::PublicKeyPointError { error: anyhow!(e) })
    };
    //ECDSA::Group::Secp256k1.generator.multiply_by_scalar(il.to_i(16))
    let child_pub_key_point = parent_pub_key_point + (ProjectivePoint::generator() * il_scalar);

    let internal_pub_key: K256PublicKey = match K256PublicKey::from_affine(child_pub_key_point.to_affine()) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::PublicKeyPointError { error: anyhow!(e) })
    };
    let child_pub_key = match PublicKey::from_bytes_impl(&internal_pub_key.to_encoded_point(true).as_bytes(), true) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::PublicKeyPointError { error: anyhow!(e) })
    };

    Ok(ExtendedPublicKey {
      chain_code: child_chain_code.to_vec(),
      public_key: child_pub_key,
      depth: self.depth + 1,
      index,
      parent_fingerprint: fingerprint.to_vec(),
    })
  }

  pub fn derive_from_path_impl(&self, path: &str) -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    if path.to_ascii_lowercase().starts_with('m') == false {
      return Err(ExtendedPublicKeyErrors::DerivationError{ error: anyhow!("Path did not begin with 'm'") });
    }

    let children = path[1..].split('/').filter(|x| -> bool { *x != "" });
    let child_indices = children.map(Self::parse_str_to_idx).collect::<Result<Vec<u32>, ExtendedPublicKeyErrors>>()?; 

    if child_indices.len() <= 0 {
      return Err(ExtendedPublicKeyErrors::DerivationError{ error: anyhow!(format!("No path was provided. Please provide a string of the form m/0. Given path: {}", path)) });
    }

    let mut xpriv = self.derive_impl(child_indices[0])?;
    for index in child_indices[1..].iter() {
      xpriv = xpriv.derive_impl(*index)?;
    }

    return Ok(xpriv);
  }

  fn parse_str_to_idx(x: &str) -> Result<u32, ExtendedPublicKeyErrors> {
    let is_hardened = x.ends_with("'") || x.to_lowercase().ends_with("h");
    let index_str = x
      .trim_end_matches("'")
      .trim_end_matches("h")
      .trim_end_matches("H");

    let index = match u32::from_str_radix(index_str, 10) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPublicKeyErrors::DerivationError{ error: anyhow!(e) })
    };

    if index >= HARDENED_KEY_OFFSET {
      return Err(ExtendedPublicKeyErrors::DerivationError{ error: anyhow!(format!("Indicies may not be greater than {}", HARDENED_KEY_OFFSET-1)) });
    }

    Ok(match is_hardened {
      true => index + HARDENED_KEY_OFFSET,
      false => index
    })
  }
}

#[wasm_bindgen]
impl ExtendedPublicKey {
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
impl ExtendedPublicKey {
  #[wasm_bindgen(js_name = deriveChild)]
  pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, JsValue> {
    match Self::derive_impl(&self, index) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = derive)]
  pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, JsValue> {
    match Self::derive_from_path_impl(&self, path) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromSeed)]
  pub fn from_seed(seed: Vec<u8>) -> Result<ExtendedPublicKey, JsValue> {
    match Self::from_seed_impl(seed) {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromRandom)]
  pub fn from_random() -> Result<ExtendedPublicKey, JsValue> {
    match Self::from_random_impl() {
      Ok(v) => Ok(v),
      Err(e) => throw_str(&e.to_string()),
    }
  }

  #[wasm_bindgen(js_name = fromString)]
  pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey, JsValue> {
    match Self::from_string_impl(xpub_string) {
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
impl ExtendedPublicKey {
  pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    Self::derive_impl(&self, index)
  }

  pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    Self::derive_from_path_impl(&self, path)
  }

  pub fn from_seed(seed: Vec<u8>) -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    Self::from_seed_impl(seed)
  }

  pub fn from_random() -> Result<ExtendedPublicKey, ExtendedPublicKeyErrors> {
    Self::from_random_impl()
  }
  pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey> {
    Self::from_string_impl(xpub_string)
  }
  pub fn to_string(&self) -> Result<String> {
    Self::to_string_impl(&self)
  }
}
