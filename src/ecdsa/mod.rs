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

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// Change the byte order of the `k` digest or message digest when generating `ECDSA` signatures.
pub enum DigestAction {
    None,
    ReverseK,
    ReverseDigest,
    ReverseKAndDigest,
}
