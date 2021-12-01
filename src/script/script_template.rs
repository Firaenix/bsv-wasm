use crate::{get_varint_size, varint, BSVErrors, OpCodes, PublicKey, Script, Signature, VarInt};
use byteorder::{LittleEndian, ReadBytesExt};
use core::fmt::Display;
use digest::generic_array::typenum::Len;
use num_traits::FromPrimitive;
use std::error::Error;
use std::io::{Cursor, Read};
use std::mem::size_of;
use strum_macros::Display;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum ScriptTemplateErrors {
    #[error("Length of Script did not satisfy the Script Template")]
    LengthFailure,

    #[error("Script did not match template: {0}")]
    Error(&'static str),

    #[error("Public Key at position {1} is invalid: {0}")]
    PubkeyError(BSVErrors, u64),

    #[error("Signature at position {1} is invalid: {0}")]
    SignatureError(BSVErrors, u64),

    #[error("Failed to read length {1}: {0}")]
    FailureToRead(&'static str, u64),

    #[error("Template match failure. Script[{0}]: {1:?}, Template[{2}]: {3}")]
    IndexMatchError(u64, Option<u8>, u64, u8),
}

/**
 * Script Template
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Script {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(jsName = doesMatchScriptTemplate))]
    pub fn verify_with_template(&self, script_template: &Script) -> Result<(), ScriptTemplateErrors> {
        if self.0.len() == 0 && script_template.0.len() != 0 {
            return Err(ScriptTemplateErrors::LengthFailure);
        }

        if self.0.len() != 0 && script_template.0.len() == 0 {
            return Err(ScriptTemplateErrors::LengthFailure);
        }

        let mut template_cursor = Cursor::new(script_template.0.clone());

        let mut script_position = template_cursor.position();
        let mut script_cursor: Cursor<Vec<u8>> = Cursor::new(self.0.clone());

        while let Ok(op_code) = template_cursor.read_u8() {
            let template_position = template_cursor.position();
            script_cursor.set_position(script_position);

            // Check for Pseudo opcodes (OP_PUBKEY, OP_PUBKEYHASH, OP_SIG)
            match OpCodes::from_u8(op_code) {
                Some(OpCodes::OP_DATA) => {
                    let data_length = template_cursor
                        .read_u64::<LittleEndian>()
                        .map_err(|_| ScriptTemplateErrors::Error("Could not read OP_DATA length. Must be a u64"))?;

                    script_position = script_position + data_length;
                }
                Some(OpCodes::OP_SIG | OpCodes::OP_PUBKEY | OpCodes::OP_PUBKEYHASH) => {
                    let pushdata_length = script_cursor.read_varint().map_err(|_| ScriptTemplateErrors::Error("Could not read pushdata length"))?;

                    let mut bytes = vec![0; pushdata_length as usize];
                    script_cursor
                        .read_exact(&mut bytes)
                        .map_err(|_| ScriptTemplateErrors::FailureToRead("Unable to read pushdata into buffer", pushdata_length))?;

                    if op_code == OpCodes::OP_PUBKEY as u8 {
                        if let Err(e) = PublicKey::from_bytes_impl(&bytes) {
                            return Err(ScriptTemplateErrors::PubkeyError(e, script_position));
                        }
                    }

                    if op_code == OpCodes::OP_SIG as u8 {
                        if let Err(e) = Signature::from_compact_bytes(&bytes) {
                            return Err(ScriptTemplateErrors::SignatureError(e, script_position));
                        }
                    }

                    script_position = script_position + pushdata_length;
                }
                _ if self.0.get(script_position as usize) != Some(&op_code) => {
                    let script_code = self.0.get(script_position as usize).map(|x| x.clone());

                    return Err(ScriptTemplateErrors::IndexMatchError(script_position, script_code, template_position, op_code));
                }
                _ => (),
            };

            if template_position >= script_template.0.len() as u64 {
                break;
            }

            script_position += 1
        }

        Ok(())
    }
}
