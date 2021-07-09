#[cfg(test)]
mod tests {
    use bsv_wasm::{address::*, PrivateKey, PublicKey};
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn pub_key_from_private_key() {
        // Arrange
        let private_key = PrivateKey::from_hex(String::from("E9873D79C6D87DC0FB6A5778633389F4453213303DA61F20BD67FC233AA33262")).unwrap();

        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_private_key(&private_key, Some(true));
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_private_key(&private_key, true);
        let pub_key_hex = pub_key.to_hex().unwrap();

        // Assert
        assert_eq!(pub_key_hex, "02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9")
    }

    #[test]
    #[wasm_bindgen_test]
    fn pub_key_from_hex() {
        // Arrange
        #[cfg(target_arch = "wasm32")]
        let pub_key = PublicKey::from_hex(String::from("02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9"), Some(true)).unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        let pub_key = PublicKey::from_hex(String::from("02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9"), true).unwrap();

        let pub_key_hex = pub_key.to_hex().unwrap();

        // Assert
        assert_eq!(pub_key_hex, "02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9")
    }
}
