use crate::SignatureErrors;
use hex::FromHexError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum PrivateKeyErrors {
    #[error("Could not decode Base58 string: {} {}", string, error)]
    Base58Decode { error: anyhow::Error, string: String },

    #[error("Could not parse hex: {}", error)]
    ParseHex { error: FromHexError },

    #[error("Could not parse hex: {}", error)]
    ByteDecode { error: anyhow::Error },

    #[error("Invalid Point: {}", error)]
    InvalidPoint { error: elliptic_curve::Error },

    #[error("Could not generate secret key: {}", error)]
    SecretKey { error: anyhow::Error },

    #[error("Could not generate secret key: {}", error)]
    SignatureError { error: anyhow::Error },

    #[error("Something went wrong: {}", message)]
    Other { message: String },
}
