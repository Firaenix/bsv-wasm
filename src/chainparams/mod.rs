use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChainParams {
    pub p2pkh: u8,
    pub p2sh: u8,
    pub privkey: u8,
    pub xpub: u32,
    pub xpriv: u32,
    pub magic: u32,
}

impl Default for ChainParams {
    fn default() -> ChainParams {
        ChainParams {
            p2pkh: 0x00,
            p2sh: 0x05,
            privkey: 0x80,
            xpub: 0x0488b21e,
            xpriv: 0x0488ade4,
            magic: 0xe3e1f3e8,
        }
    }
}

impl ChainParams {
    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(constructor))]
    pub fn new(p2pkh: u8, p2sh: u8, privkey: u8, xpub: u32, xpriv: u32, magic: u32) -> ChainParams {
        ChainParams {
            p2pkh,
            p2sh,
            privkey,
            xpub,
            xpriv,
            magic,
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = Mainnet))]
    pub fn mainnet() -> ChainParams {
        ChainParams::default()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = Testnet))]
    pub fn testnet() -> ChainParams {
        ChainParams {
            p2pkh: 0x6f,
            p2sh: 0xc4,
            privkey: 0xef,
            xpub: 0x043587cf,
            xpriv: 0x04358394,
            magic: 0xf4e5f3f4,
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = Regtest))]
    pub fn regtest() -> ChainParams {
        let ChainParams {
            p2pkh, p2sh, privkey, xpub, xpriv, ..
        } = ChainParams::testnet();
        ChainParams {
            p2pkh,
            p2sh,
            privkey,
            xpub,
            xpriv,
            magic: 0xdab5bffa,
        }
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-address"), wasm_bindgen(js_name = STN))]
    pub fn stn() -> ChainParams {
        let ChainParams {
            p2pkh, p2sh, privkey, xpub, xpriv, ..
        } = ChainParams::testnet();
        ChainParams {
            p2pkh,
            p2sh,
            privkey,
            xpub,
            xpriv,
            magic: 0xfbcec4f9,
        }
    }
}
