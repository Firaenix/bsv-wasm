use snafu::*;

#[derive(Debug, Snafu)]
pub enum PBKDF2Errors {
    #[snafu(display("Could not use given salt: {}", error))]
    UseSaltError {
      error: anyhow::Error
    },

    #[snafu(display("Could not calculate PBKDF2 Hash: {}", error))]
    HashError {
      error: anyhow::Error,
    },
}