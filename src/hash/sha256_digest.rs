use digest::{BlockInput, FixedOutputDirty, Reset, Update, consts::{U64, U32}};
use bitcoin_hashes::{*, self, Hash, HashEngine};

#[derive(Clone)]
pub struct Sha256 { 
    engine: sha256::HashEngine,
    pub reverse: bool
}

pub trait ReversibleDigest  {
    /**
     * Returns a reversed version of the given digest
     */
    fn reverse(&self) -> Self;
  }
  
  impl ReversibleDigest for Sha256 {
    fn reverse(&self) -> Self {
      let mut reversed = self.clone();
      reversed.reverse = true;
      reversed
    }
  }

impl Sha256 {
    pub fn new(reverse: bool) -> Self {
        Self{ engine: sha256::Hash::engine(), reverse }
    }
}

impl Default for Sha256 {
    fn default() -> Self {
        Self{ engine: sha256::Hash::engine(), reverse: false }
    }
}

impl BlockInput for Sha256 {
    type BlockSize = U64;
}

impl Update for Sha256 {
    fn update(&mut self, input: impl AsRef<[u8]>) {
      self.engine.input(input.as_ref())
    }
}

impl FixedOutputDirty for Sha256 {
    type OutputSize = U32;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
      let finished_hash = sha256::Hash::from_engine(self.engine.clone());
      let mut vec = finished_hash.to_vec();

      if self.reverse {
          vec.reverse()
      }

      out.copy_from_slice(&vec);
    }
}

impl Reset for Sha256 {
    fn reset(&mut self) {
        self.engine = sha256::Hash::engine();
    }
}

digest::impl_write!(Sha256);