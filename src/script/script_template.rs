use crate::OpCodes::OP_0;
use std::str::FromStr;

use crate::{BSVErrors, OpCodes, PublicKey, Script, ScriptBit, Signature, VarInt};
use hex::FromHexError;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, throw_str, JsValue};

#[derive(Debug, Error)]
pub enum ScriptTemplateErrors {
    #[error("Script did not match template at index {0}. {2} is not equal to {1:?}")]
    MatchFailure(usize, MatchToken, ScriptBit),

    #[error("Failed to parse OP_DATA code {0}: {1}")]
    OpDataParse(String, String),

    #[error("Script is empty but template is not.")]
    EmptyScriptDoesntMatch,

    #[error("{0}")]
    MalformedHex(
        #[from]
        #[source]
        FromHexError,
    ),
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"), wasm_bindgen)]
#[derive(Debug, Clone, Display)]
pub enum DataLengthConstraints {
    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
}

#[derive(Debug, Clone, Display)]
pub enum MatchToken {
    // Precise Matches
    OpCode(OpCodes),
    Push(Vec<u8>),
    PushData(OpCodes, Vec<u8>),

    // Fuzzy matches
    AnyData,
    Data(usize, DataLengthConstraints),
    Signature,
    PublicKey,
    PublicKeyHash,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"), wasm_bindgen)]
#[derive(Debug, Clone, Display, Serialize, Deserialize)]
pub enum MatchDataTypes {
    Data,
    Signature,
    PublicKey,
    PublicKeyHash,
}

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[derive(Debug, Clone, Serialize, Deserialize)]
pub type Match = (MatchDataTypes, Vec<u8>);

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"), wasm_bindgen)]
#[derive(Debug, Clone)]
pub struct ScriptTemplate(Vec<MatchToken>);

impl ScriptTemplate {
    fn map_string_to_match_token(code: &str) -> Result<MatchToken, ScriptTemplateErrors> {
        // Number OP_CODES
        if let Ok(num_code) = u8::from_str(code) {
            match num_code {
                0 => return Ok(MatchToken::OpCode(OP_0)),
                v @ 1..=16 => return Ok(MatchToken::OpCode(OpCodes::from_u8(v + 80).unwrap())),
                _ => (),
            }
        }

        // Standard OP_CODES
        match OpCodes::from_str(code) {
            Ok(OpCodes::OP_SIG) => return Ok(MatchToken::Signature),
            Ok(OpCodes::OP_PUBKEY) => return Ok(MatchToken::PublicKey),
            Ok(OpCodes::OP_PUBKEYHASH) => return Ok(MatchToken::PublicKeyHash),
            Ok(OpCodes::OP_DATA) => return Ok(MatchToken::AnyData),

            Ok(v) => return Ok(MatchToken::OpCode(v)),
            Err(_) => (),
        }

        if code.starts_with(&OpCodes::OP_DATA.to_string()) {
            // Match on >=
            if let Some((_, length_str)) = code.split_once(">=") {
                let len = usize::from_str(length_str).map_err(|e| ScriptTemplateErrors::OpDataParse(code.to_string(), e.to_string()))?;
                return Ok(MatchToken::Data(len, DataLengthConstraints::GreaterThanOrEquals));
            }

            // Match on <=
            if let Some((_, length_str)) = code.split_once("<=") {
                let len = usize::from_str(length_str).map_err(|e| ScriptTemplateErrors::OpDataParse(code.to_string(), e.to_string()))?;
                return Ok(MatchToken::Data(len, DataLengthConstraints::LessThanOrEquals));
            }

            // Match on =
            if let Some((_, length_str)) = code.split_once("=") {
                let len = usize::from_str(length_str).map_err(|e| ScriptTemplateErrors::OpDataParse(code.to_string(), e.to_string()))?;
                return Ok(MatchToken::Data(len, DataLengthConstraints::Equals));
            }

            // Match on >
            if let Some((_, length_str)) = code.split_once(">") {
                let len = usize::from_str(length_str).map_err(|e| ScriptTemplateErrors::OpDataParse(code.to_string(), e.to_string()))?;
                return Ok(MatchToken::Data(len, DataLengthConstraints::GreaterThan));
            }

            // Match on <
            if let Some((_, length_str)) = code.split_once("<") {
                let len = usize::from_str(length_str).map_err(|e| ScriptTemplateErrors::OpDataParse(code.to_string(), e.to_string()))?;
                return Ok(MatchToken::Data(len, DataLengthConstraints::LessThan));
            }
        }

        // PUSHDATA OP_CODES
        let data_bytes = hex::decode(code)?;
        let token = match VarInt::get_pushdata_opcode(data_bytes.len() as u64) {
            Some(v) => MatchToken::PushData(v, data_bytes),
            None => MatchToken::Push(data_bytes),
        };

        Ok(token)
    }

