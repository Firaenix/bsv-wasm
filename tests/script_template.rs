#[cfg(test)]
mod script_template_tests {
    use bsv::{MatchDataTypes, Script, ScriptTemplate};

    #[test]
    fn empty_script_does_not_match_template() {
        let script = Script::default();

        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        assert_eq!(script.is_match(&script_template), false);
    }

    #[test]
    fn exact_script_template_matches_script_without_extracting_data() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        println!("Script Test {:?}", script);
        let script_template = ScriptTemplate::from_asm_string(
            "d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        let match_result = script.matches(&script_template);

        println!("Matches? {:?}", match_result);
        assert_eq!(match_result.is_ok(), true);

        let extracted = match_result.unwrap();

        assert!(extracted.is_empty());
    }

    #[test]
    fn exact_script_template_matches_script_without_extracting_data_should_fail() {
        let script =
            Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
                .unwrap();

        println!("Script Test {:?}", script);
        let script_template = ScriptTemplate::from_asm_string(
            "3333333333333333333333333333333333333333333333333333333333333333 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG",
        )
        .unwrap();

        let match_result = script.matches(&script_template);

        println!("Matches? {:?}", match_result);
        assert_eq!(match_result.is_err(), true);
    }

    #[test]
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
    fn op_data_script_template_matches_hash_puzzle() {
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

    //     #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    //     fn p2pkh_script_template_doesnt_match_21e8_puzzle_wasm() {
    //         let script =
    //             Script::from_asm_string("d26f2b12ee0a5923dab7314e533917f2ab5b50da5ce302d3d60941f0ee8000a2 21e8 OP_SIZE OP_4 OP_PICK OP_SHA256 OP_SWAP OP_SPLIT OP_DROP OP_EQUALVERIFY OP_DROP OP_CHECKSIG")
    //                 .unwrap();

    //         let script_template = ScriptTemplate::from_asm_string("OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

    //         assert_eq!(script.is_match(&script_template), false)
    //     }

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
    fn fully_formed_p2pkh_matches_with_script_template() {
        let script = Script::from_asm_string("304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c5541 0319a38fb498ff221b6e1b528b911c62f6ff2ac5023405c637859e4d7ff28f265d OP_DUP OP_HASH160 08ed73ac2a3564dd1a431c61f7c2ce6b64e1fe80 OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let script_template = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG").unwrap();

        let match_result = script.matches(&script_template);
        assert_eq!(match_result.is_ok(), true, "Failed to match script");

        let extracted = match_result.unwrap();
        assert_eq!(extracted.len(), 3);

        match &extracted[0] {
            (MatchDataTypes::Signature, v) => {
                assert_eq!(v.len(), 71, "Signature was not 71 bytes long");
                assert_eq!(
                    v,
                    &hex::decode("304402206173a490a5e62036e64f77f8c98db6c57f162a68147cb276bc61da589a114e27022053c19c60dbe7a97ce609631071ee5293c6e6bf4b859094c25a3385490f772c5541").unwrap()
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

    #[test]
    fn matches_sigil_purchase_tx() {
        use bsv::{MatchCriteria, Transaction};

        let tx = Transaction::from_compact_hex("A46776657273696F6E0266696E7075747382A66A707265765F74785F696478406661623130323139393064303535653933336163343463323463363436333064626639343532323265323138346335356566393663383866663635323562653164766F757418436A7363726970745F73696783788E3330343430323230356538616231313038336336303235343264616633343831633937373564386462303463666566313735643839616530663261383732663833653461643463343032323037643337663436313637316430373466373362333834393436323164616235363162356533346638383038633962653432386238333032396533303831326332633378423032316338353931633566323034633134363332363066643430343466303261656638643761356461616662373661633662656635313132633263643136396131637840303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030306873657175656E63651AFFFFFFFF70756E6C6F636B696E675F7363726970748A6A4F505F484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366E4F505F455155414C564552494659664F505F4455506A4F505F484153483136307828663964666335613461653532353665353933386332643831393733386637623537653464376234366E4F505F455155414C5645524946596B4F505F434845434B534947694F505F52455455524E826C4F505F5055534844415441317901FC37623232373436393734366336353232336132323534373537323632366632303436366637383230323333363339323232633232363436353733363337323639373037343639366636653232336132323537363532303663363936623635323037343638363532303636366637383232326332323665373536643632363537323232336133363339326332323733363537323639363537333232336133363339326332323639366436313637363532323361323236323361326632663333333736313336363336333339363336363339363133343631363133363632333536363332333136353334333333333331333633393335363636363636363133343636333233303339333633333335333636323339363333363336333336343336333933363336333333393632333336333633333033373635333736353331323232633232363137343734373236393632373537343635373332323361356237623232373437323631363937343566373437393730363532323361323234333666366336663732323232633232373636313663373536353232336132323532363536343232376432633762323237343732363136393734356637343739373036353232336132323434363937353732366536313663363937343739323232633232373636313663373536353232336132323434363137393232376435643764687361746F7368697318DAA46A707265765F74785F696478406565633364383266303162613766363833623863633263313632656238393936373464333836383237353734636162383230656364663464333139333135666264766F7574016A7363726970745F73696782789033303435303232313030633963633537383039663631393565656665366261306161626437666436313165616138333463656165343532646332373130303961633634316262373130623032323033633531383564646436626266613066313233373638356237343035373731306531623834346132633364393163623435393965353538646662386338303231343178423033366433396130373439636137383030623531623339663763613764313037386237666338373035396534376165653431363966313563393936316236353838646873657175656E63651AFFFFFFFF676F75747075747382A26576616C756518DA6E7363726970745F7075625F6B65798A6A4F505F484153483136307828623862636230376636333434623432616230343235306338366136653862373564336664626263366E4F505F455155414C564552494659664F505F4455506A4F505F484153483136307828313461383033366338623364393130613765323464343630363730343864383736313237346235356E4F505F455155414C5645524946596B4F505F434845434B534947694F505F52455455524E826C4F505F5055534844415441317901FC37623232373436393734366336353232336132323534373537323632366632303436366637383230323333363339323232633232363436353733363337323639373037343639366636653232336132323537363532303663363936623635323037343638363532303636366637383232326332323665373536643632363537323232336133363339326332323733363537323639363537333232336133363339326332323639366436313637363532323361323236323361326632663333333736313336363336333339363336363339363133343631363133363632333536363332333136353334333333333331333633393335363636363636363133343636333233303339333633333335333636323339363333363336333336343336333933363336333333393632333336333633333033373635333736353331323232633232363137343734373236393632373537343635373332323361356237623232373437323631363937343566373437393730363532323361323234333666366336663732323232633232373636313663373536353232336132323532363536343232376432633762323237343732363136393734356637343739373036353232336132323434363937353732366536313663363937343739323232633232373636313663373536353232336132323434363137393232376435643764A26576616C75651A000DE1066E7363726970745F7075625F6B657985664F505F4455506A4F505F484153483136307828333934613034363761313739636130303237656530616638623565346530666165666637316462616E4F505F455155414C5645524946596B4F505F434845434B5349476A6E5F6C6F636B74696D6500").unwrap();

        let tmp = ScriptTemplate::from_asm_string("OP_SIG OP_PUBKEY OP_DATA OP_HASH160 OP_DATA OP_EQUALVERIFY OP_DUP OP_HASH160 OP_PUBKEYHASH OP_EQUALVERIFY OP_CHECKSIG OP_RETURN OP_DATA").unwrap();
        let final_match = tx.get_input(0).unwrap().get_finalised_script().unwrap().matches(&tmp).unwrap();

        assert_eq!(final_match.is_empty(), false);
        assert_eq!(tx.get_input(1).unwrap().get_finalised_script().unwrap().matches(&tmp).is_err(), true);

        let criteria = MatchCriteria::new().set_script_template(&tmp);

        let matching_inputs = tx.match_inputs(&criteria);

        assert_eq!(matching_inputs.len(), 1);
    }
}
