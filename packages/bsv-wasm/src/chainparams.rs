use bsv::chainparams::ChainParams as BSVChainParams;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ChainParams(pub(crate) BSVChainParams);

impl From<BSVChainParams> for ChainParams {
    fn from(v: BSVChainParams) -> ChainParams {
        ChainParams(v)
    }
}

#[wasm_bindgen]
impl ChainParams {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChainParams {
        ChainParams(BSVChainParams::default())
    }

    pub fn mainnet() -> ChainParams {
        ChainParams(BSVChainParams::default())
    }

    pub fn testnet() -> ChainParams {
        ChainParams(BSVChainParams::testnet())
    }

    pub fn regtest() -> ChainParams {
        ChainParams(BSVChainParams::regtest())
    }

    pub fn stn() -> ChainParams {
        ChainParams(BSVChainParams::stn())
    }
}
