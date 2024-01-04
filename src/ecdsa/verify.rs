use crate::BSVErrors;
use crate::DigestBytes;
use crate::Signature;
use crate::{get_hash_digest, PublicKey, SigningHash, ECDSA};
use digest::generic_array::GenericArray;
use ecdsa::hazmat::VerifyPrimitive;
use ecdsa::signature::DigestVerifier;
use elliptic_curve::ops::Reduce;
use elliptic_curve::sec1::FromEncodedPoint;
use elliptic_curve::AffinePoint;
use k256::Secp256k1;
use k256::{ecdsa::VerifyingKey, EncodedPoint, Scalar, U256};

impl ECDSA {
    pub(crate) fn verify_digest_impl(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let point = EncodedPoint::from_bytes(pub_key_bytes).map_err(|e| BSVErrors::CustomECDSAError(e.to_string()))?;
        let key = VerifyingKey::from_encoded_point(&point)?;
        let digest = get_hash_digest(hash_algo, message);
        key.verify_digest(digest, &signature.sig)?;
        Ok(true)
    }

    pub(crate) fn verify_hashbuf_impl(digest: DigestBytes, pub_key: &PublicKey, signature: &Signature) -> Result<bool, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let z = <Scalar as Reduce<U256>>::from_be_bytes_reduced(digest);
        let point = EncodedPoint::from_bytes(pub_key_bytes).map_err(|e| BSVErrors::CustomECDSAError(e.to_string()))?;
        let key: AffinePoint<Secp256k1> = AffinePoint::<Secp256k1>::from_encoded_point(&point).unwrap();
        key.verify_prehashed(z, &signature.sig)?;
        Ok(true)
    }
}

impl ECDSA {
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo)
    }

    pub fn verify_hashbuf(digest: &[u8], pub_key: &PublicKey, signature: &Signature) -> Result<bool, BSVErrors> {
        ECDSA::verify_hashbuf_impl(*GenericArray::from_slice(digest), pub_key, signature)
    }
}
