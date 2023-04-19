use crate::BSVErrors;
use crate::VarIntReader;
use crate::VarIntWriter;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;

use crate::{
    utils::{from_reverse_hex, to_reverse_hex},
    Script,
};
use serde::*;

use byteorder::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxIn {
    #[serde(serialize_with = "to_reverse_hex", deserialize_with = "from_reverse_hex")]
    pub(crate) prev_tx_id: Vec<u8>,
    pub(crate) vout: u32,
    /**
     * The script to unlock a UTXO at an outpoint.
     * AKA ScriptSig
     */
    #[serde(rename(deserialize = "script_sig", serialize = "script_sig"))]
    pub(crate) unlocking_script: Script,
    pub(crate) sequence: u32,

    /**
     * Part of the extended transaction serialisation format.
     * The representation of this TxIn's past life as a UTXO (The TxOut's
     * ScriptPubKey/LockingScript)
     */
    #[serde(rename(deserialize = "unlocking_script", serialize = "unlocking_script"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) locking_script: Option<Script>,
    /**
     * Part of the extended transaction serialisation format.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) satoshis: Option<u64>,
}

impl Default for TxIn {
    fn default() -> TxIn {
        TxIn {
            prev_tx_id: vec![],
            satoshis: None,
            unlocking_script: Script::default(),
            sequence: u32::MAX,
            locking_script: None,
            vout: 0,
        }
    }
}

impl TxIn {
    pub(crate) fn get_finalised_script_impl(&self) -> Result<Script, BSVErrors> {
        match self.locking_script.as_ref() {
            // If there is a specified unlocking script, prepend it to the locking script
            Some(locking_script) => {
                let mut unlocking_script_bytes = self.unlocking_script.to_bytes();
                let locking_script_bytes = locking_script.to_bytes();

                unlocking_script_bytes.extend_from_slice(&locking_script_bytes);
                Script::from_bytes(&unlocking_script_bytes)
            }
            None => Ok(self.unlocking_script.clone()),
        }
    }

    pub(crate) fn from_hex_impl(hex_str: &str) -> Result<TxIn, BSVErrors> {
        let txin_bytes = hex::decode(hex_str)?;

        let mut cursor = Cursor::new(txin_bytes);

        TxIn::read_in(&mut cursor)
    }

    pub(crate) fn is_coinbase_outpoint_impl(prev_tx_id: &Vec<u8>, vout: &u32) -> bool {
        prev_tx_id == &vec![0u8; 32] && vout == &0xFFFFFFFF
    }

    pub(crate) fn is_coinbase_impl(&self) -> bool {
        TxIn::is_coinbase_outpoint_impl(&self.prev_tx_id, &self.vout)
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
        let unlocking_script_size = match cursor.read_varint() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxIn("unlocking_script_size".to_string(), e)),
        };

        // Script Sig
        let mut unlocking_script = vec![0; unlocking_script_size as usize];
        if let Err(e) = cursor.read(&mut unlocking_script) {
            return Err(BSVErrors::DeserialiseTxIn("unlocking_script".to_string(), e));
        }

        // Sequence - 4 bytes
        let sequence = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => return Err(BSVErrors::DeserialiseTxIn("sequence".to_string(), e)),
        };

        Ok(TxIn {
            unlocking_script: match TxIn::is_coinbase_outpoint_impl(&prev_tx_id, &vout) {
                true => Script::from_coinbase_bytes(&unlocking_script)?,
                false => Script::from_bytes(&unlocking_script)?,
            },
            prev_tx_id,
            vout,
            sequence,
            satoshis: None,
            locking_script: None,
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

        let finalised_script = self.unlocking_script.clone();

        // Script Sig Size
        if let Err(e) = buffer.write_varint(finalised_script.get_script_length() as u64) {
            return Err(BSVErrors::SerialiseTxIn("unlocking_script_size".to_string(), e));
        }

        // Script Sig
        if let Err(e) = buffer.write(&finalised_script.to_bytes()) {
            return Err(BSVErrors::SerialiseTxIn("unlocking_script".to_string(), e));
        }

        // Sequence
        if let Err(e) = buffer.write_u32::<LittleEndian>(self.sequence) {
            return Err(BSVErrors::SerialiseTxIn("sequence".to_string(), e));
        }

        // Write out bytes
        Ok(buffer)
    }

    pub(crate) fn to_hex_impl(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(self.to_bytes_impl()?))
    }

    pub(crate) fn to_json_string_impl(&self) -> Result<String, BSVErrors> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    pub(crate) fn from_outpoint_bytes_impl(outpoint: &[u8]) -> Result<TxIn, BSVErrors> {
        if outpoint.len() != 36 {
            return Err(BSVErrors::OutOfBounds("An Outpoint must be precisely 36 bytes long".into()));
        }

        let mut tx_in = TxIn::default();

        let mut outpoint_bytes = outpoint[0..32].to_vec();
        outpoint_bytes.reverse();

        let vout = u32::from_le_bytes([outpoint[32], outpoint[33], outpoint[34], outpoint[35]]);

        tx_in.set_prev_tx_id(&outpoint_bytes);
        tx_in.set_vout(vout);

        Ok(tx_in)
    }

    /**
     * Deserialises the provided buffer to the TX+ format
     */
    pub(crate) fn from_compact_bytes_impl(compact_buffer: &[u8]) -> Result<Self, BSVErrors> {
        let tx = ciborium::de::from_reader(compact_buffer)?;
        Ok(tx)
    }

    /**
     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
     */
    pub(crate) fn to_compact_bytes_impl(&self) -> Result<Vec<u8>, BSVErrors> {
        let mut buffer = vec![];
        ciborium::ser::into_writer(&self, &mut buffer)?;
        Ok(buffer)
    }
}

