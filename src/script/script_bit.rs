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
    // "OP_RETURN" <non-script-data>,
    // Any bytes after an OP_RETURN that is not in a branch block are not evaluated and there are no grammatical requirements for those bytes.
    // https://github.com/bitcoin-sv-specs/protocol/blob/master/updates/genesis-spec.md#formal-grammar-for-bitcoin-script
    NonScriptData(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
    Coinbase(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] Vec<u8>),
}
