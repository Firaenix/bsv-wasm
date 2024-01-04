use crate::get_hash_digest;
use crate::hash::sha256d_digest::Sha256d;
use crate::BSVErrors;
use crate::DigestBytes;
use crate::PrivateKey;
use crate::RecoveryInfo;
use crate::Sha256r;
use crate::Signature;
use crate::SigningHash;
use crate::ECDSA;
use digest::generic_array::GenericArray;
use digest::{Digest, FixedOutput};
use ecdsa::hazmat::{rfc6979_generate_k, SignPrimitive};
use ecdsa::RecoveryId;
use elliptic_curve::ops::Reduce;
use k256::ecdsa::{recoverable, Signature as SecpSignature};
use k256::{FieldBytes, Scalar, SecretKey, U256};
use rand_core::OsRng;
use rand_core::RngCore;
use sha2::Sha256;

impl ECDSA {
    fn sign_preimage_deterministic_k<D>(priv_key: &SecretKey, digest: D, reverse_endian_k: bool) -> Result<(SecpSignature, Option<RecoveryId>), BSVErrors>
    where
        D: FixedOutput<OutputSize = digest::consts::U32> + digest::BlockInput + Clone + Default + digest::Reset + digest::Update + crate::ReversibleDigest,
    {
        let priv_scalar = priv_key.to_nonzero_scalar();
        let final_digest = digest.finalize_fixed();
        let k_digest = match reverse_endian_k {
            true => <Scalar as Reduce<U256>>::from_le_bytes_reduced(final_digest),
            false => <Scalar as Reduce<U256>>::from_be_bytes_reduced(final_digest),
        };

        let k = rfc6979_generate_k::<_, D>(&priv_scalar, &k_digest, &[]);

        let msg_scalar = <Scalar as Reduce<U256>>::from_be_bytes_reduced(final_digest);
        let (signature, recid) = priv_scalar.try_sign_prehashed(**k, msg_scalar)?;
        let recoverable_id = recid.ok_or_else(ecdsa::Error::new)?.try_into()?;
        let rec_sig = recoverable::Signature::new(&signature, recoverable_id)?;

        let id = rec_sig.recovery_id();
        let sig = SecpSignature::from(rec_sig);

        Ok((sig, Some(id.into())))
    }

    fn sign_digest_bytes_deterministic_k(priv_key: &SecretKey, digest_bytes: DigestBytes) -> Result<(SecpSignature, Option<RecoveryId>), BSVErrors> {
        let priv_scalar = priv_key.to_nonzero_scalar();
        let msg_scalar = <Scalar as Reduce<U256>>::from_be_bytes_reduced(digest_bytes);

        let k = rfc6979_generate_k::<_, Sha256r>(&priv_scalar, &msg_scalar, &[]);

        let (signature, recid) = priv_scalar.try_sign_prehashed(**k, msg_scalar)?;
        let recoverable_id = recid.ok_or_else(ecdsa::Error::new)?.try_into()?;
        let rec_sig = recoverable::Signature::new(&signature, recoverable_id)?;

        let id = rec_sig.recovery_id();
        let sig = SecpSignature::from(rec_sig);

        Ok((sig, Some(id.into())))
    }

    fn sign_preimage_random_k(priv_key: &SecretKey, digest: &[u8], reverse_endian_k: bool, hash_algo: SigningHash) -> Result<(SecpSignature, Option<RecoveryId>), ecdsa::Error> {
        let mut added_entropy = FieldBytes::default();
        let rng = &mut OsRng;
        rng.fill_bytes(&mut added_entropy);

        let priv_scalar = priv_key.to_nonzero_scalar();
        let k_digest = match reverse_endian_k {
            true => {
                let mut reversed_digest = digest.to_vec();
                reversed_digest.reverse();

                // TODO: Does this need to be from_be_slice ?
                let scalar_uint = U256::from_le_slice(&reversed_digest);
                Scalar::from_uint_reduced(scalar_uint)
            }
            false => Scalar::from_uint_reduced(U256::from_le_slice(digest)),
        };

        let k = match hash_algo {
            SigningHash::Sha256 => **rfc6979_generate_k::<_, Sha256>(&priv_scalar, &k_digest, &added_entropy),
            SigningHash::Sha256d => **rfc6979_generate_k::<_, Sha256d>(&priv_scalar, &k_digest, &added_entropy),
        };

        let msg_scalar = Scalar::from_uint_reduced(U256::from_le_slice(digest));
        priv_scalar.try_sign_prehashed(k, msg_scalar)
    }

    /**
     * Signs a message digest with a specific private and ephemeral key. I hope you know what you're doing!
     */
    pub(crate) fn sign_with_k_impl(private_key: &PrivateKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<Signature, BSVErrors> {
        let priv_scalar = *private_key.secret_key.to_nonzero_scalar();
        let k = *ephemeral_key.secret_key.to_nonzero_scalar();
        let digest = get_hash_digest(hash_algo, preimage);
        let digest_finalised = digest.finalize_fixed();
        let msg_scalar = <Scalar as Reduce<U256>>::from_be_bytes_reduced(digest_finalised);
        let (signature, recid) = priv_scalar.try_sign_prehashed(k, msg_scalar)?;
        let recoverable_id = recid.ok_or_else(ecdsa::Error::new)?.try_into()?;
        let rec_sig = recoverable::Signature::new(&signature, recoverable_id)?;
        let recovery: Option<RecoveryId> = Some(rec_sig.recovery_id().into());
        let sig = SecpSignature::from(rec_sig);
        Ok(Signature {
            sig,
            recovery: recovery.map(|x| RecoveryInfo::new(x.is_y_odd(), x.is_x_reduced(), private_key.is_pub_key_compressed)),
        })
    }

    /**
     * Signs the preimage hash digest directly. I hope you know what you're doing!
     */
    pub(crate) fn sign_digest_with_deterministic_k_impl(private_key: &PrivateKey, digest: &DigestBytes) -> Result<Signature, BSVErrors> {
        let (sig, recovery) = ECDSA::sign_digest_bytes_deterministic_k(&private_key.secret_key, *digest)?;

        Ok(Signature {
            sig,
            recovery: recovery.map(|x| RecoveryInfo::new(x.is_y_odd(), x.is_x_reduced(), private_key.is_pub_key_compressed)),
        })
    }

    /**
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_deterministic_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        let digest = get_hash_digest(hash_algo, preimage);

        let (sig, recovery) = ECDSA::sign_preimage_deterministic_k(&private_key.secret_key, digest, reverse_k)?;

        Ok(Signature {
            sig,
            recovery: recovery.map(|x| RecoveryInfo::new(x.is_y_odd(), x.is_x_reduced(), private_key.is_pub_key_compressed)),
        })
    }

    /**
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_random_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        let digest = get_hash_digest(hash_algo, preimage);

        let (sig, recovery) = ECDSA::sign_preimage_random_k(&private_key.secret_key, digest.finalize().as_slice(), reverse_k, hash_algo)?;

        Ok(Signature {
            sig,
            recovery: recovery.map(|x| RecoveryInfo::new(x.is_y_odd(), x.is_x_reduced(), private_key.is_pub_key_compressed)),
        })
    }
}

impl ECDSA {
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k)
    }

    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k)
    }

    pub fn sign_with_k(private_key: &PrivateKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_k_impl(private_key, ephemeral_key, preimage, hash_algo)
    }

    pub fn sign_digest_with_deterministic_k(private_key: &PrivateKey, digest: &[u8]) -> Result<Signature, BSVErrors> {
        ECDSA::sign_digest_with_deterministic_k_impl(private_key, GenericArray::from_slice(digest))
    }
}