/**
 * Platform Agnostic Functions
 * ie. Don't need Result<T, E>
 */
impl TxIn {
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(constructor))]
    pub fn new(prev_tx_id: &[u8], vout: u32, unlocking_script: &Script, sequence: Option<u32>) -> TxIn {
        TxIn {
            prev_tx_id: prev_tx_id.to_vec(),
            vout,
            unlocking_script: unlocking_script.clone(),
            sequence: match sequence {
                Some(v) => v,
                None => u32::MAX,
            },
            satoshis: None,
            locking_script: None,
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen)]

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getPrevTxId))]
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

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getPrevTxIdHex))]
    pub fn get_prev_tx_id_hex(&self, little_endian: Option<bool>) -> String {
        hex::encode(self.get_prev_tx_id(little_endian))
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getVOut))]
    pub fn get_vout(&self) -> u32 {
        self.vout
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getUnlockingScriptSize))]
    pub fn get_unlocking_script_size(&self) -> u64 {
        self.unlocking_script.get_script_length() as u64
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getUnlockingScript))]
    pub fn get_unlocking_script(&self) -> Script {
        self.unlocking_script.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getUnlockingScriptHex))]
    pub fn get_unlocking_script_hex(&self) -> String {
        hex::encode(self.unlocking_script.to_bytes())
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getSequence))]
    pub fn get_sequence(&self) -> u32 {
        self.sequence
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getSequenceAsBytes))]
    pub fn get_sequence_as_bytes(&self) -> Vec<u8> {
        self.sequence.to_be_bytes().to_vec()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getOutpointBytes))]
    pub fn get_outpoint_bytes(&self, little_endian: Option<bool>) -> Vec<u8> {
        let mut outpoint_bytes = self.get_prev_tx_id(little_endian);
        outpoint_bytes.extend_from_slice(&self.vout.to_le_bytes());
        outpoint_bytes
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getOutpointHex))]
    pub fn get_outpoint_hex(&self, little_endian: Option<bool>) -> String {
        hex::encode(self.get_outpoint_bytes(little_endian))
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setUnlockingScript))]
    pub fn set_unlocking_script(&mut self, script: &Script) {
        self.unlocking_script = script.clone();
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setPrevTxId))]
    pub fn set_prev_tx_id(&mut self, txid: &[u8]) {
        self.prev_tx_id = txid.to_vec();
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setVOut))]
    pub fn set_vout(&mut self, vout: u32) {
        self.vout = vout;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setSequence))]
    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setSatoshis))]
    pub fn set_satoshis(&mut self, satoshis: u64) {
        self.satoshis = Some(satoshis);
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getSatoshis))]
    pub fn get_satoshis(&self) -> Option<u64> {
        self.satoshis
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setLockingScript))]
    pub fn set_locking_script(&mut self, locking_script: &Script) {
        self.locking_script = Some(locking_script.clone());
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getLockingScript))]
    pub fn get_locking_script(&self) -> Option<Script> {
        self.locking_script.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getLockingScriptBytes))]
    pub fn get_locking_script_bytes(&self) -> Option<Vec<u8>> {
        self.locking_script.as_ref().map(|v| v.to_bytes())
    }
}

