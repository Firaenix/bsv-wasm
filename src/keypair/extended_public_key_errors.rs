use crate::ExtendedPrivateKeyErrors;
use std::any;

use thiserror::*;

use crate::ECDSAErrors;

#[derive(Debug, Error)]
pub enum ExtendedPublicKeyErrors {
    #[error("Could not generate randomness: {0}")]
    RandomnessGenerationError(#[from] getrandom::Error),

    #[error("Could not calculate private key bytes from seed: {0}")]
    InvalidSeedHmacError(String),

    #[error("Could not get public key point: {}", error)]
    PublicKeyPointError { error: ECDSAErrors },

    #[error("Could not serialise xpub: {}", error)]
    SerialisationError { error: String },

    #[error("Could not derive xpub: {0}")]
    DerivationError(String),

    #[error("{}", source)]
    ECDSA {
        #[from]
        source: ECDSAErrors,
    },

    #[error("{}", source)]
    Curve {
        #[from]
        source: elliptic_curve::Error,
    },

    #[error("{}", source)]
    XPriv {
        #[from]
        source: ExtendedPrivateKeyErrors,
    },
}
