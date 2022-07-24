use bsv::TxOut as BSVTxOut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TxOut(pub(crate) BSVTxOut);
