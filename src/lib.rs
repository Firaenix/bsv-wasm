#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]


use wasm_bindgen::prelude::*;


#[wasm_bindgen(js_name=configureStacktrace)]
pub fn configure_stacktrace() {
    console_error_panic_hook::set_once();
}

#[macro_use]
extern crate num_derive;

pub mod chainparams;
pub use chainparams::*;

pub mod keypair;
pub use keypair::*;

pub mod signature;
pub use signature::*;

pub mod traits;
pub use traits::*;

pub mod errors;
pub use errors::*;

pub mod address;
pub use address::*;

pub mod utils;
pub use utils::*;

pub mod transaction;
pub use transaction::*;

pub mod script;
pub use script::*;

pub mod hash;
pub use hash::*;

pub mod kdf;
pub use kdf::*;

pub mod encryption;
pub use encryption::*;

pub mod ecdsa;
pub use crate::ecdsa::*;

pub mod bsm;
pub use bsm::*;

pub mod ecies;
pub use ecies::*;
