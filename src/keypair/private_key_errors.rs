use hex::FromHexError;
use thiserror::*;

#[derive(Debug, Error)]
pub enum PrivateKeyErrors {
  #[error("{}: {}", message, error)]
  ToWIF { message: String, error: anyhow::Error },

  #[error("{}", source)]
  Hex {
    #[from]
    source: FromHexError,
  },
}
