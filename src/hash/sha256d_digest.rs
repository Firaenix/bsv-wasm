use digest::{
    consts::{U32, U64},
    BlockInput, Digest, FixedOutput, Reset, Update,
};

use sha2::Sha256;

use crate::reverse_digest::ReversibleDigest;

#[derive(Clone, Default)]
pub struct Sha256d {
    engine: Sha256,
    reverse: bool,
}

impl ReversibleDigest for Sha256d {
    fn reverse(&self) -> Self {
        let mut reversed = self.clone();
        reversed.reverse = true;
        reversed
    }
}

impl BlockInput for Sha256d {
    type BlockSize = U64;
}

impl Update for Sha256d {
    fn update(&mut self, data: impl AsRef<[u8]>) {
        Digest::update(&mut self.engine, data)
    }
}

impl FixedOutput for Sha256d {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut digest::generic_array::GenericArray<u8, Self::OutputSize>) {
        let first_hash = &*self.engine.finalize();
        let mut finished_hash = Sha256::digest(first_hash);
        // let mut vec = finished_hash.to_vec();

        if self.reverse {
            finished_hash.reverse()
        }

        out.copy_from_slice(&*finished_hash)
    }

    fn finalize_into_reset(&mut self, out: &mut digest::generic_array::GenericArray<u8, Self::OutputSize>) {
        self.clone().finalize_into(out);
        digest::Reset::reset(self);
    }

    fn finalize_fixed(self) -> digest::generic_array::GenericArray<u8, Self::OutputSize>
    where
        Self: Sized,
    {
        let mut out = Default::default();
        self.finalize_into(&mut out);
        out
    }

    fn finalize_fixed_reset(&mut self) -> digest::generic_array::GenericArray<u8, Self::OutputSize> {
        let mut out = Default::default();
        self.finalize_into_reset(&mut out);
        out
    }
}

impl Reset for Sha256d {
    fn reset(&mut self) {
        self.engine = Sha256::default();
    }
}

digest::impl_write!(Sha256d);
