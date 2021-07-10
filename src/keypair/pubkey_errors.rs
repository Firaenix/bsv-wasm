use anyhow::*;
use hex::FromHexError;
use std::convert::TryInto;
use thiserror::*;

#[derive(Debug, Error)]
pub enum PublicKeyErrors {
    #[error("Could not decode Base58 string: {} {}", string, message)]
    Base58Decode { message: String, string: String },

    #[error("Could not parse hex: {}", error)]
    ParseHex { error: FromHexError },

    #[error("Could not parse hex: {}", message)]
    ByteDecode { message: String },

    #[error("Invalid Point: {}", error)]
    InvalidPoint { error: elliptic_curve::Error },

    #[error("Something went wrong: {}", message)]
    Other { message: String },
}