/**
 * WASM Specific Functions
 */

// #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen)]
// #[cfg(feature = "wasm-bindgen-transaction")]
// impl TxIn {
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = fromHex))]
//     pub fn from_hex(hex_str: &str) -> Result<TxIn, wasm_bindgen::JsError> {
//         Ok(TxIn::from_hex_impl(hex_str)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toJSON))]
//     pub fn to_json(&self) -> Result<JsValue, JsError> {
//         Ok(serde_wasm_bindgen::to_value(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toString))]
//     pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(TxIn::to_json_string_impl(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toBytes))]
//     pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//         Ok(TxIn::to_bytes_impl(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toHex))]
//     pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(TxIn::to_hex_impl(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = fromOutpointBytes))]
//     pub fn from_outpoint_bytes(outpoint: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
//         Ok(TxIn::from_outpoint_bytes_impl(outpoint)?)
//     }

//     /**
//      * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
//      */
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toCompactBytes))]
//     pub fn to_compact_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//         Ok(self.to_compact_bytes_impl()?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = toCompactHex))]
//     pub fn to_compact_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(hex::encode(self.to_compact_bytes_impl()?))
//     }

//     /**
//      * Deserialises the provided CBOR buffer to the TX+ format
//      */
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = fromCompactBytes))]
//     pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
//         Ok(TxIn::from_compact_bytes_impl(compact_buffer)?)
//     }

//     /**
//      * Deserialises the provided CBOR buffer to the TX+ format
//      */
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = fromCompactHex))]
//     pub fn from_compact_hex(compact_hex: String) -> Result<TxIn, wasm_bindgen::JsError> {
//         let compact_buffer = hex::decode(compact_hex)?;

//         Ok(TxIn::from_compact_bytes_impl(&compact_buffer)?)
//     }

//     /// Concatenates ScriptSig and UnlockingScript into a single script.
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = getFinalisedScript))]
//     pub fn get_finalised_script(&self) -> Result<Script, JsError> {
//         Ok(self.get_finalised_script_impl()?)
//     }

//     // Checks if input is a coinbase
//     #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = isCoinbase))]
//     pub fn is_coinbase(&self) -> bool {
//         self.is_coinbase_impl()
//     }
// }

/**
 * Native Specific Functions
 */
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

    /**
     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + XT
     */
    pub fn to_compact_bytes(&self) -> Result<Vec<u8>, BSVErrors> {
        self.to_compact_bytes_impl()
    }

    /**
     * Deserialises the provided CBOR buffer to the XT format
     */
    pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<Self, BSVErrors> {
        TxIn::from_compact_bytes_impl(compact_buffer)
    }

    /**
     * Serialises this entire transaction to CBOR Hex, preserving all fields from the standard Transaction format + XT
     */
    pub fn to_compact_hex(&self) -> Result<String, BSVErrors> {
        Ok(hex::encode(self.to_compact_bytes_impl()?))
    }

    /**
     * Deserialises the provided CBOR hex to the XT format
     */
    pub fn from_compact_hex(compact_hex: &str) -> Result<Self, BSVErrors> {
        let compact_buffer = hex::decode(compact_hex)?;
        TxIn::from_compact_bytes_impl(&compact_buffer)
    }

    pub fn to_json_string(&self) -> Result<String, BSVErrors> {
        TxIn::to_json_string_impl(self)
    }

    pub fn to_json(&self) -> Result<serde_json::Value, BSVErrors> {
        let json = serde_json::to_value(self)?;
        Ok(json)
    }

    pub fn from_outpoint_bytes(outpoint: &[u8]) -> Result<TxIn, BSVErrors> {
        TxIn::from_outpoint_bytes_impl(outpoint)
    }

    pub fn get_finalised_script(&self) -> Result<Script, BSVErrors> {
        self.get_finalised_script_impl()
    }

    // Checks if input is a coinbase
    pub fn is_coinbase(&self) -> bool {
        self.is_coinbase_impl()
    }
}
