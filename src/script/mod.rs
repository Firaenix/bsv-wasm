pub mod op_codes;
use crate::OpCodes::OP_0;
use crate::VarIntReader;
pub use op_codes::*;
use strum_macros::Display;

use crate::utils::{from_hex, to_hex};
use std::{
    io::{Cursor, Read},
    str::FromStr,
    usize,
};

use crate::{BSVErrors, VarInt};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str};

mod script_template;
pub use script_template::*;

#[derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptBit {
    OpCode(OpCodes),
    Push(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
    PushData(OpCodes, #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script(pub(crate) Vec<ScriptBit>);

/**
 * Serialise Methods
 */
impl Script {
    pub(crate) fn to_asm_string_impl(&self, extended: bool) -> String {
        self.0
            .iter()
            .map(|x| match x {
                ScriptBit::OpCode(OP_0) => match extended {
                    true => OP_0.to_string(),
                    false => 0.to_string(),
                },
                ScriptBit::Push(bytes) => match extended {
                    true => format!("OP_PUSH {} {}", bytes.len(), hex::encode(bytes)),
                    false => hex::encode(bytes),
                },
                ScriptBit::PushData(code, bytes) => match extended {
                    true => format!("{} {} {}", code, bytes.len(), hex::encode(bytes)),
                    false => hex::encode(bytes),
                },
                ScriptBit::OpCode(code) => code.to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/**
 * Deserialise Methods
 */
impl Script {
    pub(crate) fn from_hex_impl(hex: &str) -> Result<Script, BSVErrors> {
        Script::from_bytes_impl(&hex::decode(hex)?)
    }

    pub(crate) fn from_bytes_impl(bytes: &[u8]) -> Result<Script, BSVErrors> {
        let mut cursor = Cursor::new(bytes);

        let mut bit_accumulator = vec![];
        while let Ok(byte) = cursor.read_u8() {
            if byte >= 0x01 && byte <= 0x4b {
                let mut data = vec![0; byte as usize];
                if let Err(e) = cursor.read(&mut data) {
                    return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data {}", e.to_string())));
                }

                bit_accumulator.push(ScriptBit::Push(data));
                continue;
            }

            let bit = match OpCodes::from_u8(byte) {
                Some(v @ (OpCodes::OP_PUSHDATA1 | OpCodes::OP_PUSHDATA2 | OpCodes::OP_PUSHDATA4)) => {
                    let data_length = match v {
                        OpCodes::OP_PUSHDATA1 => cursor.read_u8()? as usize,
                        OpCodes::OP_PUSHDATA2 => cursor.read_u16::<LittleEndian>()? as usize,
                        _ => cursor.read_u32::<LittleEndian>()? as usize,
                    };

                    let mut data = vec![0; data_length];
                    if let Err(e) = cursor.read(&mut data) {
                        return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSHDATA data {}", e.to_string())));
                    }

                    ScriptBit::PushData(v, data)
                }
                Some(v) => ScriptBit::OpCode(v),
                None => return Err(BSVErrors::DeserialiseScript(format!("Unknown opcode {}", byte))),
            };

            bit_accumulator.push(bit);
        }

        Ok(Script(bit_accumulator))
    }

    fn map_string_to_script_bit(code: &str) -> Result<ScriptBit, BSVErrors> {
        // Number OP_CODES
        if code.len() > 0 && code.len() < 3 {
            if let Ok(num_code) = u8::from_str(code) {
                match num_code {
                    0 => return Ok(ScriptBit::OpCode(OP_0)),
                    v @ 1..=16 => return Ok(ScriptBit::OpCode(OpCodes::from_u8(v + 80).unwrap())),
                    _ => (),
                }
            }
        }

        // Standard OP_CODES
        if let Ok(opcode) = OpCodes::from_str(code) {
            return Ok(ScriptBit::OpCode(opcode));
        }

        // PUSHDATA OP_CODES
        let data_bytes = hex::decode(code)?;
        let bit = match VarInt::get_pushdata_opcode(data_bytes.len() as u64) {
            Some(v) => ScriptBit::PushData(v, data_bytes),
            None => ScriptBit::Push(data_bytes),
        };
        Ok(bit)
    }

    pub(crate) fn from_asm_string_impl(asm: &str) -> Result<Script, BSVErrors> {
        let bits: Result<Vec<_>, _> = asm.split(' ').map(Script::map_string_to_script_bit).collect();

        Ok(Script(bits?))
    }

    pub(crate) fn get_pushdata_prefix_bytes_impl(length: usize) -> Result<Vec<u8>, BSVErrors> {
        match length {
            op_push @ 0x01..=0x4b => Ok(vec![op_push as u8]),
            op_pushdata1_size @ 0x4c..=0xFF => {
                let op_pushdata1_byte = OpCodes::OP_PUSHDATA1
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA1 Code to u8".into()))?;

                Ok(vec![op_pushdata1_byte, op_pushdata1_size as u8])
            }
            op_pushdata2_size @ 0x100..=0xFFFF => {
                let op_pushdata2_byte = OpCodes::OP_PUSHDATA2
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA2 Code to u8".into()))?;

                let mut push_data_prefix = vec![op_pushdata2_byte];
                push_data_prefix.write_u16::<LittleEndian>(op_pushdata2_size as u16)?;

                Ok(push_data_prefix)
            }
            op_pushdata4_size if op_pushdata4_size > 0x10000 && op_pushdata4_size <= 0xFFFFFFFF => {
                let op_pushdata4_byte = OpCodes::OP_PUSHDATA4
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA4 Code to u8".into()))?;

                let mut push_data_prefix = vec![op_pushdata4_byte];
                push_data_prefix.write_u32::<LittleEndian>(op_pushdata4_size as u32)?;

                Ok(push_data_prefix)
            }
            size => return Err(BSVErrors::DeserialiseScript(format!("{} is too large for OP_PUSHDATAX commands", size))),
        }
    }

    pub(crate) fn encode_pushdata_impl(data_bytes: &[u8]) -> Result<Vec<u8>, BSVErrors> {
        let mut pushdata_bytes = Script::get_pushdata_prefix_bytes_impl(data_bytes.len())?;
        pushdata_bytes.append(&mut data_bytes.to_vec());

        Ok(pushdata_bytes)
    }
}

/**
 * Shared Functions
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Script {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Vec<u8> {
        let bytes = self
            .0
            .iter()
            .map(|x| match x {
                ScriptBit::OpCode(code) => vec![*code as u8],
                ScriptBit::Push(bytes) => {
                    let mut pushbytes = bytes.clone();
                    pushbytes.insert(0, bytes.len() as u8);
                    pushbytes
                }
                ScriptBit::PushData(code, bytes) => {
                    let mut pushbytes = vec![*code as u8];

                    let length_bytes = match code {
                        OpCodes::OP_PUSHDATA1 => (bytes.len() as u8).to_le_bytes().to_vec(),
                        OpCodes::OP_PUSHDATA2 => (bytes.len() as u16).to_le_bytes().to_vec(),
                        _ => (bytes.len() as u32).to_le_bytes().to_vec(),
                    };
                    pushbytes.extend(length_bytes);
                    pushbytes.extend(bytes);
                    pushbytes
                }
            })
            .flatten()
            .collect();

        bytes
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptLength))]
    pub fn get_script_length(&self) -> usize {
        self.to_bytes().len()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn remove_codeseparators(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|x| *x != ScriptBit::OpCode(OpCodes::OP_CODESEPARATOR)).collect();
    }
}

/**
 * Only export to inside Rust calling code
 */
impl Script {
    /**
     * Rust only: wasm-bindgen doesnt handle 2D arrays of u8.
     */
    pub fn from_chunks(chunks: Vec<Vec<u8>>) -> Result<Script, BSVErrors> {
        Script::from_bytes_impl(&chunks.into_iter().flatten().collect::<Vec<u8>>())
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl Script {
    pub fn to_asm_string(&self) -> String {
        Script::to_asm_string_impl(self, false)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Script, BSVErrors> {
        Script::from_bytes_impl(bytes)
    }

    pub fn to_extended_asm_string(&self) -> String {
        Script::to_asm_string_impl(self, true)
    }

    pub fn from_hex(hex: &str) -> Result<Script, BSVErrors> {
        Script::from_hex_impl(hex)
    }

    pub fn from_asm_string(asm_string: &str) -> Result<Script, BSVErrors> {
        let script = Script::from_asm_string_impl(asm_string);

        script
    }

    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, BSVErrors> {
        Script::encode_pushdata_impl(data_bytes)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, BSVErrors> {
        Script::get_pushdata_prefix_bytes_impl(length)
    }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Script {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toASMString))]
    pub fn to_asm_string(&self) -> String {
        Script::to_asm_string_impl(&self, false)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toExtendedASMString))]
    pub fn to_extended_asm_string(&self) -> String {
        Script::to_asm_string_impl(&self, true)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex: &str) -> Result<Script, JsValue> {
        match Script::from_hex_impl(hex) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: &[u8]) -> Result<Script, JsValue> {
        match Script::from_bytes_impl(bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromASMString))]
    pub fn from_asm_string(asm_string: &str) -> Result<Script, JsValue> {
        match Script::from_asm_string_impl(asm_string) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodePushData))]
    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        match Script::encode_pushdata_impl(data_bytes) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPushDataBytes))]
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, JsValue> {
        match Script::get_pushdata_prefix_bytes_impl(length) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}
