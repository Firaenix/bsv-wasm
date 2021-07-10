use thiserror::*;

#[derive(Debug, Error)]
pub enum PBKDF2Errors {
    #[error("Could not use given salt: {}", error)]
    UseSaltError { error: anyhow::Error },

    #[error("Could not calculate PBKDF2 Hash: {}", error)]
    HashError { error: anyhow::Error },
}
