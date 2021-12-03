use std::{array::TryFromSliceError, fmt::Display, num::ParseIntError};

use thiserror::*;

#[derive(Debug, Error)]
pub enum BSVErrors {
    #[error("{0}")]
    ECDSAError(
        #[source]
        #[from]
        ecdsa::Error,
    ),

    #[error("{0}")]
    CurveError(
        #[source]
        #[from]
        elliptic_curve::Error,
    ),

    #[error("{0}")]
    HexDecode(
        #[source]
        #[from]
        hex::FromHexError,
    ),

    #[error("{0}")]
    Base58Decode(
        #[source]
        #[from]
        bs58::decode::Error,
    ),

    #[error("{0}")]
    Io(
        #[source]
        #[from]
        std::io::Error,
    ),

    #[error("{0}")]
    ParseInt(
        #[source]
        #[from]
        ParseIntError,
    ),

    #[error("{0}")]
    RandomnessGeneration(
        #[source]
        #[from]
        getrandom::Error,
    ),

    #[error("{0}")]
    Json(
        #[source]
        #[from]
        serde_json::Error,
    ),

    #[error("{0}")]
    InvalidKeyIvLength(
        #[source]
        #[from]
        block_modes::InvalidKeyIvLength,
    ),

    #[error("{0}")]
    BlockModeError(
        #[source]
        #[from]
        block_modes::BlockModeError,
    ),

    #[error("{0}")]
    CborSerialise(
        #[source]
        #[from]
        ciborium::ser::Error<std::io::Error>,
    ),

    #[error("{0}")]
    CborDeserialise(
        #[source]
        #[from]
        ciborium::de::Error<std::io::Error>,
    ),

    // Custom Errors
    #[error("Leading byte {0} does not match compressed or uncompressed")]
    PublicKeyReadCompressionByte(u8),

    #[error("Unable to recover public key: {0} {1:?}")]
    PublicKeyRecoveryError(String, #[source] Option<ecdsa::Error>),

    #[error("Unable to verify message: {0}")]
    MessageVerification(String),

    #[error("Error generating Script: {0}")]
    GenerateScript(String),

    #[error("Could not calculate private key bytes from seed: {0}")]
    InvalidSeedHmacError(String),

    #[error("Unable to derive child key: {0}")]
    DerivationError(String),

    #[error("Unable to retrieve private key from WIF: {0}")]
    FromWIF(String),

    #[error("Unable to convert to sighash: {0}")]
    ToSighash(String),

    #[error("Unable to convert from sighash: {0}")]
    FromSighash(String),

    #[error("{0}")]
    OutOfBounds(String),

    #[error("{0}")]
    ECIESError(String),

    #[error("{0}")]
    P2PKHAddress(&'static str),

    #[error("Unable to deserialise P2PKH from slice {0}")]
    P2PKHAddressFromSlice(#[source] TryFromSliceError),

    //=========== Serialisation Errors ==============
    #[error("Error deserialising transaction field {0}: {1}")]
    DeserialiseTransaction(String, #[source] std::io::Error),

    #[error("Error Serialising transaction field {0}: {1}")]
    SerialiseTransaction(String, #[source] std::io::Error),

    #[error("Error when deserialising Script: {0}")]
    DeserialiseScript(String),

    #[error("Error when Serialising Script: {0} {1:?}")]
    SerialiseScript(String, #[source] Option<std::io::Error>),

    #[error("Error deserialising TxIn field {0}: {1}")]
    DeserialiseTxIn(String, #[source] std::io::Error),

    #[error("Error serialising TxIn field {0}: {1}")]
    SerialiseTxIn(String, #[source] std::io::Error),

    #[error("Error deserialising TxOut field {0}: {1}")]
    DeserialiseTxOut(String, #[source] std::io::Error),

    #[error("Error serialising TxOut field {0}: {1}")]
    SerialiseTxOut(String, #[source] std::io::Error),
}
