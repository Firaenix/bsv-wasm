use crate::{chainparams::ChainParams, keypair::public_key::PublicKey, script::Script, signature::Signature};
use bsv::P2PKHAddress as BSVP2PKHAdress;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct P2PKHAddress(pub(crate) BSVP2PKHAdress);

#[wasm_bindgen]
impl P2PKHAddress {
    pub fn from_pubkey_hash(hash_bytes: &[u8]) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(BSVP2PKHAdress::from_pubkey_hash(hash_bytes)?))
    }

    pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(BSVP2PKHAdress::from_pubkey(&pub_key.0)?))
    }

    pub fn set_chain_params(&self, chain_params: &ChainParams) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(BSVP2PKHAdress::set_chain_params(&self.0, &chain_params.0)?))
    }

    pub fn to_address_string(&self) -> Result<String, wasm_bindgen::JsError> {
        Ok(BSVP2PKHAdress::to_address_string(&self.0)?)
    }

    pub fn from_string(address_string: &str) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
        Ok(P2PKHAddress(BSVP2PKHAdress::from_string(address_string)?))
    }

    pub fn get_locking_script(&self) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(BSVP2PKHAdress::get_locking_script(&self.0)?))
    }

    pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &SighashSignature) -> Result<Script, wasm_bindgen::JsError> {
        Ok(Script(BSVP2PKHAdress::get_unlocking_script(&self.0, &pub_key.0, sig.0)?))
    }

    pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
        Ok(BSVP2PKHAdress::verify_bitcoin_message(&self.0, message, &signature.0)?)
    }
}
