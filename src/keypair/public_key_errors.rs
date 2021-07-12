use crate::BSVErrors;
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
  BSVErrors {
    #[from]
    source: BSVErrors,
  },
}
