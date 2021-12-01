#[cfg(test)]
mod script_template_tests {
    extern crate wasm_bindgen_test;
    use bsv_wasm::{OpCodes, Script, ScriptTemplateErrors};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    // #[test]
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    // fn to_hex_string() {
    //   let script_bytes = hex::decode("006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d").unwrap();
    //   let script = Script::from_bytes(script_bytes);

    //   assert_eq!(script.to, "006a2231394878696756345179427633744870515663554551797131707a5a56646f4175744c8d2453485541207768616c65206170706561726564efbc81efbc810a4d61796265205348554120736861726befbc81f09f98aff09f98af0a0a68747470733a2f2f7477657463682e6170702f742f643435653233303233383762616235643138663534666566643736633461353462356466636338316461396436623133643832303335333838313064633565630a746578742f706c61696e04746578741f7477657463685f7477746578745f313631393336303532303332352e747874017c223150755161374b36324d694b43747373534c4b79316b683536575755374d74555235035345540b7477646174615f6a736f6e046e756c6c0375726c046e756c6c07636f6d6d656e74046e756c6c076d625f75736572046e756c6c057265706c79046e756c6c047479706504706f73740974696d657374616d70046e756c6c036170700674776574636807696e766f6963652438613262346330622d636531362d346166342d393932312d386638393334343436383938017c22313550636948473232534e4c514a584d6f53556157566937575371633768436676610d424954434f494e5f45434453412231513533564e7853316e647a56444431623834316545795a4458574c6a42735167694c5848384f4c685141527132365371784d56494852554449624b4e45686d49424e6e6e362f376a70786466685731434b61387a63356c3043497471425177557a47734b4b4f2b6e4e4c33444c45424c2f7467433657666f46413d");
    // }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn exact_script_template_matches_script() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        assert!(script.verify_with_template(&script_template).is_ok());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn empty_scripts_match() {
        let script = Script::default();

        let script_template = Script::default();

        assert!(script.verify_with_template(&script_template).is_ok());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_pubkeyhash_matches_p2pkh_script_template() {
        let script = Script::from_asm_string("OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = Script::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert!(script.verify_with_template(&script_template).is_ok());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_data_script_template_matches_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = Script::from_asm_string("OP_DATA 32 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        script.verify_with_template(&script_template).unwrap();

        assert!(true);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_data_script_template_matches_hash_puzzle() {
        use OpCodes::*;

        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = Script::from_asm_string("OP_DATA 32 OP_DATA 2 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        script.verify_with_template(&script_template).unwrap();

        assert!(true);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn p2pkh_script_template_doesnt_match_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = Script::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert!(script.verify_with_template(&script_template).is_err());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn pubkey_script_template_matches_compressed_p2pk() {
        let script = Script::from_asm_string("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG").unwrap();

        let script_template = Script::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        script.verify_with_template(&script_template).unwrap();
        assert!(true);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn pubkey_script_template_matches_uncompressed_p2pk() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG").unwrap();

        let script_template = Script::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        script.verify_with_template(&script_template).unwrap();
        assert!(true);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn multi_pub_key_and_p2pkh_script_template() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG OP_1 OP_DUP 03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = Script::from_asm_string("OP_PUBKEY OP_CHECKSIG OP_1 OP_DUP OP_PUBKEY OP_CHECKSIG OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        script.verify_with_template(&script_template).unwrap();
        assert!(true);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fully_formed_p2pkh_matches_with_script_template() {
        let script = Script::from_asm_string("47304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c554121 0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d OP_DUP OP_HASH160 08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = Script::from_asm_string("OP_SIG OP_PUBKEY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        script.verify_with_template(&script_template).unwrap();
        assert!(true);
    }
}
