pub mod op_codes;
use digest::Reset;
pub use op_codes::*;

use std::{
    io::{Cursor, Read},
    str::FromStr,
    usize,
};

use crate::{
    utils::{from_hex, to_hex},
    BSVErrors,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::*;
use thiserror::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] pub(crate) Vec<u8>);

/**
 * Serialise Methods
 */
impl Script {
    pub(crate) fn to_asm_string_impl(&self, extended: bool) -> Result<String, BSVErrors> {
        let mut cursor = Cursor::new(self.0.clone());

        // Read bytes until end of string
        self.read_opcodes(&mut cursor, "", extended)
    }

    fn read_opcodes(&self, cursor: &mut Cursor<Vec<u8>>, builder_str: &str, extended: bool) -> Result<String, BSVErrors> {
        if cursor.position() >= self.0.len() as u64 {
            return Ok(builder_str.to_string());
        }
        let mut new_str = builder_str.to_string();

        if cursor.position() > 0 {
            new_str.push(' ');
        }

        let byte = cursor.read_u8()?;

        if let Some(special_opcode) = Script::get_special_opcode(byte, extended, cursor)? {
            new_str.push_str(&special_opcode);
            return Script::read_opcodes(self, cursor, &new_str, extended);
        }

        let opcode_str = match FromPrimitive::from_u8(byte) {
            Some(v @ OpCodes::OP_0) => match extended {
                true => v.to_string(),
                false => 0.to_string(),
            },
            Some(v @ OpCodes::OP_PUSHDATA1) => Script::format_pushdata_string(cursor, v, extended)?,
            Some(v @ OpCodes::OP_PUSHDATA2) => Script::format_pushdata_string(cursor, v, extended)?,
            Some(v @ OpCodes::OP_PUSHDATA4) => Script::format_pushdata_string(cursor, v, extended)?,
            Some(v) => v.to_string(),
            None => return Err(BSVErrors::SerialiseScript(format!("Unknown opcode {}", byte), None)),
        };

        new_str.push_str(&opcode_str);
        Script::read_opcodes(self, cursor, &new_str, extended)
    }

    fn get_pushdata_length(cursor: &mut Cursor<Vec<u8>>, opcode: OpCodes) -> Result<usize, BSVErrors> {
        let result = match opcode {
            OpCodes::OP_PUSHDATA1 => cursor.read_u8().map(|x| x as usize),
            OpCodes::OP_PUSHDATA2 => cursor.read_u16::<LittleEndian>().map(|x| x as usize),
            OpCodes::OP_PUSHDATA4 => cursor.read_u32::<LittleEndian>().map(|x| x as usize),
            _ => return Err(BSVErrors::SerialiseScript(format!("Given opcode {} is not pushdata", opcode), None)),
        }?;

        Ok(result)
    }

    fn get_pushdata(cursor: &mut Cursor<Vec<u8>>, size: usize) -> Result<Vec<u8>, BSVErrors> {
        let mut data_buf = vec![0; size];
        match cursor.read(&mut data_buf) {
            Ok(_) => Ok(data_buf),
            Err(e) => Err(BSVErrors::SerialiseScript(format!("Read {} OP_PUSHDATA bytes", size), Some(e))),
        }
    }

    /**
     * OpCodes such as OP_PUSH or the numerical OpCodes (OP_1-OP_16)
     */
    fn get_special_opcode(byte: u8, extended: bool, cursor: &mut Cursor<Vec<u8>>) -> Result<Option<String>, BSVErrors> {
        let code = match byte {
            size @ 0x01..=0x4b => {
                let pushdata = Script::get_pushdata(cursor, size as usize)?;

                let pushdata_hex = hex::encode(pushdata);
                match extended {
                    true => Some(format!("OP_PUSH {} {}", size, pushdata_hex)),
                    false => Some(pushdata_hex),
                }
            }

            v @ 82..=96 => OpCodes::from_u8(v).map(|num_opcode| num_opcode.to_string()),
            _ => None,
        };
        Ok(code)
    }

    fn format_pushdata_string(cursor: &mut Cursor<Vec<u8>>, v: OpCodes, extended: bool) -> Result<String, BSVErrors> {
        let size = Script::get_pushdata_length(cursor, v)?;
        let pushdata = Script::get_pushdata(cursor, size)?;

        let pushdata_hex = hex::encode(pushdata);
        Ok(match extended {
            true => format!("{} {} {}", v, size, pushdata_hex),
            false => pushdata_hex,
        })
    }
}

