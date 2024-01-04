use crate::{BSVErrors, OpCodes};
use thiserror::*;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Number provided is out of range of an i32")]
    NumberOutOfRange,
    #[error("Stack is empty")]
    EmptyStack,

    #[error("NonScriptData can not be pushed")]
    NonScriptData,

    #[error("Invalid OpCode: {0}")]
    InvalidOpcode(OpCodes),

    #[error("Data on top of stack is too long to be casted to a boolean.")]
    TooLongForBool,

    #[error("Item on top of the stack was not true")]
    VerifyFailed,

    #[error("Provided OpCode is disabled {0}")]
    DisabledOpCode(&'static OpCodes),

    #[error("Provided OpCode is requires a transaction {0}")]
    RequiresTransaction(&'static OpCodes),

    #[error("Failed to convert byte to SigHash flag")]
    FailedToConvertSighash,

    #[error("Stack operation is invalid {0}")]
    InvalidStackOperation(&'static str),

    #[error("The TxIn was not provided for this transaction")]
    NoTxInProvided,

    #[error("Could not calculate SigHash preimage {0}")]
    SighashPreimageCalculation(String),

    #[error("{0}")]
    BSVErrors(#[from] BSVErrors),
}
