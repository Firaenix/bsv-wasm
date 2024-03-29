use crate::get_hash_digest;
use crate::BSVErrors;
use crate::PrivateKey;
use crate::PublicKey;
use crate::Signature;
use crate::SigningHash;
use crate::ECDSA;
use digest::FixedOutput;
use elliptic_curve::bigint::Encoding;
use elliptic_curve::bigint::U1024;

impl ECDSA {
    /**
     * Recovers a Private Key from a signature with a known message digest and K value.
     */

    fn private_key_from_signature_k_impl(signature: &Signature, public_key: &PublicKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<PrivateKey, BSVErrors> {
        let mut k_final = [0u8; 128];
        ephemeral_key
            .secret_key
            .to_nonzero_scalar()
            .to_bytes()
            .iter()
            .copied()
            .enumerate()
            .for_each(|(i, x)| k_final[i + 96] = x);
        let k = U1024::from_be_slice(&k_final);

        let m_final = get_hash_digest(hash_algo, preimage).finalize_fixed().to_vec();

        let m = U1024::from_be_slice(&[
            0x00u8,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            m_final[0],
            m_final[1],
            m_final[2],
            m_final[3],
            m_final[4],
            m_final[5],
            m_final[6],
            m_final[7],
            m_final[8],
            m_final[9],
            m_final[10],
            m_final[11],
            m_final[12],
            m_final[13],
            m_final[14],
            m_final[15],
            m_final[16],
            m_final[17],
            m_final[18],
            m_final[19],
            m_final[20],
            m_final[21],
            m_final[22],
            m_final[23],
            m_final[24],
            m_final[25],
            m_final[26],
            m_final[27],
            m_final[28],
            m_final[29],
            m_final[30],
            m_final[31],
        ]);

        let s_final = signature.sig.s().to_bytes().to_vec();
        let s = U1024::from_be_slice(&[
            0x00u8,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            s_final[0],
            s_final[1],
            s_final[2],
            s_final[3],
            s_final[4],
            s_final[5],
            s_final[6],
            s_final[7],
            s_final[8],
            s_final[9],
            s_final[10],
            s_final[11],
            s_final[12],
            s_final[13],
            s_final[14],
            s_final[15],
            s_final[16],
            s_final[17],
            s_final[18],
            s_final[19],
            s_final[20],
            s_final[21],
            s_final[22],
            s_final[23],
            s_final[24],
            s_final[25],
            s_final[26],
            s_final[27],
            s_final[28],
            s_final[29],
            s_final[30],
            s_final[31],
        ]);

        let inv_r = signature.sig.r().invert();
        if inv_r.is_none().into() {
            return Err(BSVErrors::CustomECDSAError("Invalid modInvR value".to_string()));
        }
        let rinv_final = inv_r.unwrap().to_bytes().to_vec();

        let r_inverse = U1024::from_be_slice(&[
            0x00u8,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            rinv_final[0],
            rinv_final[1],
            rinv_final[2],
            rinv_final[3],
            rinv_final[4],
            rinv_final[5],
            rinv_final[6],
            rinv_final[7],
            rinv_final[8],
            rinv_final[9],
            rinv_final[10],
            rinv_final[11],
            rinv_final[12],
            rinv_final[13],
            rinv_final[14],
            rinv_final[15],
            rinv_final[16],
            rinv_final[17],
            rinv_final[18],
            rinv_final[19],
            rinv_final[20],
            rinv_final[21],
            rinv_final[22],
            rinv_final[23],
            rinv_final[24],
            rinv_final[25],
            rinv_final[26],
            rinv_final[27],
            rinv_final[28],
            rinv_final[29],
            rinv_final[30],
            rinv_final[31],
        ]);

        let n = U1024::from_be_slice(&[
            0x00u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C,
            0xD0, 0x36, 0x41, 0x41,
        ]);

        // The formula to reverse a private key is P = r^-1(N) * ((k * s) -H(m)) % N
        // however as Bitcoin signatures must use a low S value due to Bip 62, we
        // must account for both high and low S values in our equation.
        let mut k_s = r_inverse.wrapping_mul(&k.wrapping_mul(&s).wrapping_sub(&m)).wrapping_rem(&n);
        let mut priv_p = PrivateKey::from_bytes_impl(&k_s.to_be_bytes()[96..])?;
        let mut pub_p = PublicKey::from_private_key_impl(&priv_p).to_bytes_impl()?;
        // Handle edge case for high S
        if pub_p != public_key.to_bytes_impl()? {
            k_s = r_inverse.wrapping_mul(&k.wrapping_mul(&n.wrapping_sub(&s)).wrapping_sub(&m)).wrapping_rem(&n);
            priv_p = PrivateKey::from_bytes_impl(&k_s.to_be_bytes()[96..])?;
            pub_p = PublicKey::from_private_key_impl(&priv_p).to_bytes_impl()?;
        }
        // Handle edge case for extremely low private key
        if pub_p != public_key.to_bytes_impl()? {
            k_s = r_inverse.wrapping_mul(&k.wrapping_mul(&n.wrapping_sub(&s)).wrapping_sub(&m.wrapping_add(&n))).wrapping_rem(&n);
            priv_p = PrivateKey::from_bytes_impl(&k_s.to_be_bytes()[96..])?;
            pub_p = PublicKey::from_private_key_impl(&priv_p).to_bytes_impl()?;
        }
        if pub_p == public_key.to_bytes_impl()? {
            Ok(priv_p)
        } else {
            Err(BSVErrors::CustomECDSAError("Unable to recover private key.".to_string()))
        }
    }
}

impl ECDSA {
    pub fn private_key_from_signature_k(signature: &Signature, public_key: &PublicKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<PrivateKey, BSVErrors> {
        ECDSA::private_key_from_signature_k_impl(signature, public_key, ephemeral_key, preimage, hash_algo)
    }
}
