use crate::{ReversibleDigest, Sha256r, SigningHash};
use digest::consts::U32;
use digest::{generic_array::GenericArray, Digest, FixedOutput};
// Reenable this when trait aliases become stable
// pub trait Digest32 = digest::FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest;

pub trait HashDigest: FixedOutput<OutputSize = U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest {}

impl HashDigest for Sha256r {}

pub type HashBuffer = GenericArray<u8, U32>;

pub fn get_hash_digest(hash_algo: SigningHash, preimage: &[u8]) -> impl HashDigest {
    match hash_algo {
        SigningHash::Sha256 => Digest::chain(Sha256r::default(), preimage),
        SigningHash::Sha256d => Digest::chain(Sha256r::default(), Sha256r::digest(preimage)),
    }
}

/* pub fn get_hash_digest_bytes(hash_algo: SigningHash, preimage: &[u8]) -> HashBuffer {
    get_hash_digest(hash_algo, preimage).finalize_fixed()
}
 */