    pub fn from_script_impl(script: &Script) -> Result<ScriptTemplate, ScriptTemplateErrors> {
        ScriptTemplate::from_asm_string_impl(&script.to_asm_string_impl(false))
    }

    pub fn from_asm_string_impl(asm: &str) -> Result<ScriptTemplate, ScriptTemplateErrors> {
        let tokens: Result<Vec<_>, _> = asm.split(' ').map(ScriptTemplate::map_string_to_match_token).collect();

        Ok(ScriptTemplate(tokens?))
    }
}

#[cfg(not(feature = "wasm-bindgen-script-template"))]
impl ScriptTemplate {
    pub fn from_script(script: &Script) -> Result<ScriptTemplate, ScriptTemplateErrors> {
        ScriptTemplate::from_script_impl(script)
    }

    pub fn from_asm_string(asm: &str) -> Result<ScriptTemplate, ScriptTemplateErrors> {
        ScriptTemplate::from_asm_string_impl(asm)
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"), wasm_bindgen)]
impl ScriptTemplate {
    pub fn from_script(script: &Script) -> Result<ScriptTemplate, JsValue> {
        match ScriptTemplate::from_script_impl(script) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    pub fn from_asm_string(asm: &str) -> Result<ScriptTemplate, JsValue> {
        match ScriptTemplate::from_asm_string_impl(asm) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

/**
 * Script Template
 */
impl Script {
    pub fn match_impl(&self, script_template: &ScriptTemplate) -> Result<Vec<Match>, ScriptTemplateErrors> {
        if self.0.is_empty() && !script_template.0.is_empty() {
            return Err(ScriptTemplateErrors::EmptyScriptDoesntMatch);
        }

        let mut matches = vec![];

        for (i, (template, script)) in script_template.0.iter().zip(self.0.iter()).enumerate() {
            let is_match = match (template, script) {
                (MatchToken::OpCode(tmpl_code), ScriptBit::OpCode(op_code)) => tmpl_code == op_code,
                (MatchToken::Push(tmpl_data), ScriptBit::Push(data)) => tmpl_data == data,
                (MatchToken::PushData(tmpl_op, tmpl_data), ScriptBit::PushData(op, data)) => tmpl_op == op && tmpl_data == data,

                (MatchToken::Data(len, constraint), ScriptBit::PushData(_, data) | ScriptBit::Push(data)) => match constraint {
                    DataLengthConstraints::Equals => &data.len() == len,
                    DataLengthConstraints::GreaterThan => &data.len() > len,
                    DataLengthConstraints::LessThan => &data.len() < len,
                    DataLengthConstraints::GreaterThanOrEquals => &data.len() >= len,
                    DataLengthConstraints::LessThanOrEquals => &data.len() <= len,
                },

                (MatchToken::AnyData, ScriptBit::Push(_)) => true,
                (MatchToken::AnyData, ScriptBit::PushData(_, _)) => true,

                (MatchToken::Signature, ScriptBit::Push(sig_buf)) => Signature::from_compact_impl(sig_buf).is_ok(),

                (MatchToken::PublicKey, ScriptBit::Push(pubkey_buf)) => PublicKey::from_bytes_impl(pubkey_buf).is_ok(),

                (MatchToken::PublicKeyHash, ScriptBit::Push(pubkeyhash_buf)) => pubkeyhash_buf.len() == 20, // OP_HASH160

                _ => false,
            };

            if !is_match {
                return Err(ScriptTemplateErrors::MatchFailure(i, template.clone(), script.clone()));
            }

            // Now that we know script bit is a match, we can add the data parts to the matches array.
            match (template, script) {
                (MatchToken::Data(_, _), ScriptBit::PushData(_, data) | ScriptBit::Push(data)) => matches.push((MatchDataTypes::Data, data.clone())),

                (MatchToken::AnyData, ScriptBit::Push(data)) => matches.push((MatchDataTypes::Data, data.clone())),
                (MatchToken::AnyData, ScriptBit::PushData(_, data)) => matches.push((MatchDataTypes::Data, data.clone())),

                (MatchToken::Signature, ScriptBit::Push(data)) => matches.push((MatchDataTypes::Signature, data.clone())),

                (MatchToken::PublicKey, ScriptBit::Push(data)) => matches.push((MatchDataTypes::PublicKey, data.clone())),

                (MatchToken::PublicKeyHash, ScriptBit::Push(data)) => matches.push((MatchDataTypes::PublicKeyHash, data.clone())), // OP_HASH160
                _ => (),
            }
        }

        Ok(matches)
    }

    pub fn test_impl(&self, script_template: &ScriptTemplate) -> bool {
        self.match_impl(script_template).is_ok()
    }
}

#[cfg(not(feature = "wasm-bindgen-script-template"))]
impl Script {
    /// Matches the Script against the provided ScriptTemplate.
    ///
    /// If any data can be gleaned from the Script (ie. OP_DATA, OP_PUBKEY, OP_SIG, etc.), it will return it in a `Vec<Match>`
    ///
    /// # Example
    /// ```
    /// use bsv_wasm::{ Script, MatchDataTypes, ScriptTemplate };
    ///
    /// let script = Script::from_asm_string("OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 21e8").unwrap();
    /// let script_template = ScriptTemplate::from_asm_string("OP_HASH160 OP_DATA=20 OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();
    ///
    /// let match_result = script.matches(&script_template);
    /// let extracted = match_result.unwrap();
    /// assert_eq!(extracted.len(), 3);
    /// match &extracted[0] {
    ///    (MatchDataTypes::Data, v) => {
    ///        assert_eq!(v.len(), 20, "Data was not 20 bytes long");
    ///        assert_eq!(v, &hex::decode("b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6").unwrap())
    ///    }
    ///    _ => assert!(false, "Index 0 did not contain Signature"),
    /// }
    /// ```
    pub fn matches(&self, script_template: &ScriptTemplate) -> Result<Vec<Match>, ScriptTemplateErrors> {
        self.match_impl(script_template)
    }

    /// Matches the Script against the provided ScriptTemplate.
    ///
    /// Returns `true` if the Script matches the ScriptTemplate.
    pub fn is_match(&self, script_template: &ScriptTemplate) -> bool {
        self.test_impl(script_template)
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"))]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen-script-template"), wasm_bindgen)]
impl Script {
    /// Matches the Script against the provided ScriptTemplate.
    ///
    /// If any data can be gleaned from the Script (ie. OP_DATA, OP_PUBKEY, OP_SIG, etc.), it will return it in a `Vec<Match>`
    /// @returns {[string, Uint8Array][]}
    pub fn matches(&self, script_template: &ScriptTemplate) -> Result<JsValue, JsValue> {
        let matches = match self.match_impl(script_template) {
            Ok(v) => v,
            Err(e) => throw_str(&e.to_string()),
        };

        match JsValue::from_serde(&matches) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    /// Matches the Script against the provided ScriptTemplate.
    ///
    /// Returns `true` if the Script matches the ScriptTemplate.
    /// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = isMatch))]
    pub fn is_match(&self, script_template: &ScriptTemplate) -> bool {
        self.test_impl(script_template)
    }
}
