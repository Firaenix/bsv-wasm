use std::io::Read;
use std::io::{Cursor, Write};

use crate::{
    utils::{from_hex, to_hex},
    VarInt,
};
use serde::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

use anyhow::*;
use byteorder::*;
use snafu::*;

#[derive(Debug, Snafu)]
pub enum TxOutErrors {
    #[snafu(display("Error deserialising TxOut field {:?}: {}", field, error))]
    Deserialise {
        field: Option<String>,
        error: anyhow::Error,
    },

    #[snafu(display("Error serialising TxOut field {:?}: {}", field, error))]
    Serialise {
        field: Option<String>,
        error: anyhow::Error,
    },
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TxOut {
    value: u64,
    #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
    script_pub_key: Vec<u8>,
}

impl TxOut {
    pub(crate) fn from_hex_impl(hex_str: String) -> Result<TxOut, TxOutErrors> {
        let txout_bytes = match hex::decode(&hex_str) {
            Ok(v) => v,
            Err(e) => {
                return Err(TxOutErrors::Deserialise {
                    field: None,
                    error: anyhow!(e),
                })
            }
        };

        let mut cursor = Cursor::new(txout_bytes);

        TxOut::read_in(&mut cursor)
    }

    pub fn read_in(cursor: &mut Cursor<Vec<u8>>) -> Result<TxOut, TxOutErrors> {
        // Satoshi Value - 8 bytes
        let satoshis = match cursor.read_u64::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                return Err(TxOutErrors::Deserialise {
                    field: Some("satoshis".to_string()),
                    error: anyhow!(e),
                })
            }
        };

        // Script Pub Key Size - 1-9 bytes
        let script_pub_key_size = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => {
                return Err(TxOutErrors::Deserialise {
                    field: Some("script_pub_key_size".to_string()),
                    error: anyhow!(e),
                })
            }
        };

        // Script Pub Key
        let mut script_pub_key = vec![0; script_pub_key_size as usize];
        if let Err(e) = cursor.read(&mut script_pub_key) {
            return Err(TxOutErrors::Deserialise {
                field: Some("script_pub_key".to_string()),
                error: anyhow!(e),
            });
        };

        Ok(TxOut {
            value: satoshis,
            script_pub_key,
        })
    }

    pub(crate) fn to_bytes_impl(&self) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::new();

        // Satoshi Value - 8 bytes
        buffer.write_u64::<LittleEndian>(self.value)?;

        // Script Pub Key Size - 1-9 bytes
        buffer.write_varint(self.get_script_pub_key_size())?;

        // Script Pub Key
        buffer.write_all(&self.script_pub_key)?;

        // Write out bytes
        Ok(buffer)
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String> {
        Ok(hex::encode(&self.to_bytes_impl()?))
    }

    pub(crate) fn to_json_string_impl(&self) -> Result<String, TxOutErrors> {
        match serde_json::to_string_pretty(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(TxOutErrors::Serialise {
                field: None,
                error: anyhow!(e),
            }),
        }
    }
}

#[wasm_bindgen]
impl TxOut {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u64, script_pub_key: Vec<u8>) -> TxOut {
        TxOut {
            value,
            script_pub_key,
        }
    }

    #[wasm_bindgen(js_name = getSatoshis)]
    pub fn get_satoshis(&self) -> u64 {
        self.value
    }

    #[wasm_bindgen(js_name = getSatoshisAsBytes)]
    pub fn get_satoshis_as_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }

    #[wasm_bindgen(js_name = getScriptPubKeySize)]
    pub fn get_script_pub_key_size(&self) -> u64 {
        self.script_pub_key.len() as u64
    }

    #[wasm_bindgen(js_name = getScriptPubKey)]
    pub fn get_script_pub_key(&self) -> Vec<u8> {
        self.script_pub_key.clone()
    }

    #[wasm_bindgen(js_name = getScriptPubKeyHex)]
    pub fn get_script_pub_key_hex(&self) -> String {
        hex::encode(self.script_pub_key.clone())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TxOut {
    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex_str: String) -> Result<TxOut, JsValue> {
        match TxOut::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match TxOut::to_bytes_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        match TxOut::to_hex_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        match JsValue::from_serde(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_json_string(&self) -> Result<String, JsValue> {
        match TxOut::to_json_string_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl TxOut {
    pub fn from_hex(hex_str: String) -> Result<TxOut, TxOutErrors> {
        TxOut::from_hex_impl(hex_str)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        TxOut::to_bytes_impl(&self).map_err(|e| anyhow!(e))
    }

    pub fn to_hex(&self) -> Result<String> {
        TxOut::to_hex_impl(&self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json(&self) -> Result<serde_json::Value, TxOutErrors> {
        match serde_json::to_value(self) {
            Ok(v) => Ok(v),
            Err(e) => Err(TxOutErrors::Serialise {
                field: None,
                error: anyhow!(e),
            }),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json_string(&self) -> Result<String, TxOutErrors> {
        TxOut::to_json_string_impl(&self)
    }
}
