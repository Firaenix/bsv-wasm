pub mod op_codes;
pub use op_codes::*;

pub mod script_bit;
pub use script_bit::*;

use crate::OpCodes::OP_0;
use strum_macros::Display;

use crate::utils::{from_hex, to_hex};
use std::{
    io::{Cursor, Read},
    slice::Iter,
    str::FromStr,
    usize,
};

use crate::{BSVErrors, VarInt};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::{FromPrimitive, ToPrimitive};

use serde::{Deserialize, Serialize};

use wasm_bindgen::{prelude::*, throw_str};

mod script_template;
pub use script_template::*;

#[cfg_attr(feature = "wasm-bindgen-script", wasm_bindgen)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script(pub(crate) Vec<ScriptBit>);

/**
 * Serialise Methods
 */
impl Script {
    fn script_bits_to_asm_string(codes: &[ScriptBit], extended: bool) -> String {
        codes
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
                ScriptBit::If { code, pass, fail } => {
                    format!(
                        "{} {} {} {} {}",
                        code,
                        Script::script_bits_to_asm_string(pass, extended),
                        OpCodes::OP_ELSE,
                        Script::script_bits_to_asm_string(fail, extended),
                        OpCodes::OP_ENDIF
                    )
                }
                ScriptBit::Coinbase(bytes) => hex::encode(bytes),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn script_bits_to_bytes(codes: &[ScriptBit]) -> Vec<u8> {
        let bytes = codes
            .iter()
            .flat_map(|x| match x {
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
                ScriptBit::If { code, pass, fail } => {
                    let mut bytes = vec![*code as u8];

                    bytes.extend_from_slice(&Script::script_bits_to_bytes(pass));
                    bytes.push(OpCodes::OP_ELSE as u8);
                    bytes.extend_from_slice(&Script::script_bits_to_bytes(fail));
                    bytes.push(OpCodes::OP_ENDIF as u8);

                    bytes
                }
                ScriptBit::Coinbase(bytes) => bytes.to_vec(),
            })
            .collect();

        bytes
    }

    pub(crate) fn to_asm_string_impl(&self, extended: bool) -> String {
        Script::script_bits_to_asm_string(&self.0, extended)
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
            if (0x01..=0x4b).contains(&byte) {
                let mut data = vec![0; byte as usize];
                if let Err(e) = cursor.read(&mut data) {
                    return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data {}", e)));
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
                        return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSHDATA data {}", e)));
                    }

                    ScriptBit::PushData(v, data)
                }
                Some(v) => ScriptBit::OpCode(v),
                None => return Err(BSVErrors::DeserialiseScript(format!("Unknown opcode {}", byte))),
            };

            bit_accumulator.push(bit);
        }

        let nested_bits = Script::if_statement_pass(&bit_accumulator)?;

