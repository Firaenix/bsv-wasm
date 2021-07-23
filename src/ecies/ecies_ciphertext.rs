use crate::PublicKey;
use wasm_bindgen::{prelude::*, throw_str};

use crate::BSVErrors;

#[wasm_bindgen]
pub struct ECIESCiphertext {
    pub(crate) public_key_bytes: Option<Vec<u8>>,
    pub(crate) ciphertext_bytes: Vec<u8>,
    pub(crate) hmac_bytes: Vec<u8>,
}

const PUB_KEY_OFFSET: u8 = 4;
const PUB_KEY_END: u8 = PUB_KEY_OFFSET + 33;

impl ECIESCiphertext {
    pub fn extract_public_key_impl(&self) -> Result<PublicKey, BSVErrors> {
        let bytes = self.public_key_bytes.clone().ok_or_else(|| BSVErrors::ECIESError("No public key exists in this ciphertext".into()))?;
        PublicKey::from_bytes_impl(&bytes)
    }
}

#[wasm_bindgen]
impl ECIESCiphertext {
    #[wasm_bindgen(js_name = extractCiphertext)]
    pub fn extract_ciphertext(&self) -> Vec<u8> {
        self.ciphertext_bytes.clone()
    }

    #[wasm_bindgen(js_name = extractHMAC)]
    pub fn extract_hmac(&self) -> Vec<u8> {
        self.hmac_bytes.clone()
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

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(&self, buffer: Vec<u8>, has_pub_key: bool) -> ECIESCiphertext {
        let pub_key = match has_pub_key {
            true => Some(buffer[PUB_KEY_OFFSET as usize..PUB_KEY_END as usize].to_vec()),
            false => None,
        };

        let hmac = &buffer[buffer.len() - 32..buffer.len()];
        let ciphertext = match has_pub_key {
            true => &buffer[PUB_KEY_END as usize..buffer.len() - 32],
            false => &buffer[4..buffer.len() - 32],
        };

        ECIESCiphertext {
            public_key_bytes: pub_key,
            hmac_bytes: hmac.into(),
            ciphertext_bytes: ciphertext.into(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ECIESCiphertext {
    #[wasm_bindgen(js_name = extractPublicKey)]
    pub fn extract_public_key(&self) -> Result<PublicKey, JsValue> {
        match self.extract_public_key_impl() {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECIESCiphertext {
    pub fn extract_public_key(&self) -> Result<PublicKey, BSVErrors> {
        self.extract_public_key_impl()
    }
}
