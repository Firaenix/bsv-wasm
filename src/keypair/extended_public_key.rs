use elliptic_curve::sec1::ToEncodedPoint;
use k256::{ProjectivePoint, PublicKey as K256PublicKey, SecretKey};

use crate::{HARDENED_KEY_OFFSET, XPUB_VERSION_BYTE};
use std::io::{Cursor, Read, Write};

use crate::{hash::Hash, BSVErrors, ExtendedPrivateKey, PublicKey};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use getrandom::*;


pub struct ExtendedPublicKey {
    public_key: PublicKey,
    chain_code: Vec<u8>,
    depth: u8,
    index: u32,
    parent_fingerprint: Vec<u8>,
}

impl ExtendedPublicKey {
    pub fn new(public_key: &PublicKey, chain_code: &[u8], depth: &u8, index: &u32, parent_fingerprint: Option<&[u8]>) -> Self {
        let fingerprint = parent_fingerprint.unwrap_or(&[0, 0, 0, 0]);

        ExtendedPublicKey {
            public_key: public_key.clone(),
            chain_code: chain_code.to_vec(),
            depth: *depth,
            index: *index,
            parent_fingerprint: fingerprint.to_vec(),
        }
    }

    pub fn to_string_impl(&self) -> Result<String, BSVErrors> {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        cursor
            .write_u32::<BigEndian>(XPUB_VERSION_BYTE)
            .and_then(|_| cursor.write_u8(self.depth))
            .and_then(|_| cursor.write(&self.parent_fingerprint))
            .and_then(|_| cursor.write_u32::<BigEndian>(self.index))
            .and_then(|_| cursor.write(&self.chain_code))?;

        let pub_key_bytes = self.public_key.to_bytes_impl()?;
        cursor.write_all(&pub_key_bytes)?;

        let mut serialised = Vec::new();
        cursor.set_position(0);
        cursor.read_to_end(&mut serialised)?;

        let checksum = &Hash::sha_256d(&serialised).to_bytes()[0..4];
        cursor.write_all(checksum)?;

        serialised = Vec::new();
        cursor.set_position(0);
        cursor.read_to_end(&mut serialised)?;

        Ok(bs58::encode(serialised).into_string())
    }

    pub fn from_string_impl(xpub_string: &str) -> Result<Self, BSVErrors> {
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
        let public_key = PublicKey::from_bytes_impl(&pub_key_bytes)?;

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

    pub fn from_random_impl() -> Result<Self, BSVErrors> {
        let mut seed = vec![0; 64];
        getrandom(&mut seed)?;

        Self::from_seed_impl(&seed)
    }

    pub fn from_seed_impl(seed: &[u8]) -> Result<Self, BSVErrors> {
        let xpriv = ExtendedPrivateKey::from_seed_impl(seed)?;

        Ok(Self::from_xpriv(&xpriv))
    }

    pub fn derive_impl(&self, index: u32) -> Result<ExtendedPublicKey, BSVErrors> {
        if index >= HARDENED_KEY_OFFSET {
            return Err(BSVErrors::DerivationError(format!(
                "Cannot generate a hardened xpub, choose an index between 0 and {}.",
                HARDENED_KEY_OFFSET - 1
            )));
        }

        let mut key_data: Vec<u8> = vec![];
        let pub_key_bytes = &self.public_key.clone().to_bytes_impl()?;
        key_data.extend_from_slice(pub_key_bytes);
        key_data.extend_from_slice(&index.to_be_bytes());

        let pub_key_bytes = &self.public_key.clone().to_bytes_impl()?;
        let hash = Hash::hash_160(pub_key_bytes);
        let fingerprint = &hash.to_bytes()[0..4];

        let hmac = Hash::sha_512_hmac(&key_data, &self.chain_code.clone());
        let seed_bytes = hmac.to_bytes();

        let mut seed_chunks = seed_bytes.chunks_exact(32_usize);
        // let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
        let child_public_key_bytes = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for private key".into())),
        };
        let child_chain_code = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for chain code".into())),
        };

        let parent_pub_key_bytes = self.public_key.to_bytes_impl()?;
        let parent_pub_key_point = K256PublicKey::from_sec1_bytes(&parent_pub_key_bytes)?.to_projective();

        // Pass child_public_key_bytes to secretkey because both Private + Public use same scalar, just need to multiply by it and add the new point
        let il_scalar = *SecretKey::from_be_bytes(child_public_key_bytes)?.to_nonzero_scalar();
        let child_pub_key_point = parent_pub_key_point + (ProjectivePoint::GENERATOR * il_scalar);

