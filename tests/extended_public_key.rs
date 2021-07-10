#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod xpub_tests {
    use bsv_wasm::{hash::Hash, keypair::*};
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[wasm_bindgen_test]
    fn xpriv_to_xpub() {
        let key = ExtendedPrivateKey::from_string("xprv9tuogRdb5YTgcL3P8Waj7REqDuQx4sXcodQaWTtEVFEp6yRKh1CjrWfXChnhgHeLDuXxo2auDZegMiVMGGxwxcrb2PmiGyCngLxvLeGsZRq").unwrap();
        let pub_key = ExtendedPublicKey::from_xpriv(&key);

        assert_eq!(key.to_string().unwrap(), "xprv9tuogRdb5YTgcL3P8Waj7REqDuQx4sXcodQaWTtEVFEp6yRKh1CjrWfXChnhgHeLDuXxo2auDZegMiVMGGxwxcrb2PmiGyCngLxvLeGsZRq");
        assert_eq!(pub_key.to_string().unwrap(), "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu");
    }

    #[test]
    #[wasm_bindgen_test]
    fn xpub_string_to_xpub() {
        let pub_key = ExtendedPublicKey::from_string("xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu").unwrap();

        assert_eq!(pub_key.to_string().unwrap(), "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu");
    }

    #[test]
    #[wasm_bindgen_test]
    fn xpub_derived_key_is_correct() {
        let pub_key = ExtendedPublicKey::from_string("xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu").unwrap();

        assert_eq!(pub_key.derive_from_path("m/0/0/20/40/1/1/0/1").unwrap().to_string().unwrap(), "xpub6Q6M4vyCnxQeutTLtqQQTndsyiLiyF82UEQLG8eDeRBHL73KYmgTMk57cJ1cxNPB9aUgjE8Myh7CAZWJaYewtiGiYKZEpzAj5hGzcqRw8rF");

        let derived_pub_key = pub_key.derive_from_path("m/0").unwrap();

        assert_eq!(derived_pub_key.to_string().unwrap(), "xpub6AsnzNXCyC9QfhSxRt7PjnzYnUTaRzhxefykbgeSip3RN4oTDn3cGsjsJadVGobz4HbjXyeAsf1miBbwoJF4Hae5G3m8bMJ2HF2Afy4HY5W");
        assert_eq!(derived_pub_key.derive_from_path("m/0").unwrap().to_string().unwrap(), "xpub6DWFU3SyVAYmkgSJRRhXPZNXJikfZaBYQrq4L9NuzJc96Fyv3ibRhZqob8nA384KYq2VfdE3HsJSumnvzRxrXXRZPoc2AnuDLCAg6H3mx1t");
    }

    #[test]
    fn xpub_cannot_do_hardened_derivation() {
        let pub_key = ExtendedPublicKey::from_string("xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu").unwrap();

        // Cannot do hardened derivation
        assert_eq!(pub_key.derive_from_path("m/0'/1'").is_err(), true);
    }
}
