use snafu::*;
use anyhow::*;

#[derive(Debug, Snafu)]
pub enum TransactionErrors {
    #[snafu(display("Error deserialising transaction: {}", error))]
    Deserialise {
      error: anyhow::Error
    },
    #[snafu(display("Error serialising transaction: {}", error))]
    Serialise {
      error: anyhow::Error
    }
}