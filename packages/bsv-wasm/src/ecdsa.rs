use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}

impl Into<bsv::SigningHash> for SigningHash {
    fn into(self) -> bsv::SigningHash {
        match self {
            SigningHash::Sha256 => bsv::SigningHash::Sha256,
            SigningHash::Sha256d => bsv::SigningHash::Sha256d,
        }
    }
}
