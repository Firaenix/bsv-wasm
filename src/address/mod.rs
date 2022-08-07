use crate::chainparams::ChainParams;
use crate::{BSVErrors, Hash, Script, SighashSignature, BSM};
use crate::{PublicKey, Signature};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct P2PKHAddress(u8, [u8; 20], [u8; 4]);

impl Serialize for P2PKHAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let addr = self.to_string_impl().map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        serializer.serialize_str(&addr)
    }
}

impl<'de> Deserialize<'de> for P2PKHAddress {
    fn deserialize<D>(deserializer: D) -> Result<P2PKHAddress, D::Error>
    where
        D: Deserializer<'de>,
    {
        let address_string = String::deserialize(deserializer)?;

        let p2pkh = P2PKHAddress::from_string_impl(&address_string).map_err(|e| serde::de::Error::custom(e.to_string()))?;
        Ok(p2pkh)
    }
}

impl P2PKHAddress {
    pub(crate) fn from_pubkey_hash_impl(hash_bytes: &[u8]) -> Result<P2PKHAddress, BSVErrors> {
        let mut addr = vec![0x00];
        addr.extend_from_slice(hash_bytes);

        let shad_bytes = Hash::sha_256d(&addr).to_bytes();
        let checksum_bytes = &shad_bytes[0..4];

        Ok(P2PKHAddress(
            0x00,
            hash_bytes.try_into().map_err(BSVErrors::P2PKHAddressFromSlice)?,
            checksum_bytes.try_into().map_err(BSVErrors::P2PKHAddressFromSlice)?,
        ))
    }

    pub(crate) fn from_pubkey_impl(pub_key: &PublicKey) -> Result<P2PKHAddress, BSVErrors> {
        let pub_key_bytes = pub_key.to_bytes_impl()?;
        let pub_key_hash = Hash::hash_160(&pub_key_bytes);

        P2PKHAddress::from_pubkey_hash_impl(&pub_key_hash.to_bytes())
    }

    pub fn set_chain_params_impl(&self, chain: &ChainParams) -> Result<P2PKHAddress, BSVErrors> {
        let mut addr_bytes = vec![chain.p2pkh];
        addr_bytes.extend_from_slice(&self.1);
        let checksum = Hash::sha_256d(&addr_bytes).to_bytes()[0..4].to_vec();
        let checksum_bytes = [checksum[0], checksum[1], checksum[2], checksum[3]];
        Ok(P2PKHAddress(chain.p2pkh, self.1, checksum_bytes))
    }

    pub(crate) fn to_string_impl(&self) -> Result<String, BSVErrors> {
        let mut pub_key_hash_bytes = self.1.to_vec();

        let mut address_bytes: Vec<u8> = vec![self.0];
        address_bytes.append(&mut pub_key_hash_bytes);

        let shad_bytes = Hash::sha_256d(&address_bytes).to_bytes();
        let mut checksum_bytes = shad_bytes[0..4].to_vec();

        address_bytes.append(&mut checksum_bytes);

        let address = bs58::encode(address_bytes);
        let addr_string = address.into_string();
        Ok(addr_string)
    }

    pub(crate) fn from_string_impl(address_string: &str) -> Result<P2PKHAddress, BSVErrors> {
        if address_string.len() < 33 {
            return Err(BSVErrors::P2PKHAddress("Too Short! invalid address"));
        }

        let decoded = bs58::decode(address_string);
        let decoded_bytes = decoded.into_vec()?;

        let address_bytes: Vec<u8> = decoded_bytes[..decoded_bytes.len() - 4].to_vec();
        let address_checksum = decoded_bytes[decoded_bytes.len() - 4..].to_vec();

        let shad_bytes = Hash::sha_256d(&address_bytes).to_bytes();
        let checksum_bytes = shad_bytes[0..4].to_vec();

        if checksum_bytes != address_checksum {
            return Err(BSVErrors::P2PKHAddress("Checksum failed! invalid address."));
        }

        // // Remove 0x00 from the front and the 4 byte checksum off the end
        let chain_byte = decoded_bytes[0];
        let pub_key_hash = decoded_bytes[1..decoded_bytes.len() - 4].try_into().map_err(BSVErrors::P2PKHAddressFromSlice)?;
        let checksum = decoded_bytes[decoded_bytes.len() - 4..].try_into().map_err(BSVErrors::P2PKHAddressFromSlice)?;

        Ok(P2PKHAddress(chain_byte, pub_key_hash, checksum))
    }

