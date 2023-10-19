use bsv::chainparams::ChainParams as BSVChainParams;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct ChainParams(pub(crate) BSVChainParams);

impl From<BSVChainParams> for ChainParams {
    fn from(v: BSVChainParams) -> ChainParams {
        ChainParams(v)
    }
}

impl From<ChainParams> for BSVChainParams {
    fn from(v: ChainParams) -> BSVChainParams {
        v.0
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
