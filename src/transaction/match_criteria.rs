use crate::{ScriptTemplate, Transaction, TxIn, TxOut};

#[derive(Debug, Clone, Default)]
pub struct MatchCriteria {
    pub(crate) script_template: Option<ScriptTemplate>,
    pub(crate) exact_value: Option<u64>,
    pub(crate) min_value: Option<u64>,
    pub(crate) max_value: Option<u64>,
}

impl MatchCriteria {
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(constructor))]
    pub fn new() -> MatchCriteria {
        MatchCriteria::default()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setScriptTemplate))]
    pub fn set_script_template(&mut self, script_template: &ScriptTemplate) -> MatchCriteria {
        self.script_template = Some(script_template.clone());

        self.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setValue))]
    pub fn set_value(&mut self, value: u64) -> MatchCriteria {
        self.exact_value = Some(value);

        self.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setMin))]
    pub fn set_min(&mut self, min: u64) -> MatchCriteria {
        self.min_value = Some(min);

        self.clone()
    }

    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = setMax))]
    pub fn set_max(&mut self, max: u64) -> MatchCriteria {
        self.max_value = Some(max);

        self.clone()
    }
}

impl Transaction {
    fn is_matching_output(txout: &TxOut, criteria: &MatchCriteria) -> bool {
        // If script is specified and doesnt match
        if matches!(&criteria.script_template, Some(crit_script) if !txout.script_pub_key.is_match(crit_script)) {
            return false;
        }

        // If exact_value is specified and doesnt match
        if criteria.exact_value.is_some() && criteria.exact_value != Some(txout.value) {
            return false;
        }

        // If min_value is specified and value is less than min value
        if criteria.min_value.is_some() && criteria.min_value > Some(txout.value) {
            return false;
        }

        // If min_value is specified and value is greater than max value
        if criteria.max_value.is_some() && criteria.max_value < Some(txout.value) {
            return false;
        }

        true
    }

    /**
     * Returns the first output index that matches the given parameters, returns None or null if not found.
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = matchOutput))]
    pub fn match_output(&self, criteria: &MatchCriteria) -> Option<usize> {
        self.outputs.iter().enumerate().find_map(|(i, txout)| match Transaction::is_matching_output(txout, criteria) {
            true => Some(i),
            false => None,
        })
    }

    /**
     * Returns a list of outputs indexes that match the given parameters
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = matchOutputs))]
    pub fn match_outputs(&self, criteria: &MatchCriteria) -> Vec<usize> {
        let matches = self
            .outputs
            .iter()
            .enumerate()
            .filter_map(|(i, txout)| match Transaction::is_matching_output(txout, criteria) {
                true => Some(i),
                false => None,
            })
            .collect();

        matches
    }

    fn is_matching_input(txin: &TxIn, criteria: &MatchCriteria) -> bool {
        // If script is specified and doesnt match
        if matches!(&criteria.script_template, Some(crit_script) if !txin.get_finalised_script_impl().unwrap().is_match(crit_script)) {
            return false;
        }

        // If exact_value is specified and doesnt match
        if criteria.exact_value.is_some() && criteria.exact_value != txin.satoshis {
            return false;
        }

        // If min_value is specified and value is less than min value
        if criteria.min_value.is_some() && criteria.min_value > txin.satoshis {
            return false;
        }

        // If min_value is specified and value is greater than max value
        if criteria.max_value.is_some() && criteria.max_value < txin.satoshis {
            return false;
        }

        true
    }

    /**
     * Returns the first input index that matches the given parameters, returns None or null if not found.
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = matchInput))]
    pub fn match_input(&self, criteria: &MatchCriteria) -> Option<usize> {
        self.inputs.iter().enumerate().find_map(|(i, txin)| match Transaction::is_matching_input(txin, criteria) {
            true => Some(i),
            false => None,
        })
    }

    /**
     * Returns a list of input indexes that match the given parameters
     */
    // #[cfg_attr(all(feature = "wasm-bindgen-transaction"), wasm_bindgen(js_name = matchInputs))]
    pub fn match_inputs(&self, criteria: &MatchCriteria) -> Vec<usize> {
        let matches = self
            .inputs
            .iter()
            .enumerate()
            .filter_map(|(i, txin)| match Transaction::is_matching_input(txin, criteria) {
                true => Some(i),
                false => None,
            })
            .collect();

        matches
    }
}