    /**
     * Produces the locking script for a P2PKH address.
     * Should be inserted into a new TxOut.
     */
    pub(crate) fn to_locking_script_impl(&self) -> Result<Script, BSVErrors> {
        Script::from_asm_string(&format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", self.to_pubkey_hash_hex()))
    }

    /**
     * Produces the unlocking script for a P2PKH address.
     * Should be inserted into a TxIn.
     */
    pub(crate) fn to_unlocking_script_impl(&self, pub_key: &PublicKey, sig: &SighashSignature) -> Result<Script, BSVErrors> {
        // Make sure the given Public Key matches this address.
        let verifying_address = P2PKHAddress::from_pubkey_impl(pub_key)?;

        if verifying_address != *self {
            return Err(BSVErrors::GenerateScript("Given public key does not correspond to this address".into()));
        }

        let pub_key_hex = pub_key.to_hex_impl()?;
        let script = Script::from_asm_string(&format!("{} {}", sig.to_hex_impl()?, pub_key_hex))?;

        Ok(script)
    }
}

/**
  Shared Methods
*/
impl P2PKHAddress {
    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = toPubKeyHashBytes))]
    pub fn to_pubkey_hash(&self) -> Vec<u8> {
        self.1.to_vec()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = toPubKeyHashHex))]
    pub fn to_pubkey_hash_hex(&self) -> String {
        hex::encode(self.1)
    }

    /**
     * Check if message is signed by this Address.
     *
     * Returns a boolean
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = isValidBitcoinMessage))]
    pub fn is_valid_bitcoin_message(&self, message: &[u8], signature: &Signature) -> bool {
        BSM::verify_message_impl(message, signature, self).is_ok()
    }
}

/**
 * WASM Exported Methods
 */
// #[cfg(all(feature = "wasm-bindgen-transaction"))]
// #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen)]
// impl P2PKHAddress {
//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = fromPubKeyHash))]
//     pub fn from_pubkey_hash(hash_bytes: &[u8]) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress::from_pubkey_hash_impl(hash_bytes)?)
//     }
//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = fromPubKey))]
//     pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress::from_pubkey_impl(pub_key)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = setChainParams))]
//     pub fn set_chain_params(&self, chain_params: &ChainParams) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress::set_chain_params_impl(&self, chain_params)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = toString))]
//     pub fn to_address_string(&self) -> Result<String, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress::to_address_string_impl(&self)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = fromString))]
//     pub fn from_string(address_string: &str) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress::from_string_impl(address_string)?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = toLockingScript))]
//     pub fn get_locking_script(&self) -> Result<Script, wasm_bindgen::JsError> {
//         Ok(self.to_locking_script_impl()?)
//     }

//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = toUnlockingScript))]
//     pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &SighashSignature) -> Result<Script, wasm_bindgen::JsError> {
//         Ok(self.to_unlocking_script_impl(pub_key, sig)?)
//     }

//     /**
//      * Verify if message is signed by this Address.
//      *
//      * Throws an error if invalid.
//      */
//     #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = verifyBitcoinMessage))]
//     pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
//         Ok(BSM::verify_message_impl(message, signature, self)?)
//     }
// }

/**
 * Native Exported Methods
 */
impl P2PKHAddress {
    pub fn from_pubkey_hash(hash_bytes: &[u8]) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_pubkey_hash_impl(hash_bytes)
    }

    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_pubkey_impl(pub_key)
    }

    pub fn to_string(&self) -> Result<String, BSVErrors> {
        P2PKHAddress::to_string_impl(self)
    }

    pub fn set_chain_params(&self, chain_params: &ChainParams) -> Result<P2PKHAddress, BSVErrors> {
        self.set_chain_params_impl(chain_params)
    }

    pub fn from_string(address_string: &str) -> Result<P2PKHAddress, BSVErrors> {
        P2PKHAddress::from_string_impl(address_string)
    }

    pub fn get_locking_script(&self) -> Result<Script, BSVErrors> {
        self.to_locking_script_impl()
    }

    pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &SighashSignature) -> Result<Script, BSVErrors> {
        self.to_unlocking_script_impl(pub_key, sig)
    }

    /**
     * Verify if message is signed by this Address.
     *
     * Returns a Result
     */
    pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, BSVErrors> {
        BSM::verify_message_impl(message, signature, self)
    }
}
