use wasm_bindgen::prelude::*;
use getrandom::*;
use snafu::*;
use anyhow::*;

use crate::{PrivateKey, PrivateKeyErrors, PublicKey, PublicKeyErrors, hash::Hash};

#[derive(Debug, Snafu)]
pub enum ExtendedPrivateKeyErrors {
    #[snafu(display("Could not generate randomness: {}", error))]
    RandomnessGenerationError {
      error: anyhow::Error
    },
    #[snafu(display("Could not calculate private key bytes from seed: {}", error))]
    InvalidSeedHmacError {
      error: anyhow::Error
    },
    #[snafu(display("Could not calculate private key: {}", error))]
    InvalidPrivateKeyError {
      error: PrivateKeyErrors
    },
    #[snafu(display("Could not calculate public key: {}", error))]
    InvalidPublicKeyError {
      error: PublicKeyErrors
    }
}


#[wasm_bindgen]
pub struct ExtendedPrivateKey {
  private_key: PrivateKey,
  public_key: PublicKey,
  chain_code: Vec<u8>,
}

impl ExtendedPrivateKey {
  pub fn from_random() -> Result<Self, ExtendedPrivateKeyErrors> {
    let mut seed = vec![0; 64];
    match getrandom(&mut seed) {
      Ok(_) => (),
      Err(e) => return Err(ExtendedPrivateKeyErrors::RandomnessGenerationError{error: anyhow!(e)})
    }

    Self::from_seed(seed)
  }

  pub fn from_seed(seed: Vec<u8>) -> Result<Self, ExtendedPrivateKeyErrors> {
    let seed_hmac = Hash::sha_512_hmac(&seed, b"Bitcoin seed");

    let seed_bytes = seed_hmac.to_bytes();
    let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
    let private_key_bytes = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError{ error: anyhow!("Could not get 32 bytes for private key") })
    };
    let chain_code = match seed_chunks.next() {
      Some(b) => b,
      None => return Err(ExtendedPrivateKeyErrors::InvalidSeedHmacError{ error: anyhow!("Could not get 32 bytes for chain code") })
    };


    let priv_key = match PrivateKey::from_bytes_impl(private_key_bytes) {
      Ok(v) => v,
      Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPrivateKeyError{ error: e })
    };

    let pub_key = PublicKey::from_private_key(&priv_key, true);

    Ok(Self{
      private_key: priv_key.clone(),
      public_key: pub_key.clone(),
      chain_code: chain_code.to_vec()
    })
  }

  pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, ExtendedPrivateKeyErrors> {
    let is_hardened = match index {
      v @ 0..=0x7FFFFFFF => false,
      _ => true
    };

    let key_data = match is_hardened {
      true => {
        let mut append_index = index.clone().to_be_bytes().to_vec();
        let concat_bytes = self.private_key.clone().to_bytes();
        concat_bytes.append(&mut append_index);
        concat_bytes
      },
      false => {
        let mut append_index = index.clone().to_be_bytes().to_vec();
        let concat_bytes = match self.public_key.clone().to_bytes() {
          Ok(v) => v,
          Err(e) => return Err(ExtendedPrivateKeyErrors::InvalidPublicKeyError{ error:e })
        };
        concat_bytes.append(&mut append_index);
        concat_bytes
      }
    };

    let hmac = Hash::sha_512_hmac(&key_data, &self.chain_code.clone());

    k256::elliptic_curve::group::Group
  }
}