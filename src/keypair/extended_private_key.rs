use crate::{BSVErrors, HARDENED_KEY_OFFSET, KDF, XPRIV_VERSION_BYTE};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use getrandom::*;
use k256::SecretKey;
use std::{
    io::{Cursor, Read, Write},
    ops::Add,
    vec,
};

use crate::{hash::Hash, PrivateKey, PublicKey};

pub struct ExtendedPrivateKey {
    private_key: PrivateKey,
    public_key: PublicKey,
    chain_code: Vec<u8>,
    depth: u8,
    index: u32,
    parent_fingerprint: Vec<u8>,
}

impl ExtendedPrivateKey {
    pub fn new(private_key: &PrivateKey, chain_code: &[u8], depth: &u8, index: &u32, parent_fingerprint: Option<&[u8]>) -> Self {
        let fingerprint = parent_fingerprint.unwrap_or(&[0, 0, 0, 0]);

        ExtendedPrivateKey {
            private_key: private_key.clone(),
            public_key: PublicKey::from_private_key_impl(private_key),
            chain_code: chain_code.to_vec(),
            depth: *depth,
            index: *index,
            parent_fingerprint: fingerprint.to_vec(),
        }
    }

    pub fn to_string_impl(&self) -> Result<String, BSVErrors> {
        let mut buffer: Vec<u8> = vec![];

        buffer
            .write_u32::<BigEndian>(XPRIV_VERSION_BYTE)
            .and_then(|_| buffer.write_u8(self.depth))
            .and_then(|_| buffer.write(&self.parent_fingerprint))
            .and_then(|_| buffer.write_u32::<BigEndian>(self.index))
            .and_then(|_| buffer.write(&self.chain_code))
            .and_then(|_| buffer.write_u8(0))
            .and_then(|_| buffer.write(&self.private_key.to_bytes()))?;

        let checksum = &Hash::sha_256d(&buffer).to_bytes()[0..4];
        buffer.write_all(checksum)?;

        Ok(bs58::encode(buffer).into_string())
    }

    pub fn from_mnemonic_and_passphrase_impl(mnemonic: &[u8], passphrase: Option<Vec<u8>>) -> Result<Self, BSVErrors> {
        let fixed_phrase = match passphrase {
            Some(v) => v,
            None => b"mnemonic".to_vec(),
        };

        let seed = KDF::pbkdf2(mnemonic, Some(fixed_phrase.to_vec()), crate::PBKDF2Hashes::SHA512, 2048, 64);
        let seed_bytes = seed.get_hash().to_bytes();
        Self::from_seed_impl(&seed_bytes)
    }

    pub fn from_string_impl(xprv_string: &str) -> Result<Self, BSVErrors> {
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
        let private_key = PrivateKey::from_bytes_impl(&private_key_bytes)?;
        let public_key = PublicKey::from_private_key_impl(&private_key);

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

    pub fn from_random_impl() -> Result<Self, BSVErrors> {
        let mut seed = vec![0; 64];
        getrandom(&mut seed)?;

        Self::from_seed_impl(&seed)
    }

    pub fn from_seed_impl(seed: &[u8]) -> Result<Self, BSVErrors> {
        let seed_hmac = Hash::sha_512_hmac(seed, b"Bitcoin seed");

        let seed_bytes = seed_hmac.to_bytes();
        let mut seed_chunks = seed_bytes.chunks_exact(32_usize);
        let private_key_bytes = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for private key".into())),
        };
        let chain_code = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for chain code".into())),
        };

        let priv_key = PrivateKey::from_bytes_impl(private_key_bytes)?;

        let pub_key = PublicKey::from_private_key_impl(&priv_key);

