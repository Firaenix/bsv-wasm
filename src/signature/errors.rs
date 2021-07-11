use hex::FromHexError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum SignatureErrors {
    #[error("Could not decode Base58 string: {} {}", string, message)]
    Base58Decode { message: String, string: String },
    #[error("Could not parse hex: {}", error)]
    ParseHex { error: FromHexError },

    #[error("Could not parse hex: {}", message)]
    ByteDecode { message: String },

    #[error("Invalid Point: {}", error)]
    InvalidPoint { error: elliptic_curve::Error },

    #[error("{}", error)]
    SecpError { error: k256::ecdsa::Error },

    #[error("Something went wrong: {}", message)]
    Other { message: String },

    #[error("Unable to recover public key from signature {}", error)]
    DerivePublicKey { error: anyhow::Error },
}
