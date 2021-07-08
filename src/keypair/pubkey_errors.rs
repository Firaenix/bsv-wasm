use hex::FromHexError;
use snafu::*;

#[derive(Debug, Snafu)]
pub enum PublicKeyErrors {
    #[snafu(display("Could not decode Base58 string: {} {}", string, message))]
    Base58Decode { message: String, string: String },

    #[snafu(display("Could not parse hex: {}", error))]
    ParseHex { error: FromHexError },

    #[snafu(display("Could not parse hex: {}", message))]
    ByteDecode { message: String },

    #[snafu(display("Invalid Point: {}", error))]
    InvalidPoint { error: elliptic_curve::Error },

    #[snafu(display("Something went wrong: {}", message))]
    Other { message: String },
}