        Ok(Self {
            private_key: priv_key,
            public_key: pub_key,
            chain_code: chain_code.to_vec(),
            depth: 0,
            index: 0,
            parent_fingerprint: [0, 0, 0, 0].to_vec(),
        })
    }

    pub fn derive_impl(&self, index: u32) -> Result<ExtendedPrivateKey, BSVErrors> {
        let is_hardened = index >= HARDENED_KEY_OFFSET;

        let key_data = match is_hardened {
            true => {
                let mut bytes: Vec<u8> = vec![0x0];
                bytes.extend_from_slice(&self.private_key.clone().to_bytes());
                bytes.extend_from_slice(&index.to_be_bytes());
                bytes
            }
            false => {
                let mut bytes: Vec<u8> = vec![];

                let pub_key_bytes = &self.public_key.clone().to_bytes_impl()?;

                bytes.extend_from_slice(pub_key_bytes);
                bytes.extend_from_slice(&index.to_be_bytes());
                bytes
            }
        };

        let pub_key_bytes = &self.public_key.clone().to_bytes_impl()?;
        let hash = Hash::hash_160(pub_key_bytes);
        let fingerprint = &hash.to_bytes()[0..4];

        let hmac = Hash::sha_512_hmac(&key_data, &self.chain_code.clone());
        let seed_bytes = hmac.to_bytes();

        let mut seed_chunks = seed_bytes.chunks_exact(32_usize);
        // let mut seed_chunks = seed_bytes.chunks_exact(32 as usize);
        let private_key_bytes = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for private key".into())),
        };
        let child_chain_code = match seed_chunks.next() {
            Some(b) => b,
            None => return Err(BSVErrors::InvalidSeedHmacError("Could not get 32 bytes for chain code".into())),
        };

        let parent_scalar = SecretKey::from_be_bytes(&self.private_key.to_bytes())?.to_nonzero_scalar();

        let il_scalar = *SecretKey::from_be_bytes(private_key_bytes)?.to_nonzero_scalar();

        // child_private_key = il + parent_key % n
        let derived_private_key = parent_scalar.add(il_scalar);

        let child_private_key = PrivateKey::from_bytes_impl(&derived_private_key.to_bytes())?;

        let child_chain_code_bytes = child_chain_code.to_vec();
        let child_pub_key = PublicKey::from_private_key_impl(&child_private_key);

        Ok(ExtendedPrivateKey {
            chain_code: child_chain_code_bytes,
            private_key: child_private_key,
            public_key: child_pub_key,
            depth: self.depth + 1,
            index,
            parent_fingerprint: fingerprint.to_vec(),
        })
    }

    pub fn derive_from_path_impl(&self, path: &str) -> Result<ExtendedPrivateKey, BSVErrors> {
        #[allow(clippy::bool_comparison)]
        if path.to_ascii_lowercase().starts_with('m') == false {
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

        let index = index_str.parse::<u32>()?;

        if index >= HARDENED_KEY_OFFSET {
            return Err(BSVErrors::DerivationError(format!("Indicies may not be greater than {}", HARDENED_KEY_OFFSET - 1)));
        }

        Ok(match is_hardened {
            true => index + HARDENED_KEY_OFFSET,
            false => index,
        })
    }
}

impl ExtendedPrivateKey {
    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPrivateKey))]
    pub fn get_private_key(&self) -> PrivateKey {
        self.private_key.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPublicKey))]
    pub fn get_public_key(&self) -> PublicKey {
        self.public_key.clone()
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
// impl ExtendedPrivateKey {
//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = deriveChild))]
//     pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::derive_impl(&self, index)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = derive))]
//     pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::derive_from_path_impl(&self, path)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromSeed))]
//     pub fn from_seed(seed: &[u8]) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::from_seed_impl(seed)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromRandom))]
//     pub fn from_random() -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::from_random_impl()?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromString))]
//     pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::from_string_impl(xprv_string)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = toString))]
//     pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(Self::to_string_impl(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = fromMnemonic))]
//     pub fn from_mnemonic(mnemonic: &[u8], passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//         Ok(Self::from_mnemonic_and_passphrase_impl(mnemonic, passphrase)?)
//     }
// }

impl ExtendedPrivateKey {
    pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::derive_impl(self, index)
    }

    pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::derive_from_path_impl(self, path)
    }

    pub fn from_seed(seed: &[u8]) -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::from_seed_impl(seed)
    }

    pub fn from_random() -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::from_random_impl()
    }

    pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::from_string_impl(xprv_string)
    }

    pub fn to_string(&self) -> Result<String, BSVErrors> {
        Self::to_string_impl(self)
    }

    pub fn from_mnemonic(mnemonic: &[u8], passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, BSVErrors> {
        Self::from_mnemonic_and_passphrase_impl(mnemonic, passphrase)
    }
}
