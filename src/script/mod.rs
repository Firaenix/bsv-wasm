use crate::{Hash, OpCodes::OP_0};

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

pub mod op_codes;
pub use op_codes::*;

pub mod script_bit;
pub use script_bit::*;

mod script_template;
pub use script_template::*;

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
                ScriptBit::NonScriptData(bytes) => format!("non-script-data:{}", hex::encode(bytes)),
                ScriptBit::OpCode(code) => code.to_string(),
                ScriptBit::If { code, pass, fail } => {
                    let mut string_parts = vec![];

                    string_parts.push(code.to_string());

                    let pass_string = Script::script_bits_to_asm_string(pass, extended);
                    if !pass_string.is_empty() {
                        string_parts.push(pass_string);
                    }

                    if let Some(fail) = fail {
                        string_parts.push(OpCodes::OP_ELSE.to_string());
                        let fail_string = Script::script_bits_to_asm_string(fail, extended);
                        if !fail_string.is_empty() {
                            string_parts.push(fail_string);
                        }
                    }

                    string_parts.push(OpCodes::OP_ENDIF.to_string());

                    string_parts.join(" ")
                }
                ScriptBit::Coinbase(bytes) => hex::encode(bytes),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn script_bits_to_bytes(codes: &[ScriptBit]) -> Vec<u8> {
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
                ScriptBit::NonScriptData(bytes) => {
                    let mut pushbytes = vec![];
                    pushbytes.extend(bytes);
                    pushbytes
                }
                ScriptBit::If { code, pass, fail } => {
                    let mut bytes = vec![*code as u8];

                    bytes.extend_from_slice(&Script::script_bits_to_bytes(pass));

                    if let Some(fail) = fail {
                        bytes.push(OpCodes::OP_ELSE as u8);
                        bytes.extend_from_slice(&Script::script_bits_to_bytes(fail));
                    }
                    bytes.push(OpCodes::OP_ENDIF as u8);

                    bytes
                }
                ScriptBit::Coinbase(bytes) => bytes.to_vec(),
            })
            .collect();

        bytes
    }

    fn check_script_bits(codes: &[ScriptBit]) -> () {
        if codes.len() == 0 {
            return;
        }
        let mut is_non_script_data = false;
        for (i, scriptbit) in codes.iter().enumerate() {
            match scriptbit {
                ScriptBit::OpCode(OpCodes::OP_RETURN) => {
                    is_non_script_data = true;
                }
                ScriptBit::NonScriptData(_) => {
                    if is_non_script_data != true {
                        panic!("NonScriptData can only appear after OP_RETURN");
                    }

                    if i != codes.len() - 1 {
                        panic!("NonScriptData can only appear at the end of the script!");
                    }
                }
                _ => (),
            }
        }
    }

    pub fn to_asm_string_impl(&self, extended: bool) -> String {
        Script::script_bits_to_asm_string(&self.0, extended)
    }

    pub fn from_hex(hex: &str) -> Result<Script, BSVErrors> {
        Script::from_bytes(&hex::decode(hex)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Script, BSVErrors> {
        let mut cursor = Cursor::new(bytes);

        let mut bit_accumulator = vec![];
        let mut scope_level = 0;
        while let Ok(byte) = cursor.read_u8() {
            if byte.eq(&(OpCodes::OP_IF as u8)) || byte.eq(&(OpCodes::OP_NOTIF as u8)) {
                scope_level += 1;
            } else if byte.eq(&(OpCodes::OP_ENDIF as u8)) {
                scope_level -= 1;
            } else if byte.eq(&(OpCodes::OP_RETURN as u8)) && scope_level == 0 {
                bit_accumulator.push(ScriptBit::OpCode(OpCodes::OP_RETURN));

                let len = cursor.get_ref().len();

                let non_script_data_length = len - cursor.position() as usize;

                if non_script_data_length > 0 {
                    let mut data: Vec<u8> = vec![0; non_script_data_length as usize];

                    match cursor.read(&mut data) {
                        Ok(_) => {
                            bit_accumulator.push(ScriptBit::NonScriptData(data));
                        }
                        Err(e) => return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data {}", e))),
                    }
                }

                break;
            }

            if byte.ne(&(OpCodes::OP_0 as u8)) && byte.lt(&(OpCodes::OP_PUSHDATA1 as u8)) {
                let mut data: Vec<u8> = vec![0; byte as usize];
                match cursor.read(&mut data) {
                    Ok(len) => {
                        if len == byte as usize {
                            bit_accumulator.push(ScriptBit::Push(data));
                        } else {
                            return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data")));
                        }
                    }
                    Err(e) => return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data {}", e))),
                }
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

                    match cursor.read(&mut data) {
                        Ok(len) => {
                            if len == data_length as usize {
                                ScriptBit::PushData(v, data)
                            } else {
                                return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data")));
                            }
                        }
                        Err(e) => return Err(BSVErrors::DeserialiseScript(format!("Failed to read OP_PUSH data {}", e))),
                    }
                }
                Some(v) => ScriptBit::OpCode(v),
                None => return Err(BSVErrors::DeserialiseScript(format!("Unknown opcode {}", byte))),
            };

            bit_accumulator.push(bit);
        }

        let nested_bits = Script::if_statement_pass(&mut bit_accumulator.iter())?;

        Ok(Script(nested_bits))
    }

    pub fn from_coinbase_bytes(bytes: &[u8]) -> Result<Script, BSVErrors> {
        Ok(Script(vec![ScriptBit::Coinbase(bytes.to_vec())]))
    }

    fn map_string_to_script_bit(code: &str, is_non_script_data: bool) -> Result<ScriptBit, BSVErrors> {
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

        if code.starts_with("non-script-data:") {
            if is_non_script_data {
                let non_script_data = hex::decode(code.trim_start_matches("non-script-data:"))?;
                return Ok(ScriptBit::NonScriptData(non_script_data));
            } else {
                return Err(BSVErrors::InvalidNonScriptData());
            }
        }

        // PUSHDATA OP_CODES
        let data_bytes = hex::decode(code)?;
        let bit = match VarInt::get_pushdata_opcode(data_bytes.len() as u64) {
            Some(v) => ScriptBit::PushData(v, data_bytes),
            None => ScriptBit::Push(data_bytes),
        };
        Ok(bit)
    }

    fn read_pass(bits_iter: &mut Iter<ScriptBit>) -> Result<(Vec<ScriptBit>, bool), BSVErrors> {
        let mut nested_bits = vec![];
        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => Script::read_if_statement(bits_iter, &mut nested_bits, v)?,
                ScriptBit::OpCode(OpCodes::OP_ELSE) => return Ok((nested_bits, false)),
                ScriptBit::OpCode(OpCodes::OP_ENDIF) => return Ok((nested_bits, true)),
                o => nested_bits.push(o.clone()),
            }
        }

        Err(BSVErrors::DeserialiseScript("OP_IF branch requires an OP_ELSE or OP_ENDIF code".into()))
    }

    fn read_fail(bits_iter: &mut Iter<ScriptBit>) -> Result<Vec<ScriptBit>, BSVErrors> {
        let mut nested_bits = vec![];
        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => Script::read_if_statement(bits_iter, &mut nested_bits, v)?,
                ScriptBit::OpCode(OpCodes::OP_ENDIF) => return Ok(nested_bits),
                o => nested_bits.push(o.clone()),
            }
        }

        Err(BSVErrors::DeserialiseScript("OP_ELSE branch requires an OP_ENDIF code".into()))
    }

    fn read_if_statement(bits_iter: &mut Iter<ScriptBit>, nested_bits: &mut Vec<ScriptBit>, v: &OpCodes) -> Result<(), BSVErrors> {
        let (pass_bits, ended) = Script::read_pass(bits_iter)?;
        nested_bits.push(ScriptBit::If {
            code: *v,
            // Read until OP_ELSE or OP_ENDIF
            pass: pass_bits,
            // Read until OP_ENDIF
            fail: match ended {
                true => None,
                false => Some(Script::read_fail(bits_iter)?),
            },
        });
        Ok(())
    }

    /// Iterates over a ScriptBit array, finds OP_XIF codes and calculates the nested ScriptBit::If block  
    fn if_statement_pass(bits_iter: &mut Iter<ScriptBit>) -> Result<Vec<ScriptBit>, BSVErrors> {
        let mut nested_bits = vec![];

        while let Some(thing) = bits_iter.next() {
            match thing {
                ScriptBit::OpCode(v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => Script::read_if_statement(bits_iter, &mut nested_bits, v)?,
                o => nested_bits.push(o.clone()),
            }
        }

        Ok(nested_bits)
    }

    /**
     * Ordinary ASM, (for example, OP_RETURN 01 01) does not contain ScriptBit::NonScriptData after being converted into ScriptBit.
     * This function wraps all ScriptBit after OP_RETURN with ScriptBit::NonScriptData.
     */
    fn wrap_with_non_script_data(bits_iter: &mut Iter<ScriptBit>, non_script_data_index: usize) -> Vec<ScriptBit> {
        let mut bits = vec![];
        let mut non_script_data_bits = vec![];
        let mut index: usize = 0;
        while let Some(thing) = bits_iter.next() {
            if index >= non_script_data_index {
                match thing {
                    ScriptBit::NonScriptData(b) => bits.push(ScriptBit::NonScriptData(b.to_vec())),
                    o => non_script_data_bits.push(o.clone()),
                }
            } else {
                bits.push(thing.clone())
            }
            index += 1;
        }

        if non_script_data_bits.len() > 0 {
            bits.push(ScriptBit::NonScriptData(Script::script_bits_to_bytes(&non_script_data_bits)))
        }

        bits
    }

    pub fn from_asm_string(asm: &str) -> Result<Script, BSVErrors> {
        let mut scope_level = 0;

        let mut is_non_script_data = false;

        let mut non_script_data_index: usize = usize::MAX;

        let bits: Result<Vec<ScriptBit>, _> = asm
            .split(' ')
            .filter(|x| !(x.is_empty() || x == &"\n" || x == &"\r"))
            .enumerate()
            .map(|(i, x)| match Script::map_string_to_script_bit(x, is_non_script_data) {
                Ok(bit) => {
                    match bit {
                        ScriptBit::OpCode(_v @ (OpCodes::OP_IF | OpCodes::OP_NOTIF | OpCodes::OP_VERIF | OpCodes::OP_VERNOTIF)) => {
                            scope_level += 1;
                        }
                        ScriptBit::OpCode(OpCodes::OP_ENDIF) => {
                            scope_level -= 1;
                        }
                        ScriptBit::OpCode(OpCodes::OP_RETURN) => {
                            if scope_level == 0 {
                                is_non_script_data = true;
                                non_script_data_index = i + 1;
                            }
                        }
                        _ => (),
                    }
                    Ok(bit)
                }
                Err(e) => Err(e),
            })
            .collect();

        if non_script_data_index != usize::MAX {
            let bits = Script::wrap_with_non_script_data(&mut bits?.iter(), non_script_data_index);

            let bits = Script::if_statement_pass(&mut bits.iter())?;

            return Ok(Script(bits));
        } else {
            let bits = Script::if_statement_pass(&mut bits?.iter())?;

            return Ok(Script(bits));
        }
    }

    pub fn get_pushdata_prefix_bytes(length: usize) -> Result<Vec<u8>, BSVErrors> {
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
            size => Err(BSVErrors::DeserialiseScript(format!("{} is too large for OP_PUSHDATAX commands", size))),
        }
    }

    pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, BSVErrors> {
        let mut pushdata_bytes = Script::get_pushdata_prefix_bytes(data_bytes.len())?;
        pushdata_bytes.append(&mut data_bytes.to_vec());

        Ok(pushdata_bytes)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        Script::script_bits_to_bytes(&self.0)
    }

    pub fn get_script_length(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    pub fn remove_codeseparators(&mut self) {
        self.0 = self.0.clone().into_iter().filter(|x| *x != ScriptBit::OpCode(OpCodes::OP_CODESEPARATOR)).collect();
    }

    pub fn from_chunks(chunks: Vec<Vec<u8>>) -> Result<Script, BSVErrors> {
        Script::from_bytes(&chunks.into_iter().flatten().collect::<Vec<u8>>())
    }

    pub fn from_script_bits(bits: Vec<ScriptBit>) -> Script {
        Script::check_script_bits(&bits);
        Script(bits)
    }

    pub fn push(&mut self, code: ScriptBit) {
        self.0.push(code);
    }

    pub fn push_array(&mut self, code: &[ScriptBit]) {
        self.0.extend_from_slice(code);
    }

    pub fn to_scripthash_hex(&self) -> String {
        hex::encode(self.to_scripthash_bytes())
    }

    pub fn to_scripthash_bytes(&self) -> Vec<u8> {
        let mut scripthash = Hash::sha_256(&self.to_bytes()).to_bytes();
        scripthash.reverse();
        scripthash
    }

    pub fn to_asm_string(&self) -> String {
        Script::to_asm_string_impl(self, false)
    }

    pub fn to_extended_asm_string(&self) -> String {
        Script::to_asm_string_impl(self, true)
    }

    /**
     * Gets the OP_PUSHDATA prefix varint
     */
    pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, BSVErrors> {
        Script::get_pushdata_prefix_bytes(length)
    }

    pub fn to_script_bits(&self) -> Vec<ScriptBit> {
        self.0.clone()
    }
}
