use crate::get_hash_digest;
use crate::hash::sha256d_digest::Sha256d;
use crate::BSVErrors;
use crate::PrivateKey;
use crate::RecoveryInfo;
use crate::Signature;
use crate::ECDSA;
use crate::{reverse_digest::ReversibleDigest, Sha256r, SigningHash};
use digest::{BlockInput, Digest, FixedOutput, Reset, Update};
use ecdsa::hazmat::{rfc6979_generate_k, SignPrimitive};
use ecdsa::RecoveryId;
use elliptic_curve::ops::Reduce;
use k256::ecdsa::recoverable::*;
use k256::FieldBytes;
use k256::U256;
use k256::{ecdsa::Signature as SecpSignature, Scalar, SecretKey};
use rand_core::OsRng;
use rand_core::RngCore;
use sha2::Sha256;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::throw_str;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

impl ECDSA {
    fn sign_preimage_deterministic_k(priv_key: &SecretKey, digest: &[u8], reverse_endian_k: bool, hash_algo: SigningHash) -> Result<(SecpSignature, Option<RecoveryId>), ecdsa::Error> {
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
            SigningHash::Sha256 => **rfc6979_generate_k::<_, Sha256>(&priv_scalar, &k_digest, &[]),
            SigningHash::Sha256d => **rfc6979_generate_k::<_, Sha256d>(&priv_scalar, &k_digest, &[]),
        };
        let msg_scalar = Scalar::from_uint_reduced(U256::from_le_slice(digest));
        priv_scalar.try_sign_prehashed(k, msg_scalar)
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
     * Hashes the preimage with the specified Hashing algorithm and then signs the specified message.
     * Secp256k1 signature inputs must be 32 bytes in length - SigningAlgo will output a 32 byte buffer.
     * HASH+HMAC can be reversed for K generation if necessary.
     */
    pub(crate) fn sign_with_deterministic_k_impl(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        let digest = get_hash_digest(hash_algo, preimage);

        let (sig, recovery) = ECDSA::sign_preimage_deterministic_k(&private_key.secret_key, digest.finalize().as_slice(), reverse_k, hash_algo)?;

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

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ECDSA {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = signWithRandomK))]
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = sign))]
    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, JsValue> {
        match ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k) {
            Ok(v) => Ok(v),
            Err(e) => throw_str(&e.to_string()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ECDSA {
    pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_random_k_impl(private_key, preimage, hash_algo, reverse_k)
    }

    pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, BSVErrors> {
        ECDSA::sign_with_deterministic_k_impl(private_key, preimage, hash_algo, reverse_k)
    }
}