        Ok(Script(nested_bits))
    }

    pub(crate) fn from_coinbase_bytes_impl(bytes: &[u8]) -> Result<Script, BSVErrors> {
        Ok(Script(vec![ScriptBit::Coinbase(bytes.to_vec())]))
    }

    fn map_string_to_script_bit(code: &str) -> Result<ScriptBit, BSVErrors> {
        let code = code.trim();
        // Number OP_CODES
        match code {
            "0" => return Ok(ScriptBit::OpCode(OpCodes::OP_0)),
            "1" => return Ok(ScriptBit::OpCode(OpCodes::OP_1)),
            "2" => return Ok(ScriptBit::OpCode(OpCodes::OP_2)),
            "3" => return Ok(ScriptBit::OpCode(OpCodes::OP_3)),
            "4" => return Ok(ScriptBit::OpCode(OpCodes::OP_4)),
            "5" => return Ok(ScriptBit::OpCode(OpCodes::OP_5)),
            "6" => return Ok(ScriptBit::OpCode(OpCodes::OP_6)),
            "7" => return Ok(ScriptBit::OpCode(OpCodes::OP_7)),
            "8" => return Ok(ScriptBit::OpCode(OpCodes::OP_8)),
            "9" => return Ok(ScriptBit::OpCode(OpCodes::OP_9)),
            "10" => return Ok(ScriptBit::OpCode(OpCodes::OP_10)),
            "11" => return Ok(ScriptBit::OpCode(OpCodes::OP_11)),
            "12" => return Ok(ScriptBit::OpCode(OpCodes::OP_12)),
            "13" => return Ok(ScriptBit::OpCode(OpCodes::OP_13)),
            "14" => return Ok(ScriptBit::OpCode(OpCodes::OP_14)),
            "15" => return Ok(ScriptBit::OpCode(OpCodes::OP_15)),
            "16" => return Ok(ScriptBit::OpCode(OpCodes::OP_16)),
            _ => (),
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

    fn read_pass(bits_iter: &mut Iter<ScriptBit>) -> Result<Vec<ScriptBit>, BSVErrors> {
        let mut nested_bits = vec![];
        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => nested_bits.push(ScriptBit::If {
                    code: *v,
                    // Read until OP_ELSE
                    pass: Script::read_pass(bits_iter)?,
                    // Read until OP_ENDIF
                    fail: Script::read_fail(bits_iter)?,
                }),
                ScriptBit::OpCode(OpCodes::OP_ELSE) => return Ok(nested_bits),
                o => nested_bits.push(o.clone()),
            }
        }

        Err(BSVErrors::DeserialiseScript("OP_IF statement requires an OP_ELSE code".into()))
    }

    fn read_fail(bits_iter: &mut Iter<ScriptBit>) -> Result<Vec<ScriptBit>, BSVErrors> {
        let mut nested_bits = vec![];
        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => nested_bits.push(ScriptBit::If {
                    code: *v,
                    // Read until OP_ELSE
                    pass: Script::read_pass(bits_iter)?,
                    // Read until OP_ENDIF
                    fail: Script::read_fail(bits_iter)?,
                }),
                ScriptBit::OpCode(OpCodes::OP_ENDIF) => return Ok(nested_bits),
                o => nested_bits.push(o.clone()),
            }
        }

        Err(BSVErrors::DeserialiseScript("OP_IF statement requires an OP_ENDIF code".into()))
    }

    /// Iterates over a ScriptBit array, finds OP_XIF codes and calculates the nested ScriptBit::If block  
    /// TODO: name this function better
    fn if_statement_pass(bits: &[ScriptBit]) -> Result<Vec<ScriptBit>, BSVErrors> {
        // let mut cursor = Cursor::new(bits);

        let mut nested_bits = vec![];
        let mut bits_iter = bits.iter();
        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => nested_bits.push(ScriptBit::If {
                    code: *v,
                    // Read until OP_ELSE
                    pass: Script::read_pass(&mut bits_iter)?,
                    // Read until OP_ENDIF
                    fail: Script::read_fail(&mut bits_iter)?,
                }),
                o => nested_bits.push(o.clone()),
            }
        }

        Ok(nested_bits)
    }

    pub(crate) fn from_asm_string_impl(asm: &str) -> Result<Script, BSVErrors> {
        let bits: Result<Vec<ScriptBit>, _> = asm.split(' ').filter(|x| !(x.is_empty() || x == &"\n" || x == &"\r")).map(Script::map_string_to_script_bit).collect();
        let bits = Script::if_statement_pass(&bits?)?;

        Ok(Script(bits))
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
#[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen)]
impl Script {
    #[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen(js_name = toBytes))]
    pub fn to_bytes(&self) -> Vec<u8> {
        Script::script_bits_to_bytes(&self.0)
    }

    #[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen(js_name = getScriptLength))]
    pub fn get_script_length(&self) -> usize {
        self.to_bytes().len()
    }

    #[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen(js_name = toHex))]
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    #[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen(js_name = removeCodeSeparators))]
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

    pub fn from_script_bits(bits: Vec<ScriptBit>) -> Script {
        Script(bits)
    }

    pub fn push(&mut self, code: ScriptBit) {
        self.0.push(code);
    }

    pub fn push_array(&mut self, code: &[ScriptBit]) {
        self.0.extend_from_slice(code);
    }
}

/**
 * Native Specific Functions
 */
#[cfg(not(all(feature = "wasm-bindgen-script")))]
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

    pub fn to_script_bits(&self) -> Vec<ScriptBit> {
        self.0.clone()
    }
}

/**
 * WASM Specific Functions
 */
#[cfg(all(feature = "wasm-bindgen-script"))]
#[cfg_attr(all(feature = "wasm-bindgen-script"), wasm_bindgen)]
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
    pub fn from_hex(hex: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script::from_hex_impl(hex)?)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromBytes))]
    pub fn from_bytes(bytes: &[u8]) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script::from_bytes_impl(bytes)?)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = fromASMString))]
    pub fn from_asm_string(asm_string: &str) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script::from_asm_string_impl(asm_string)?)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = encodePushData))]
    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(Script::encode_pushdata_impl(data_bytes)?)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = getPushDataBytes))]
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
        Ok(Script::get_pushdata_prefix_bytes_impl(length)?)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = toScriptBits))]
    pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
        Ok(serde_wasm_bindgen::to_value(self)?)
    }
}
