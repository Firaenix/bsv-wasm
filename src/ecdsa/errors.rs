use crate::SignatureErrors;
use thiserror::*;

#[derive(Debug, Error)]
pub enum ECDSAErrors {
  #[error("{}", source)]
  PrimitiveError {
    #[from]
    source: ecdsa::Error,
  },

  #[error("{}", source)]
  CurveError {
    #[from]
    source: elliptic_curve::Error,
  },

  #[error("{}", source)]
  SignatureError {
    #[from]
    source: SignatureErrors,
  },
}
