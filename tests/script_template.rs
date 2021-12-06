#[cfg(test)]
mod script_template_tests {
    extern crate wasm_bindgen_test;
    use bsv_wasm::{Match, MatchDataTypes, OpCodes, Script, ScriptTemplate, ScriptTemplateErrors};
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn empty_script_does_not_match_template() {
        let script = Script::default();

        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        assert_eq!(script.is_match(&script_template), false);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn exact_script_template_matches_script_without_extracting_data() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        let match_result = script.matches(&script_template);

        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();

        assert!(extracted.is_empty());
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_pubkeyhash_matches_p2pkh_script_template() {
        let script = Script::from_asm_string("OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKeyHash, v) => assert_eq!(v, &hex::decode("05186ff0711831d110ca96ddfc47816b5a31900d").unwrap()),
            _ => assert!(false, "Index 0 did not contain a PubKeyHash"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_data_script_template_matches_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DATA=32 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 32, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn op_data_script_template_matches_hash_puzzle() {
        use OpCodes::*;

        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DATA=32 OP_DATA=2 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG").unwrap();
        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 2);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 32, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }

        match &extracted[1] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 2, "Data was not 2 bytes long");
                assert_eq!(v, &hex::decode("21e8").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain Data"),
        }
    }

    #[test]
    fn p2pkh_script_template_doesnt_match_21e8_puzzle() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert!(script.matches(&script_template).is_err());
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn p2pkh_script_template_doesnt_match_21e8_puzzle_wasm() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        assert_eq!(script.is_match(&script_template), false)
    }

    #[test]

    fn pubkey_script_template_matches_compressed_p2pk() {
        let script = Script::from_asm_string("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "Data was not 32 bytes long");
                assert_eq!(v, &hex::decode("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Data"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn pubkey_script_template_matches_uncompressed_p2pk() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 1);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 65, "Data was not 65 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain PubKey"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn multi_pub_key_and_p2pkh_script_template() {
        let script = Script::from_asm_string("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1 OP_CHECKSIG OP_1 OP_DUP 03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296 OP_CHECKSIG OP_DUP OP_HASH160 05186ff0711831d110ca96ddfc47816b5a31900d OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_PUBKEY OP_CHECKSIG OP_1 OP_DUP OP_PUBKEY OP_CHECKSIG OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 65, "Data was not 65 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("04652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde41092967872982dc75cb2d4b5162f17e97cdcf4329e9fa4ef1b3cba155ccdb600d199b1").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain PubKey"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "Data was not 33 bytes long");
                assert_eq!(v, &hex::decode("03652aee2d0a773eccc4bc5f7816bd4c525f408da26422171a22829bfde4109296").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKey"),
        }

        match &extracted[2] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "Data was not 20 bytes long");
                assert_eq!(v, &hex::decode("05186ff0711831d110ca96ddfc47816b5a31900d").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKeyHash"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn fully_formed_p2pkh_matches_with_script_template() {
        let script = Script::from_asm_string("47304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c554121 0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d OP_DUP OP_HASH160 08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::Signature, v) => {
                assert_eq!(v.len(), 73, "Signature was not 73 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("47304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c554121").unwrap()
                )
            }
            _ => assert!(false, "Index 0 did not contain Signature"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKey, v) => {
                assert_eq!(v.len(), 33, "PubKey was not 33 bytes long");
                assert_eq!(v, &hex::decode("0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PublicKey"),
        }

        match &extracted[2] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "PubKeyHash was not 20 bytes long");
                assert_eq!(v, &hex::decode("08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80").unwrap())
            }
            _ => assert!(false, "Index 2 did not contain PublicKeyHash"),
        }
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn matches_nft() {
        let script = Script::from_asm_string("OP_HASH160 b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6 OP_EQUALVERIFY OP_DUP OP_HASH160 f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46 OP_EQUALVERIFY OP_CHECKSIG OP_RETURN 7b227469746c65223a22547572626f20466f78202331222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_HASH160 OP_DATA=20 OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v.len(), 20, "Data was not 20 bytes long");
                assert_eq!(v, &hex::decode("b8bcb07f6344b42ab04250c86a6e8b75d3fdbbc6").unwrap())
            }
            _ => assert!(false, "Index 0 did not contain Signature"),
        }

        match &extracted[1] {
            (MatchDataTypes::PublicKeyHash, v) => {
                assert_eq!(v.len(), 20, "PubKeyhash was not 20 bytes long");
                assert_eq!(v, &hex::decode("f9dfc5a4ae5256e5938c2d819738f7b57e4d7b46").unwrap())
            }
            _ => assert!(false, "Index 1 did not contain PubKeyhash"),
        }

        match &extracted[2] {
            (MatchDataTypes::Data, v) => {
                assert_eq!(v, &hex::decode("7b227469746c65223a22547572626f20466f78202331222c226465736372697074696f6e223a225765206c696b652074686520666f78222c226e756d626572223a312c22736572696573223a36392c22696d616765223a22623a2f2f33376136636339636639613461613662356632316534333331363935666666613466323039363335366239633636336436393636333962336363303765376531222c2261747472696275746573223a5b7b2274726169745f74797065223a22436f6c6f72222c2276616c7565223a224f72616e6765227d2c7b2274726169745f74797065223a22446975726e616c697479222c2276616c7565223a22446179227d5d7d").unwrap())
            }
            _ => assert!(false, "Index 2 did not contain Data"),
        }
    }
}
