#[cfg(test)]
mod tests {
    use bsv_wasm::{address::*, PrivateKey, PublicKey};
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn private_key_to_address_verify() {
        // Arrange
        let pub_key_hash = "3c3fa3d4adcaf8f52d5b1843975e122548269937";
        let pub_key_hash_bytes = hex::decode(pub_key_hash).unwrap();
        let address = P2PKHAddress::from_pubkey_hash(&pub_key_hash_bytes);

        // Act
        let address_string = address.to_address_string().unwrap();

        // Assert
        assert_eq!(address_string, "16VZnHwRhwrExfeHFHGjwrgEMq8VcYPs9r")
    }

    #[test]
    #[wasm_bindgen_test]
    fn private_key_to_address_verify_2() {
        // Arrange
        let pub_key_hash = "47c6ad3495d35e6df17ccb06831cb44dbd570995";
        let pub_key_hash_bytes = hex::decode(pub_key_hash).unwrap();
        let address = P2PKHAddress::from_pubkey_hash(&pub_key_hash_bytes);

        // Act
        let address_string = address.to_address_string().unwrap();

        // Assert
        assert_eq!(address_string, "17YWyuqbL4D8C8chM1zrC3nTKaPnNiqfTE")
    }

    #[test]
    #[wasm_bindgen_test]
    fn pub_key_hash_to_pub_key_hash() {
        // Arrange
        let pub_key_hash = "47c6ad3495d35e6df17ccb06831cb44dbd570995";
        let pub_key_hash_bytes = hex::decode(pub_key_hash).unwrap();
        let address = P2PKHAddress::from_pubkey_hash(&pub_key_hash_bytes);

        // Act
        let decoded_pub_key_hash_bytes = address.to_pubkey_hash();
        let decoded_pub_key_hash_hex = address.to_pubkey_hash_hex();

        // Assert
        assert_eq!(decoded_pub_key_hash_hex, pub_key_hash);
        assert_eq!(decoded_pub_key_hash_bytes, pub_key_hash_bytes);
    }

    #[test]
    #[wasm_bindgen_test]
    fn private_key_to_public_key_to_address() {
        let priv_key = PrivateKey::from_hex("ef235aacf90d9f4aadd8c92e4b2562e1d9eb97f0df9ba3b508258739cb013db2".into()).unwrap();
        let pub_key = PublicKey::from_private_key(&priv_key);

        let pub_key_hex = pub_key.to_hex().unwrap();
        assert_eq!(pub_key_hex, "02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737");

        let address = P2PKHAddress::from_pubkey(&pub_key).unwrap();
        let address_string = address.to_address_string().unwrap();

        assert_eq!(address_string, "1EUXSxuUVy2PC5enGXR1a3yxbEjNWMHuem")
    }

    #[test]
    #[wasm_bindgen_test]
    fn from_address_string_to_address_string() {
        let priv_key = PrivateKey::from_hex("ef235aacf90d9f4aadd8c92e4b2562e1d9eb97f0df9ba3b508258739cb013db2".into()).unwrap();
        let pub_key = PublicKey::from_private_key(&priv_key);

        let pub_key_hex = pub_key.to_hex().unwrap();
        assert_eq!(pub_key_hex, "02b4632d08485ff1df2db55b9dafd23347d1c47a457072a1e87be26896549a8737");

        let address = P2PKHAddress::from_pubkey(&pub_key).unwrap();
        let address_string = address.to_address_string().unwrap();

        assert_eq!(address_string, "1EUXSxuUVy2PC5enGXR1a3yxbEjNWMHuem");

        let decoded_address = P2PKHAddress::from_p2pkh_string("1EUXSxuUVy2PC5enGXR1a3yxbEjNWMHuem").unwrap();

        assert_eq!(decoded_address.to_address_string().unwrap(), address_string);
    }
}
