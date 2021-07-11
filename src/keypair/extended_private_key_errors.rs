use crate::PrivateKeyErrors;
use crate::PublicKeyErrors;
use anyhow::*;
use thiserror::*;

#[derive(Debug, Error)]
pub enum ExtendedPrivateKeyErrors {
    #[error("Could not generate randomness: {}", error)]
    RandomnessGenerationError { error: anyhow::Error },
    #[error("Could not calculate private key bytes from seed: {}", error)]
    InvalidSeedHmacError { error: anyhow::Error },
    #[error("Could not calculate private key: {:#?}", error)]
    InvalidPrivateKeyError { error: PrivateKeyErrors },
    #[error("Could not calculate public key: {}", error)]
    InvalidPublicKeyError { error: anyhow::Error },
    #[error("Could not serialise xpriv: {}", error)]
    SerialisationError { error: anyhow::Error },

    #[error("Could not derive xpriv: {}", error)]
    DerivationError { error: anyhow::Error },
}
