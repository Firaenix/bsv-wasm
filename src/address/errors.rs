use hex::FromHexError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum AddressErrors {
    #[error("Could not decode Base58 string: {} {}", string, error)]
    Base58Decode { error: anyhow::Error, string: String },

    #[error("{:#?}", error)]
    PublicKeyError { error: anyhow::Error },

    #[error("Could not parse hex {}: {}", hex, error)]
    ParseHex { hex: String, error: FromHexError },
}
