pub mod ecdh;
pub mod recover;
pub mod sign;
pub mod verify;

pub use ecdh::*;

/**
 * Utility struct for low level ECDSA primitives
 */
pub struct ECDSA {}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}
