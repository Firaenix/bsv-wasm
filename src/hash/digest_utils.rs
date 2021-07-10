use digest::{consts::U32, Digest};

use crate::{ReversibleDigest, Sha256r, SigningHash};

pub trait Digest32 = digest::FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest;

pub fn get_hash_digest(hash_algo: SigningHash, preimage: &[u8]) -> impl Digest32 {
  match hash_algo {
    SigningHash::Sha256 => Digest::chain(Sha256r::default(), preimage),
    SigningHash::Sha256d => Digest::chain(Sha256r::default(), Sha256r::digest(preimage)),
  }
}
