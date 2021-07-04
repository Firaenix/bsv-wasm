use crate::hash::FixedOutput;
use sha2::Sha256;
use digest::{BlockInput, Digest, FixedOutputDirty, Reset, Update, consts::{U64, U32}};

use crate::reverse_digest::ReversibleDigest;

/**
 *  Sha256 reversible - needed to convert the output of a Sha256 hash into Little Endian for SigHash BIP143
 */
#[derive(Clone, Default)]
pub struct Sha256r {
    engine: Sha256,
    reverse: bool
}

impl ReversibleDigest for Sha256r {
    fn reverse(&self) -> Self {
        let mut reversed = self.clone();
        reversed.reverse = true;
        reversed
    }
}

impl BlockInput for Sha256r {
    type BlockSize = U64;
}

impl Update for Sha256r {
    fn update(&mut self, data: impl AsRef<[u8]>) {
      Digest::update(&mut self.engine, data)
    }
}

impl FixedOutput for Sha256r {
    type OutputSize = U32;

    fn finalize_into(self, out: &mut digest::generic_array::GenericArray<u8, Self::OutputSize>) {
      let finalised_hash = &mut *self.engine.finalize();
      if self.reverse {
        finalised_hash.reverse()
      }

      out.copy_from_slice(&*finalised_hash);
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

impl Reset for Sha256r {
    fn reset(&mut self) {
      self.engine = Sha256::default();
    }
}

digest::impl_write!(Sha256r);