use wasm_bindgen::prelude::*;

use crate::Script;

#[wasm_bindgen]
#[derive(Debug, Clone, Default)]
pub struct MatchCriteria {
    pub(crate) script: Option<Script>,
    pub(crate) exact_value: Option<u64>,
    pub(crate) min_value: Option<u64>,
    pub(crate) max_value: Option<u64>,
}

#[wasm_bindgen]
impl MatchCriteria {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MatchCriteria {
        MatchCriteria::default()
    }

    #[wasm_bindgen(js_name = setScript)]
    pub fn set_script(&mut self, script: &Script) -> MatchCriteria {
        self.script = Some(script.clone());

        self.clone()
    }

    #[wasm_bindgen(js_name = setValue)]
    pub fn set_value(&mut self, value: u64) -> MatchCriteria {
        self.exact_value = Some(value);

        self.clone()
    }

    #[wasm_bindgen(js_name = setMin)]
    pub fn set_min(&mut self, min: u64) -> MatchCriteria {
        self.min_value = Some(min);

        self.clone()
    }

    #[wasm_bindgen(js_name = setMax)]
    pub fn set_max(&mut self, max: u64) -> MatchCriteria {
        self.max_value = Some(max);

        self.clone()
    }
}
