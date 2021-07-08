use crate::PrivateKeyErrors;
use crate::PublicKeyErrors;
use snafu::*;

#[derive(Debug, Snafu)]
pub enum ExtendedPrivateKeyErrors {
    #[snafu(display("Could not generate randomness: {}", error))]
    RandomnessGenerationError { error: anyhow::Error },
    #[snafu(display("Could not calculate private key bytes from seed: {}", error))]
    InvalidSeedHmacError { error: anyhow::Error },
    #[snafu(display("Could not calculate private key: {}", error))]
    InvalidPrivateKeyError { error: PrivateKeyErrors },
    #[snafu(display("Could not calculate public key: {}", error))]
    InvalidPublicKeyError { error: PublicKeyErrors },
    #[snafu(display("Could not serialise xpriv: {}", error))]
    SerialisationError { error: anyhow::Error },

    #[snafu(display("Could not derive xpriv: {}", error))]
    DerivationError { error: anyhow::Error },
}
