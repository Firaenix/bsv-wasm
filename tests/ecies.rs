#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod ecies_tests {
    use std::io::Read;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn encrypt_decrypt_text_ephemeral_private_key() {

    }

    #[test]
    #[wasm_bindgen_test]
    fn encrypt_text_specific_private_key() {

    }
}