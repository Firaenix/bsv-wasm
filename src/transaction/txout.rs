use crate::{BSVErrors, Script, VarIntReader, VarIntWriter};
use serde::*;
use std::io::Read;
use std::io::{Cursor, Write};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsError};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::throw_str;

use byteorder::*;

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOut {
    pub(crate) value: u64,
    pub(crate) script_pub_key: Script,
}

impl TxOut {
    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<TxOut, BSVErrors> {
        let txout_bytes = hex::decode(hex_str)?;

        let mut cursor = Cursor::new(txout_bytes);

        TxOut::read_in(&mut cursor)
    }

    pub fn read_in(cursor: &mut Cursor<Vec<u8>>) -> Result<TxOut, BSVErrors> {
        // Satoshi Value - 8 bytes
        let satoshis = match cursor.read_u64::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxOut("satoshis".to_string(), e)),
        };

        // Script Pub Key Size - 1-9 bytes
        let script_pub_key_size = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxOut("script_pub_key_size".to_string(), e)),
        };

        // Script Pub Key
        let mut script_pub_key = vec![0; script_pub_key_size as usize];
        if let Err(e) = cursor.read(&mut script_pub_key) {
            return Err(BSVErrors::DeserialiseTxOut("script_pub_key".to_string(), e));
        }

        Ok(TxOut {
            value: satoshis,
            script_pub_key: Script::from_bytes_impl(&script_pub_key)?,
        })
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer = Vec::new();

        // Satoshi Value - 8 bytes
        buffer.write_u64::<LittleEndian>(self.value)?;

        // Script Pub Key Size - 1-9 bytes
        buffer.write_varint(self.get_script_pub_key_size() as u64)?;

        // Script Pub Key
        buffer.write_all(&self.script_pub_key.to_bytes())?;

        // Write out bytes
        Ok(buffer)
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(&self.to_bytes_impl()?))
    }

    pub(crate) fn to_json_string_impl(&self) -> Result<String, BSVErrors> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen)]
impl TxOut {
    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(constructor))]
    pub fn new(value: u64, script_pub_key: &Script) -> TxOut {
        TxOut {
            value,
            script_pub_key: script_pub_key.clone(),
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getSatoshis))]
    pub fn get_satoshis(&self) -> u64 {
        self.value
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getSatoshisAsBytes))]
    pub fn get_satoshis_as_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getScriptPubKeySize))]
    pub fn get_script_pub_key_size(&self) -> usize {
        self.script_pub_key.get_script_length()
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getScriptPubKey))]
    pub fn get_script_pub_key(&self) -> Script {
        self.script_pub_key.clone()
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getScriptPubKeyHex))]
    pub fn get_script_pub_key_hex(&self) -> String {
        self.script_pub_key.to_hex()
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen)]
impl TxOut {
    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: &str) -> Result<TxOut, JsValue> {
        match TxOut::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match TxOut::to_bytes_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        match TxOut::to_hex_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toJSON))]
    pub fn to_json(&self) -> Result<JsValue, JsError> {
        Ok(serde_wasm_bindgen::to_value(&self)?)
    }

    #[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toString))]
    pub fn to_json_string(&self) -> Result<String, JsValue> {
        match TxOut::to_json_string_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl TxOut {
    pub fn from_hex(hex_str: &str) -> Result<TxOut, BSVErrors> {
        TxOut::from_hex_impl(hex_str)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        TxOut::to_bytes_impl(self)
    }

    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        TxOut::to_hex_impl(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json(&self) -> Result<serde_json::Value, BSVErrors> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json_string(&self) -> Result<String, BSVErrors> {
        TxOut::to_json_string_impl(self)
    }
}
