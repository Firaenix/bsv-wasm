use crate::SignatureErrors;
use hex::FromHexError;
use snafu::*;

#[derive(Debug, Snafu)]
pub enum PrivateKeyErrors {
    #[snafu(display("Could not decode Base58 string: {} {}", string, error))]
    Base58Decode {
        error: anyhow::Error,
        string: String,
    },

    #[snafu(display("Could not parse hex: {}", error))]
    ParseHex {
        error: FromHexError,
    },

    #[snafu(display("Could not parse hex: {}", error))]
    ByteDecode {
        error: anyhow::Error,
    },

    #[snafu(display("Invalid Point: {}", error))]
    InvalidPoint {
        error: elliptic_curve::Error,
    },

    #[snafu(display("Could not generate secret key: {}", error))]
    SecretKey {
        error: anyhow::Error,
    },

    SignatureError {
        error: anyhow::Error,
    },

    #[snafu(display("Something went wrong: {}", message))]
    Other {
        message: String,
    },
}
