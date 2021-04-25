use snafu::*;
use anyhow::*;

#[derive(Debug, Snafu)]
pub enum TxInErrors {
    #[snafu(display("Error deserialising TxIn: {}", error))]
    Deserialise {
      error: anyhow::Error
    }
}