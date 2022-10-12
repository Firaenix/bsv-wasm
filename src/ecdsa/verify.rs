use crate::BSVErrors;
use crate::Signature;
use crate::{get_hash_digest, PublicKey, SigningHash, ECDSA};
use ecdsa::signature::DigestVerifier;
use k256::{ecdsa::VerifyingKey, EncodedPoint};

impl ECDSA {
    pub(crate) fn verify_digest_impl(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let point = EncodedPoint::from_bytes(pub_key_bytes).map_err(|e| BSVErrors::CustomECDSAError(e.to_string()))?;
        let key = VerifyingKey::from_encoded_point(&point)?;
        let digest = get_hash_digest(hash_algo, message);
        key.verify_digest(digest, &signature.sig)?;
        Ok(true)
    }
}

impl ECDSA {
    pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, BSVErrors> {
        ECDSA::verify_digest_impl(message, pub_key, signature, hash_algo)
    }
}
