use crate::utils::{from_hex, to_hex};
use crate::OpCodes;
use serde::*;
use strum_macros::Display;

#[derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptBit {
    OpCode(OpCodes),
    If { code: OpCodes, pass: Vec<ScriptBit>, fail: Option<Vec<ScriptBit>> },
    Push(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
    PushData(OpCodes, #[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
    Coinbase(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
}
