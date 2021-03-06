pub mod ecdh;
pub mod recover;
pub mod sign;
pub mod verify;

pub use ecdh::*;


use wasm_bindgen::prelude::*;

/**
 * Utility struct for low level ECDSA primitives
 */
#[cfg_attr(all(feature = "wasm-bindgen-ecdsa"), wasm_bindgen)]
pub struct ECDSA {}

#[cfg_attr(all(feature = "wasm-bindgen-ecdsa"), wasm_bindgen)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}