        let internal_pub_key: K256PublicKey = K256PublicKey::from_affine(child_pub_key_point.to_affine())?;
        let child_pub_key = PublicKey::from_bytes_impl(internal_pub_key.to_encoded_point(true).as_bytes())?;

        Ok(ExtendedPublicKey {
            chain_code: child_chain_code.to_vec(),
            public_key: child_pub_key,
            depth: self.depth + 1,
            index,
            parent_fingerprint: fingerprint.to_vec(),
        })
    }

    pub fn derive_from_path_impl(&self, path: &str) -> Result<ExtendedPublicKey, BSVErrors> {
        if !path.to_ascii_lowercase().starts_with('m') {
            return Err(BSVErrors::DerivationError("Path did not begin with 'm'".into()));
        }

        let children = path[1..].split('/').filter(|x| -> bool { !x.is_empty() });
        let child_indices = children.map(Self::parse_str_to_idx).collect::<Result<Vec<u32>, BSVErrors>>()?;

        if child_indices.is_empty() {
            return Err(BSVErrors::DerivationError(format!(
                "No path was provided. Please provide a string of the form m/0. Given path: {}",
                path
            )));
        }

        let mut xpriv = self.derive_impl(child_indices[0])?;
        for index in child_indices[1..].iter() {
            xpriv = xpriv.derive_impl(*index)?;
        }

        Ok(xpriv)
    }

    fn parse_str_to_idx(x: &str) -> Result<u32, BSVErrors> {
        let is_hardened = x.ends_with('\'') || x.to_lowercase().ends_with('h');
        let index_str = x.trim_end_matches('\'').trim_end_matches('h').trim_end_matches('H');

        let index = match index_str.parse::<u32>() {
            Ok(v) => v,
            // TODO: Make this error handling nicer
            Err(e) => return Err(BSVErrors::DerivationError(e.to_string())),
        };

        if index >= HARDENED_KEY_OFFSET {
            return Err(BSVErrors::DerivationError(format!("Indicies may not be greater than {}", HARDENED_KEY_OFFSET - 1)));
        }

        Ok(match is_hardened {
            true => index + HARDENED_KEY_OFFSET,
            false => index,
        })
    }
}

impl ExtendedPublicKey {
    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPublicKey))]
    pub fn get_public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromXPriv))]
    pub fn from_xpriv(xpriv: &ExtendedPrivateKey) -> Self {
        Self {
            public_key: xpriv.get_public_key(),
            chain_code: xpriv.get_chain_code(),
            depth: xpriv.get_depth(),
            index: xpriv.get_index(),
            parent_fingerprint: xpriv.get_parent_fingerprint(),
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getChainCode))]
    pub fn get_chain_code(&self) -> Vec<u8> {
        self.chain_code.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getDepth))]
    pub fn get_depth(&self) -> u8 {
        self.depth
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getParentFingerprint))]
    pub fn get_parent_fingerprint(&self) -> Vec<u8> {
        self.parent_fingerprint.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getIndex))]
    pub fn get_index(&self) -> u32 {
        self.index
    }
}


// #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen)]
// #[cfg(feature = "wasm-bindgen-keypair")]
// impl ExtendedPublicKey {
//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = deriveChild))]
//     pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//         Ok(Self::derive_impl(&self, index)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = derive))]
//     pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//         Ok(Self::derive_from_path_impl(&self, path)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromSeed))]
//     pub fn from_seed(seed: &[u8]) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//         Ok(Self::from_seed_impl(seed)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromRandom))]
//     pub fn from_random() -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//         Ok(Self::from_random_impl()?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromString))]
//     pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//         Ok(Self::from_string_impl(xpub_string)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toString))]
//     pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(Self::to_string_impl(&self)?)
//     }
// }

impl ExtendedPublicKey {
    pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, BSVErrors> {
        Self::derive_impl(self, index)
    }

    pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, BSVErrors> {
        Self::derive_from_path_impl(self, path)
    }

    pub fn from_seed(seed: &[u8]) -> Result<ExtendedPublicKey, BSVErrors> {
        Self::from_seed_impl(seed)
    }

    pub fn from_random() -> Result<ExtendedPublicKey, BSVErrors> {
        Self::from_random_impl()
    }
    pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey, BSVErrors> {
        Self::from_string_impl(xpub_string)
    }
    pub fn to_string(&self) -> Result<String, BSVErrors> {
        Self::to_string_impl(self)
    }
}
