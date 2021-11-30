pub mod ecdh;
pub mod sign;
pub mod verify;

pub use ecdh::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/**
 * Utility struct for low level ECDSA primitives
 */
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct ECDSA {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}
