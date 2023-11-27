use digest::{Digest, FixedOutput};

use crate::{ReversibleDigest, Sha256r, SigningHash};

pub trait ToDigest {
    type Digest: digest::FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + FixedOutput + ReversibleDigest;

    fn to_digest(&self, hash_algo: SigningHash, reverse: bool) -> Self::Digest;
}

impl<T> ToDigest for T
where
    T: AsRef<[u8]>,
{
    type Digest = Sha256r;

    fn to_digest(&self, hash_algo: SigningHash, reverse: bool) -> Self::Digest {
        let d = match hash_algo {
            SigningHash::Sha256 => Digest::chain(Self::Digest::default(), self.as_ref()),
            SigningHash::Sha256d => Digest::chain(Self::Digest::default(), Self::Digest::digest(self.as_ref())),
        };

        match reverse {
            true => d.reverse(),
            false => d,
        }
    }
}
