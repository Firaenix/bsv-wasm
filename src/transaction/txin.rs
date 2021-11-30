use crate::BSVErrors;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use crate::{
    utils::{from_hex, to_hex},
    Script, VarInt,
};
use serde::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str, JsValue};

use byteorder::*;
use thiserror::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxIn {
    #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")]
    pub(crate) prev_tx_id: Vec<u8>,
    pub(crate) vout: u32,
    pub(crate) script_sig: Script,
    pub(crate) sequence: u32,

    /**
     * Part of the extended transaction serialisation format.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unlocking_script: Option<Script>,
    /**
     * Part of the extended transaction serialisation format.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) satoshis: Option<u64>,
}

impl TxIn {
    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<TxIn, BSVErrors> {
        let txin_bytes = hex::decode(hex_str)?;

        let mut cursor = Cursor::new(txin_bytes);

        TxIn::read_in(&mut cursor)
    }

    pub(crate) fn read_in(cursor: &mut Cursor<Vec<u8>>) -> Result<TxIn, BSVErrors> {
        // PrevTxId - 32 bytes
        let mut prev_tx_id = vec![0; 32];
        if let Err(e) = cursor.read(&mut prev_tx_id) {
            return Err(BSVErrors::DeserialiseTxIn("prev_tx_id".to_string(), e));
        }
        // Error in the original bitcoin client means that all txids in TxIns are reversed
        prev_tx_id.reverse();

        // VOut - 4 bytes
        let vout = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxIn("vout".to_string(), e)),
        };

        // Script Sig Size - VarInt
        let script_sig_size = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxIn("script_sig_size".to_string(), e)),
        };

        // Script Sig
        let mut script_sig = vec![0; script_sig_size as usize];
        if let Err(e) = cursor.read(&mut script_sig) {
            return Err(BSVErrors::DeserialiseTxIn("script_sig".to_string(), e));
        }

        // Sequence - 4 bytes
        let sequence = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxIn("sequence".to_string(), e)),
        };

        Ok(TxIn {
            prev_tx_id,
            vout,
            script_sig: Script(script_sig),
            sequence,
            satoshis: None,
            unlocking_script: None,
        })
    }

    pub(crate) fn to_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer = vec![];

        // Bitcoin TX Hex serialises txids in reverse.
        let mut prev_tx_id = self.prev_tx_id.clone();
        prev_tx_id.reverse();
        // Write Prev TxID first
        if let Err(e) = buffer.write(&prev_tx_id) {
            return Err(BSVErrors::SerialiseTxIn("prev_tx_id".to_string(), e));
        }

        // Vout
        if let Err(e) = buffer.write_u32::<LittleEndian>(self.vout) {
            return Err(BSVErrors::SerialiseTxIn("vout".to_string(), e));
        }

        let finalised_script = match self.unlocking_script.as_ref() {
            // If there is a specified unlocking script, prepend it to the locking script
            Some(us) => Script::from_asm_string_impl(&format!("{} {}", us.to_asm_string_impl(false)?, self.script_sig.to_asm_string_impl(false)?))?,
            None => self.script_sig.clone(),
        };

        // Script Sig Size
        if let Err(e) = buffer.write_varint(finalised_script.get_script_length() as u64) {
            return Err(BSVErrors::SerialiseTxIn("script_sig_size".to_string(), e));
        }

        // Script Sig
        if let Err(e) = buffer.write(&finalised_script.0) {
            return Err(BSVErrors::SerialiseTxIn("script_sig".to_string(), e));
        }

        // Sequence
        if let Err(e) = buffer.write_u32::<LittleEndian>(self.sequence) {
            return Err(BSVErrors::SerialiseTxIn("sequence".to_string(), e));
        }

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

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TxIn {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(prev_tx_id: &[u8], vout: u32, script_sig: &Script, sequence: Option<u32>) -> TxIn {
        TxIn {
            prev_tx_id: prev_tx_id.to_vec(),
            vout,
            script_sig: script_sig.clone(),
            sequence: match sequence {
                Some(v) => v,
                None => u32::MAX,
            },
            satoshis: None,
            unlocking_script: None,
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPrevTxId))]
    pub fn get_prev_tx_id(&self, little_endian: Option<bool>) -> Vec<u8> {
        match little_endian {
            Some(true) => {
                let mut reversed_tx = self.prev_tx_id.clone();
                reversed_tx.reverse();
                reversed_tx
            }
            _ => self.prev_tx_id.clone(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPrevTxIdHex))]
    pub fn get_prev_tx_id_hex(&self, little_endian: Option<bool>) -> String {
        hex::encode(self.get_prev_tx_id(little_endian))
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getVOut))]
    pub fn get_vout(&self) -> u32 {
        self.vout
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSigSize))]
    pub fn get_script_sig_size(&self) -> u64 {
        self.script_sig.0.len() as u64
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSig))]
    pub fn get_script_sig(&self) -> Script {
        self.script_sig.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptSigHex))]
    pub fn get_script_sig_hex(&self) -> String {
        hex::encode(self.script_sig.0.clone())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getSequence))]
    pub fn get_sequence(&self) -> u32 {
        self.sequence
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getSequenceAsBytes))]
    pub fn get_sequence_as_bytes(&self) -> Vec<u8> {
        self.sequence.to_be_bytes().to_vec()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getOutpointBytes))]
    pub fn get_outpoint_bytes(&self, little_endian: Option<bool>) -> Vec<u8> {
        let mut outpoint_bytes = self.get_prev_tx_id(little_endian);
        outpoint_bytes.extend_from_slice(&self.vout.to_le_bytes());
        outpoint_bytes
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getOutpointHex))]
    pub fn get_outpoint_hex(&self, little_endian: Option<bool>) -> String {
        hex::encode(self.get_outpoint_bytes(little_endian))
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setScript))]
    pub fn set_script(&mut self, script: &Script) {
        self.script_sig = script.clone();
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setPrevTxId))]
    pub fn set_prev_tx_id(&mut self, txid: &[u8]) {
        self.prev_tx_id = txid.to_vec();
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setVOut))]
    pub fn set_vout(&mut self, vout: u32) {
        self.vout = vout;
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setSequence))]
    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setSatoshis))]
    pub fn set_satoshis(&mut self, satoshis: u64) {
        self.satoshis = Some(satoshis);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getSatoshis))]
    pub fn get_satoshis(&self) -> Option<u64> {
        self.satoshis
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = setUnlockingScript))]
    pub fn set_unlocking_script(&mut self, unlocking_script: &Script) {
        self.unlocking_script = Some(unlocking_script.clone());
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getUnlockingScriptBytes))]
    pub fn get_unlocking_script_bytes(&self) -> Option<Vec<u8>> {
        self.unlocking_script.as_ref().map(|v| v.to_bytes())
    }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl TxIn {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex_str: &str) -> Result<TxIn, JsValue> {
        match TxIn::from_hex_impl(hex_str) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toJSON))]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        match JsValue::from_serde(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toString))]
    pub fn to_json_string(&self) -> Result<String, JsValue> {
        match TxIn::to_json_string_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Result<Vec<u8>, JsValue> {
        match TxIn::to_bytes_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> Result<String, JsValue> {
        match TxIn::to_hex_impl(&self) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl TxIn {
    pub fn from_hex(hex_str: &str) -> Result<TxIn, BSVErrors> {
        TxIn::from_hex_impl(hex_str)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        TxIn::to_bytes_impl(self)
    }

    pub fn to_hex(&self) -> Result<String, BSVErrors> {
        TxIn::to_hex_impl(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json_string(&self) -> Result<String, BSVErrors> {
        TxIn::to_json_string_impl(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn to_json(&self) -> Result<serde_json::Value, BSVErrors> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }
}
