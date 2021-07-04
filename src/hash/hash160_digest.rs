use digest::{BlockInput, FixedOutputDirty, Reset, Update, consts::{U64, U32}, Digest};

use ripemd160::Ripemd160;
use sha2::Sha256;


#[derive(Clone)]
pub struct Hash160 { 
    engine: Sha256,
    reverse: bool
}

pub trait ReversibleDigest  {
    /**
     * Returns a reversed Hash160version of the given digest
     */
    fn reverse(&self) -> Self;
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
      Self{ engine, reverse }
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
      let first_hash = &*self.engine.finalize();
      let finished_hash = Ripemd160::digest(first_hash);
      let mut vec = finished_hash.to_vec();

      if self.reverse {
          vec.reverse()
      }

      out.copy_from_slice(&vec);
    }
}

impl Reset for Hash160 {
    fn reset(&mut self) {
        self.engine = Sha256::default();
    }
}

digest::impl_write!(Hash160);