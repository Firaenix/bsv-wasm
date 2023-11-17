use crate::BSVErrors;
use crate::ReversibleDigest;
use crate::Signature;
use crate::{get_hash_digest, PublicKey, SigningHash, ECDSA};
use ecdsa::signature::DigestVerifier;
use k256::{ecdsa::VerifyingKey, EncodedPoint};

impl ECDSA {
    pub(crate) fn verify_digest_impl(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash, reverse_k: bool) -> Result<bool, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let point = EncodedPoint::from_bytes(pub_key_bytes).map_err(|e| BSVErrors::CustomECDSAError(e.to_string()))?;
        let key = VerifyingKey::from_encoded_point(&point)?;
        let digest = get_hash_digest(hash_algo, message);
        if reverse_k {
            digest.reverse();
            key.verify_digest(digest, &signature.sig)?;
        } else {
            key.verify_digest(digest, &signature.sig)?;
        }
        // or we can always check the reverse digest when the first verification fails to avoid the reverse_k flags
        // key.verify_digest(digest, &signature.sig).or_else(|_| {
        //     let rev: Vec<u8> = message.iter().rev().copied().collect();
        //     let reversed = get_hash_digest(hash_algo, &rev);
        //     key.verify_digest(reversed, &signature.sig)
        // })?;
        Ok(true)
    }
}

impl ECDSA {
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash, reverse_k: bool) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo, reverse_k)
    }
}
