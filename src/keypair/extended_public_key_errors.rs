use crate::PublicKeyErrors;
use anyhow::*;
use snafu::*;
use anyhow::*;

#[derive(Debug, Snafu)]
pub enum ExtendedPublicKeyErrors {
  #[snafu(display("Could not generate randomness: {}", error))]
  RandomnessGenerationError { error: anyhow::Error },

  #[snafu(display("Could not calculate private key bytes from seed: {}", error))]
  InvalidSeedHmacError { error: anyhow::Error },

  #[snafu(display("Could not calculate public key: {}", error))]
  InvalidPublicKeyError { error: PublicKeyErrors },

  #[snafu(display("Could not get public key point: {}", error))]
  PublicKeyPointError { error: anyhow::Error },

  #[snafu(display("Could not serialise xpub: {}", error))]
  SerialisationError { error: anyhow::Error },

  #[snafu(display("Could not derive xpub: {}", error))]
  DerivationError { error: anyhow::Error },
}
