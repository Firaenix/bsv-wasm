use std::{
    io::{Cursor, Read},
    str::FromStr,
    usize,
};

use crate::utils::{from_hex, to_hex};
use anyhow::*;
use byteorder::{LittleEndian, ReadBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::*;
use snafu::*;
use wasm_bindgen::{prelude::*, throw_str};

use crate::OpCodes;

#[derive(Debug, Snafu)]
pub enum ScriptErrors {
    #[snafu(display("Error deserialising Script: {}", error))]
    Deserialise { error: anyhow::Error },

    #[snafu(display("Error serialising Script field {}: {}", reason, error))]
    Serialise {
        reason: String,
        error: anyhow::Error,
    },
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script(
    #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] pub(crate) Vec<u8>,
);

/**
 * Serialise Methods
 */
impl Script {
    fn to_asm_string_impl(&self, extended: bool) -> Result<String, ScriptErrors> {
        let mut cursor = Cursor::new(self.0.clone());

        // Read bytes until end of string
        self.read_opcodes(&mut cursor, String::new(), extended)
    }

    fn read_opcodes(
        &self,
        cursor: &mut Cursor<Vec<u8>>,
        builder_str: String,
        extended: bool,
    ) -> Result<String, ScriptErrors> {
        if cursor.position() >= self.0.len() as u64 {
            return Ok(builder_str);
        }
        let mut new_str = builder_str;

        if cursor.position() > 0 {
            new_str.push(' ');
        }

        let byte = match cursor.read_u8() {
            Ok(v) => v,
            Err(e) => {
                return Err(ScriptErrors::Serialise {
                    reason: "read_byte".to_string(),
                    error: anyhow!(e),
                })
            }
        };

        if let Some(special_opcode) = Script::get_special_opcode(byte, extended, cursor)? {
            new_str.push_str(&special_opcode);
            return Script::read_opcodes(&self, cursor, new_str, extended);
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
            None => {
                return Err(ScriptErrors::Serialise {
                    reason: byte.to_string(),
                    error: anyhow!("Unknown opcode"),
                })
            }
        };

        new_str.push_str(&opcode_str);
        Script::read_opcodes(&self, cursor, new_str, extended)
    }

    fn get_pushdata_length(
        cursor: &mut Cursor<Vec<u8>>,
        opcode: OpCodes,
    ) -> Result<usize, ScriptErrors> {
        let result = match opcode {
            OpCodes::OP_PUSHDATA1 => cursor.read_u8().map(|x| x as usize),
            OpCodes::OP_PUSHDATA2 => cursor.read_u16::<LittleEndian>().map(|x| x as usize),
            OpCodes::OP_PUSHDATA4 => cursor.read_u32::<LittleEndian>().map(|x| x as usize),
            _ => {
                return Err(ScriptErrors::Serialise {
                    reason: format!("Given opcode {} is not pushdata", opcode),
                    error: anyhow!(format!("Given opcode {} is not pushdata", opcode)),
                })
            }
        };

        result.map_err(|e| ScriptErrors::Serialise {
            reason: format!("Unable to read data length for opcode: {}", opcode),
            error: anyhow!(e),
        })
    }

    fn get_pushdata(cursor: &mut Cursor<Vec<u8>>, size: usize) -> Result<String, ScriptErrors> {
        let mut data_buf = vec![0; size];
        match cursor.read(&mut data_buf) {
            Err(e) => Err(ScriptErrors::Serialise {
                reason: format!("Read {} OP_PUSHDATA bytes", size),
                error: anyhow!(e),
            }),
            _ => Ok(hex::encode(data_buf)),
        }
    }

    /**
     * OpCodes such as OP_PUSH or the numerical OpCodes (OP_1-OP_16)
     */
    fn get_special_opcode(
        byte: u8,
        extended: bool,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<Option<String>, ScriptErrors> {
        let code = match byte {
            size @ 0x01..=0x4b => {
                let pushdata = Script::get_pushdata(cursor, size as usize)?;
                match extended {
                    true => Some(format!("OP_PUSH {} {}", size, pushdata)),
                    false => Some(pushdata),
                }
            }

            v @ 82..=96 => OpCodes::from_u8(v).map(|num_opcode| num_opcode.to_string()),
            _ => None,
        };
        Ok(code)
    }

    fn format_pushdata_string(
        cursor: &mut Cursor<Vec<u8>>,
        v: OpCodes,
        extended: bool,
    ) -> Result<String, ScriptErrors> {
        let size = Script::get_pushdata_length(cursor, v)?;
        let pushdata = Script::get_pushdata(cursor, size)?;
        Ok(match extended {
            true => format!("{} {} {}", v, size, pushdata),
            false => pushdata,
        })
    }
}

/**
 * Deserialise Methods
 */
impl Script {
    pub(crate) fn from_hex_impl(hex: String) -> Result<Script, ScriptErrors> {
        match hex::decode(hex) {
            Ok(v) => Ok(Script::from_bytes(v)),
            Err(e) => Err(ScriptErrors::Deserialise { error: anyhow!(e) }),
        }
    }

    pub(crate) fn from_asm_string_impl(asm: String) -> Result<Script, ScriptErrors> {
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
            let length = code.len() / 2;
            match length {
                op_push @ 0x01..=0x4b => {
                    buffer.push(op_push as u8);
                    match hex::decode(code) {
                        Ok(v) => buffer.append(&mut v.clone()),
                        Err(e) => return Err(ScriptErrors::Deserialise { error: anyhow!(e) }),
                    }
                }
                op_pushdata1_size @ 0x4c..=0xFF => {
                    match OpCodes::OP_PUSHDATA1.to_u8() {
                        Some(pushdata1_byte) => buffer.push(pushdata1_byte),
                        None => {
                            return Err(ScriptErrors::Deserialise {
                                error: anyhow!("Unable to deserialise OP_PUSHDATA1 Code to u8"),
                            })
                        }
                    };

                    buffer.push(op_pushdata1_size as u8);
                    match hex::decode(code) {
                        Ok(v) => buffer.append(&mut v.clone()),
                        Err(e) => return Err(ScriptErrors::Deserialise { error: anyhow!(e) }),
                    }
                }
                op_pushdata2_size @ 0x100..=0xFFFF => {
                    match OpCodes::OP_PUSHDATA2.to_u8() {
                        Some(pushdata2_byte) => buffer.push(pushdata2_byte),
                        None => {
                            return Err(ScriptErrors::Deserialise {
                                error: anyhow!("Unable to deserialise OP_PUSHDATA2 Code to u8"),
                            })
                        }
                    };

                    buffer.push(op_pushdata2_size as u8);
                    match hex::decode(code) {
                        Ok(v) => buffer.append(&mut v.clone()),
                        Err(e) => return Err(ScriptErrors::Deserialise { error: anyhow!(e) }),
                    }
                }
                size => {
                    // Cant do a standard match because 0xFFFFFFFF is too large
                    if size > 0x10000 && size <= 0xFFFFFFFF {
                        match OpCodes::OP_PUSHDATA4.to_u8() {
                            Some(pushdata4_byte) => buffer.push(pushdata4_byte),
                            None => {
                                return Err(ScriptErrors::Deserialise {
                                    error: anyhow!("Unable to deserialise OP_PUSHDATA4 Code to u8"),
                                })
                            }
                        };

                        buffer.push(size as u8);
                        match hex::decode(code) {
                            Ok(v) => buffer.append(&mut v.clone()),
                            Err(e) => return Err(ScriptErrors::Deserialise { error: anyhow!(e) }),
                        }
                    }
                }
            }
        }

        Ok(Script(buffer))
    }
}

/**
 * Shared Functions
 */
#[wasm_bindgen]
impl Script {
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: Vec<u8>) -> Script {
        Script(bytes)
    }

    #[wasm_bindgen(js_name = toHex)]
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn remove_codeseparators(&mut self) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter(|x| *x != OpCodes::OP_CODESEPARATOR.to_u8().unwrap())
            .collect();
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(target_arch = "wasm32"))]
impl Script {
    pub fn to_asm_string(&self) -> Result<String, ScriptErrors> {
        Script::to_asm_string_impl(&self, false)
    }

    pub fn to_extended_asm_string(&self) -> Result<String, ScriptErrors> {
        Script::to_asm_string_impl(&self, true)
    }

    pub fn from_hex(hex: String) -> Result<Script, ScriptErrors> {
        Script::from_hex_impl(hex)
    }

    pub fn from_asm_string(asm_string: String) -> Result<Script, ScriptErrors> {
        Script::from_asm_string_impl(asm_string)
    }
}

/**
 * WASM Specific Functions
 */
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Script {
    #[wasm_bindgen(js_name = toASMString)]
    pub fn to_asm_string(&self) -> Result<String, JsValue> {
        match Script::to_asm_string_impl(&self, false) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = toExtendedASMString)]
    pub fn to_extended_asm_string(&self) -> Result<String, JsValue> {
        match Script::to_asm_string_impl(&self, true) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = fromHex)]
    pub fn from_hex(hex: String) -> Result<Script, JsValue> {
        match Script::from_hex_impl(hex) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[wasm_bindgen(js_name = fromASMString)]
    pub fn from_asm_string(asm_string: String) -> Result<Script, JsValue> {
        match Script::from_asm_string_impl(asm_string) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}
