use crate::reverse_digest::ReversibleDigest;
use digest::consts::U20;
use digest::{
    consts::{U32, U64},
    BlockInput, Digest, FixedOutputDirty, Reset, Update,
};

use ripemd160::Ripemd160;
use sha2::Sha256;

#[derive(Clone)]
pub struct Hash160 {
    engine: Sha256,
    reverse: bool,
}

impl ReversibleDigest for Hash160 {
    fn reverse(&self) -> Self {
        let mut reversed = self.clone();
        reversed.reverse = true;
        reversed
    }
}

impl Hash160 {
    pub fn new(reverse: bool) -> Self {
        let engine: Sha256 = Sha256::default();
        Self { engine, reverse }
    }
}

impl Default for Hash160 {
    fn default() -> Self {
        Self::new(false)
    }
}

impl BlockInput for Hash160 {
    type BlockSize = U64;
}

impl Update for Hash160 {
    fn update(&mut self, input: impl AsRef<[u8]>) {
        digest::Digest::update(&mut self.engine, input.as_ref())
    }
}

impl FixedOutputDirty for Hash160 {
    type OutputSize = U20;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
        let first_hash = self.clone().engine.finalize();
        let finished_hash = &mut *Ripemd160::digest(&*first_hash);
        if self.reverse {
            finished_hash.reverse()
        }

        out.copy_from_slice(&finished_hash);
    }
}

impl Reset for Hash160 {
    fn reset(&mut self) {
        self.engine = Sha256::default();
    }
}

digest::impl_write!(Hash160);