/**
 * Deserialise Methods
 */
impl Script {
    pub(crate) fn from_hex_impl(hex: &str) -> Result<Script, BSVErrors> {
        Ok(Script::from_bytes(&hex::decode(hex)?))
    }

    pub(crate) fn from_asm_string_impl(asm: &str) -> Result<Script, BSVErrors> {
        let chunks = asm.split(' ');
        let mut buffer: Vec<u8> = Vec::new();

        for code in chunks {
            // Number OP_CODES
            if let Ok(num_code) = u8::from_str(code) {
                match num_code {
                    v @ 0 => buffer.push(v),
                    v @ 1..=16 => buffer.push(v + 80),
                    _ => (),
                }

                continue;
            }

            // Standard OP_CODES
            if let Ok(opcode) = OpCodes::from_str(code) {
                if let Some(opcode_byte) = opcode.to_u8() {
                    buffer.push(opcode_byte);
                }
                continue;
            }

            // PUSHDATA OP_CODES
            let data_bytes = hex::decode(code)?;
            let mut op_pushdata = Script::encode_pushdata_impl(&data_bytes)?;
            buffer.append(&mut op_pushdata);
        }

        Ok(Script(buffer))
    }

    pub(crate) fn get_pushdata_prefix_bytes_impl(length: usize) -> Result<Vec<u8>, BSVErrors> {
        match length {
            op_push @ 0x01..=0x4b => {
                return Ok(vec![op_push as u8]);
            }
            op_pushdata1_size @ 0x4c..=0xFF => {
                let op_pushdata1_byte = OpCodes::OP_PUSHDATA1
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA1 Code to u8".into()))?;

                return Ok(vec![op_pushdata1_byte, op_pushdata1_size as u8]);
            }
            op_pushdata2_size @ 0x100..=0xFFFF => {
                let op_pushdata2_byte = OpCodes::OP_PUSHDATA2
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA2 Code to u8".into()))?;

                let mut push_data_prefix = vec![op_pushdata2_byte];
                push_data_prefix.write_u16::<LittleEndian>(op_pushdata2_size as u16)?;

                return Ok(push_data_prefix);
            }
            op_pushdata4_size if op_pushdata4_size > 0x10000 && op_pushdata4_size <= 0xFFFFFFFF => {
                let op_pushdata4_byte = OpCodes::OP_PUSHDATA4
                    .to_u8()
                    .ok_or_else(|| BSVErrors::DeserialiseScript("Unable to deserialise OP_PUSHDATA4 Code to u8".into()))?;

                let mut push_data_prefix = vec![op_pushdata4_byte];
                push_data_prefix.write_u32::<LittleEndian>(op_pushdata4_size as u32)?;

                return Ok(push_data_prefix);
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
        self.0.clone()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: &[u8]) -> Script {
        Script(bytes.to_vec())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getScriptLength))]
    pub fn get_script_length(&self) -> usize {
        self.0.len()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn remove_codeseparators(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|x| *x != OpCodes::OP_CODESEPARATOR.to_u8().unwrap()).collect();
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl Script {
    pub fn to_asm_string(&self) -> Result<String, BSVErrors> {
        Script::to_asm_string_impl(self, false)
    }

    pub fn to_extended_asm_string(&self) -> Result<String, BSVErrors> {
        Script::to_asm_string_impl(self, true)
    }

    pub fn from_hex(hex: &str) -> Result<Script, BSVErrors> {
        Script::from_hex_impl(hex)
    }

    pub fn from_asm_string(asm_string: &str) -> Result<Script, BSVErrors> {
        Script::from_asm_string_impl(asm_string)
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
    pub fn to_asm_string(&self) -> Result<String, JsValue> {
        match Script::to_asm_string_impl(&self, false) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toExtendedASMString))]
    pub fn to_extended_asm_string(&self) -> Result<String, JsValue> {
        match Script::to_asm_string_impl(&self, true) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromHex))]
    pub fn from_hex(hex: &str) -> Result<Script, JsValue> {
        match Script::from_hex_impl(hex) {
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
