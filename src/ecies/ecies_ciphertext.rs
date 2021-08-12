use crate::{CipherKeys, PublicKey};
use wasm_bindgen::{prelude::*, throw_str};

use crate::BSVErrors;

#[wasm_bindgen]
pub struct ECIESCiphertext {
    pub(crate) public_key_bytes: Option<Vec<u8>>,
    pub(crate) ciphertext_bytes: Vec<u8>,
    pub(crate) hmac_bytes: Vec<u8>,
    pub(crate) keys: Option<CipherKeys>,
}

const PUB_KEY_OFFSET: u8 = 4;
const PUB_KEY_END: u8 = PUB_KEY_OFFSET + 33;

impl ECIESCiphertext {
    pub(crate) fn extract_public_key_impl(&self) -> Result<PublicKey, BSVErrors> {
        let bytes = self.public_key_bytes.clone().ok_or_else(|| BSVErrors::ECIESError("No public key exists in this ciphertext".into()))?;
        PublicKey::from_bytes_impl(&bytes)
    }

    pub(crate) fn from_bytes_impl(buffer: &[u8], has_pub_key: bool) -> Result<ECIESCiphertext, BSVErrors> {
        let pub_key = match has_pub_key {
            true => {
                let pub_key_buf = &buffer[PUB_KEY_OFFSET as usize..PUB_KEY_END as usize];
                PublicKey::from_bytes_impl(pub_key_buf)?;
                Some(pub_key_buf.to_vec())
            }
            false => None,
        };

        let hmac = &buffer[buffer.len() - 32..buffer.len()];
        let ciphertext = match has_pub_key {
            true => &buffer[PUB_KEY_END as usize..buffer.len() - 32],
            false => &buffer[4..buffer.len() - 32],
        };

        Ok(ECIESCiphertext {
            public_key_bytes: pub_key,
            hmac_bytes: hmac.into(),
            ciphertext_bytes: ciphertext.into(),
            keys: None,
        })
    }
}

#[wasm_bindgen]
impl ECIESCiphertext {
    #[wasm_bindgen(js_name = getCiphertext)]
    pub fn get_ciphertext(&self) -> Vec<u8> {
        self.ciphertext_bytes.clone()
    }

    #[wasm_bindgen(js_name = getHMAC)]
    pub fn get_hmac(&self) -> Vec<u8> {
        self.hmac_bytes.clone()
    }

    #[wasm_bindgen(js_name = getCipherKeys)]
    pub fn get_cipher_keys(&self) -> Option<CipherKeys> {
        self.keys.clone()
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend_from_slice(b"BIE1");
        if let Some(bytes) = self.public_key_bytes.clone() {
            buffer.extend_from_slice(&bytes);
        }
        buffer.extend_from_slice(&self.ciphertext_bytes);
        buffer.extend_from_slice(&self.hmac_bytes);

        buffer
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECIESCiphertext {
    #[wasm_bindgen(js_name = extractPublicKey)]
    pub fn extract_public_key(&self) -> Result<PublicKey, JsValue> {
        self.extract_public_key_impl().map_err(|e| throw_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(buffer: &[u8], has_pub_key: bool) -> Result<ECIESCiphertext, JsValue> {
        ECIESCiphertext::from_bytes_impl(buffer, has_pub_key).map_err(|e| throw_str(&e.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECIESCiphertext {
    pub fn extract_public_key(&self) -> Result<PublicKey, BSVErrors> {
        self.extract_public_key_impl()
    }

    pub fn from_bytes(buffer: &[u8], has_pub_key: bool) -> Result<ECIESCiphertext, BSVErrors> {
        ECIESCiphertext::from_bytes_impl(buffer, has_pub_key)
    }
}
