use crate::ECDSAErrors;
use hex::FromHexError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum PublicKeyErrors {
  #[error("{}", source)]
  Hex {
    #[from]
    source: FromHexError,
  },

  #[error("{}", source)]
  ECDSA {
    #[from]
    source: ECDSAErrors,
  },
}
