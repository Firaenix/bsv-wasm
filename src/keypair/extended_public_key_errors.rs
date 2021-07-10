use crate::PublicKeyErrors;
use thiserror::*;

#[derive(Debug, Error)]
pub enum ExtendedPublicKeyErrors {
    #[error("Could not generate randomness: {}", error)]
    RandomnessGenerationError { error: anyhow::Error },

    #[error("Could not calculate private key bytes from seed: {}", error)]
    InvalidSeedHmacError { error: anyhow::Error },

    #[error("Could not calculate public key: {}", error)]
    InvalidPublicKeyError { error: PublicKeyErrors },

    #[error("Could not get public key point: {}", error)]
    PublicKeyPointError { error: anyhow::Error },

    #[error("Could not serialise xpub: {}", error)]
    SerialisationError { error: anyhow::Error },

    #[error("Could not derive xpub: {}", error)]
    DerivationError { error: anyhow::Error },
}
