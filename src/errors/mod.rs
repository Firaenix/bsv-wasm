use std::{fmt::Display, num::ParseIntError};

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
  Anyhow(
    #[source]
    #[from]
    anyhow::Error,
  ),
  #[error("Unable to recover public key: {0} {1:?}")]
  PublicKeyRecoveryError(String, #[source] Option<ecdsa::Error>),

  #[error("Unable to verify message: {0}")]
  MessageVerification(String),

  #[error("Error when deserialising Script: {0}")]
  DeserialiseScript(String),

  #[error("Error when Serialising Script: {0} {1:?}")]
  SerialiseScript(String, #[source] Option<std::io::Error>),

  #[error("Error generating Script: {0}")]
  GenerateScript(String),

  #[error("Could not calculate private key bytes from seed: {0}")]
  InvalidSeedHmacError(String),

  #[error("Unable to derive child key: {0}")]
  DerivationError(String),
}
