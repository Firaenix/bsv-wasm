use digest::{BlockInput, FixedOutputDirty, Reset, Update, consts::{U64, U32}, Digest};

use sha2::Sha256;


#[derive(Clone)]
pub struct Sha256d { 
    engine: Sha256,
    reverse: bool
}

pub trait ReversibleDigest  {
  /**
   * Returns a reversed Sha256dversion of the given digest
   */
  fn reverse(&self) -> Self;
}

impl ReversibleDigest for Sha256d {
  fn reverse(&self) -> Self {
    let mut reversed = self.clone();
    reversed.reverse = true;
    reversed
  }
}

impl Digest for Sha256d {
    type OutputSize = U32;

    fn new() -> Self {
      let engine: Sha256 = Sha256::default();
      Self{ engine, reverse: false }
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
      self.engine.update(data)
    }

    fn chain(self, data: impl AsRef<[u8]>) -> Self
    where
        Self: Sized {
        self.engine.update(data);
        self
    }

    fn finalize(self) -> digest::Output<Self> {

        // self.finalize_into_dirty(out: &mut digest::Output<Self>);
    }

    fn finalize_reset(&mut self) -> digest::Output<Self> {
      let finaled = self.finalize();
      self.reset();

      finaled
    }

    fn reset(&mut self) {
      self.engine = Sha256::default();
    }

    fn output_size() -> usize {
        32
    }

    fn digest(data: &[u8]) -> digest::Output<Self> {
        Sha256::digest(&*Sha256::digest(data))
    }
}

impl FixedOutputDirty for Sha256d {
    type OutputSize = U32;

    fn finalize_into_dirty(&mut self, out: &mut digest::Output<Self>) {
      let first_hash = &*self.engine.finalize();
      let finished_hash = Sha256::digest(first_hash);
      let mut vec = finished_hash.to_vec();

      if self.reverse {
          vec.reverse()
      }

      out.copy_from_slice(&vec);
    }
}
digest::impl_write!(Sha256d);