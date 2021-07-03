use elliptic_curve::{Curve, FieldBytes};
use k256::NonZeroScalar;
use digest::{BlockInput, Digest, FixedOutput, Reset, Update, consts::U32};
use ecdsa::{SigningKey, hazmat::{FromDigest, RecoverableSignPrimitive}, rfc6979};
use k256::ProjectivePoint;
use std::{borrow::Borrow, ops::Mul};
use elliptic_curve::{FieldSize, ProjectiveArithmetic, ops::Invert, zeroize::{Zeroize, Zeroizing}};
use k256::{Scalar, SecretKey, ecdsa::Signature};

use crate::{Sha256d, sha256_digest::Sha256};

pub fn sign_custom_preimage(
    priv_key: &SecretKey,
    preimage: &[u8],
    // msg_digest: Sha256,
    k_digest: Sha256
) -> Result<(Signature, bool), ecdsa::Error>
{
    let priv_scalar = priv_key.to_secret_scalar();

    // Reverse digest for K generation
    let k = **generate_k(&priv_scalar, k_digest.clone(), &[]);
    // Dont reverse digest for scalar conversion

    // assert_eq!(k_digest.clone().finalize().to_vec(), preimage, "k_digest must match preimage");
    // preimage.reverse();

    let hashed_scalar = *SecretKey::from_bytes(preimage).unwrap().to_secret_scalar();

    // let mut digest = k_digest;
    // let other_scalar = Scalar::from_digest(msg_digest);

    // assert_eq!(other_scalar, hashed_scalar, "Hash Scalars dont match");

    priv_scalar.try_sign_recoverable_prehashed(&k, &hashed_scalar)
}

pub fn generate_k<C, D>(
    secret_scalar: &NonZeroScalar<C>,
    msg_digest_scalar: &Scalar,
    additional_data: &[u8],
) -> Zeroizing<NonZeroScalar<C>>
where
    C: Curve + ProjectiveArithmetic,
    D: FixedOutput<OutputSize = FieldSize<C>> + BlockInput + Clone + Default + Reset + Update,
    Scalar<C>: FromDigest<C> + Invert<Output = Scalar<C>> + Zeroize,
{
    let mut x = secret_scalar.to_repr();
    let h1 = msg_digest_scalar.to_repr();
    let mut hmac_drbg = HmacDrbg::<D>::new(&x, &h1, additional_data);
    x.zeroize();

    loop {
        let mut tmp = FieldBytes::<C>::default();
        hmac_drbg.generate_into(&mut tmp);
        if let Some(k) = NonZeroScalar::from_repr(tmp) {
            return Zeroizing::new(k);
        }
    }
}