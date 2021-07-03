use digest::{BlockInput, FixedOutputDirty, Reset, Update, consts::{U64, U32}};
use bitcoin_hashes::{*, self, Hash, HashEngine};

#[derive(Clone)]
pub struct Sha256d { 
    engine: sha256::HashEngine,
    pub reverse: bool
}

impl Sha256d {
    pub fn new(reverse: bool) -> Self {
        Self{ engine: sha256d::Hash::engine(), reverse }
    }
}

impl Default for Sha256d {
    fn default() -> Self {
        Self{ engine: sha256d::Hash::engine(), reverse: false }
    }
}

impl BlockInput for Sha256d {
    type BlockSize = U64;
}

impl Update for Sha256d {
    fn update(&mut self, input: impl AsRef<[u8]>) {
      self.engine.input(input.as_ref())
    }
}

impl FixedOutputDirty for Sha256d {
    type OutputSize = U32;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
      let finished_hash = sha256d::Hash::from_engine(self.engine.clone());
      let mut vec = finished_hash.to_vec();

      if self.reverse {
          vec.reverse()
      }

      for (chunk, v) in out.chunks_exact_mut(4).zip(vec.chunks_exact(4)) {
          chunk.copy_from_slice(&v);
      }
    }
}

impl Reset for Sha256d {
    fn reset(&mut self) {
        self.engine = sha256d::Hash::engine();
    }
}

digest::impl_write!(Sha256d);