#[macro_use]
extern crate num_derive;

#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[cfg(target_arch = "wasm32")]
// extern crate console_error_panic_hook;
// // #[cfg(target_arch = "wasm32")]
// console_error_panic_hook::set_once();

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
