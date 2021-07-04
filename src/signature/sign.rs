use digest::{BlockInput, FixedOutput, Reset, Update, consts::U32};
use ecdsa::{hazmat::{FromDigest, RecoverableSignPrimitive}, rfc6979::{self, generate_k}};
use k256::{Scalar, SecretKey, ecdsa::Signature};

use crate::{hash::sha256_digest::ReversibleDigest, sha256_digest::Sha256};

pub fn sign_custom_preimage<D>(
    priv_key: &SecretKey,
    digest: D,
    reverse_endian_k: bool,
) -> Result<(Signature, bool), ecdsa::Error>
    where D: FixedOutput<OutputSize = U32> + BlockInput + Clone + Default + Reset + Update + ReversibleDigest,
{
    // Add this for non deterministic K
    // let mut added_entropy = FieldBytes::<C>::default();
    //     rng.fill_bytes(&mut added_entropy);

    let priv_scalar = priv_key.to_secret_scalar();

    let k_digest = match reverse_endian_k {
        true => digest.reverse(),
        false => digest.clone()
    };
    let k = **generate_k(&priv_scalar, k_digest, &[]);

    let msg_scalar = Scalar::from_digest(digest);
    priv_scalar.try_sign_recoverable_prehashed(&k, &msg_scalar)
}