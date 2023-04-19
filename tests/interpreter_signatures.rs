#[cfg(test)]
mod interpreter_signature_tests {
    use bsv::Interpreter;
    use bsv::{Hash, PrivateKey, Script, SigHash, Transaction, TxIn};

    #[test]
    fn simple_p2pkh_signature_test() {
        let private_key = PrivateKey::from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh").unwrap();
        let pubkey = private_key.to_public_key().unwrap();
        let mut tx = Transaction::new(2, 0);

        let locking_script = Script::from_asm_string(&format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", Hash::hash_160(&pubkey.to_bytes().unwrap()).to_hex())).unwrap();

        let mut txin = TxIn::default();
        txin.set_satoshis(0);
        txin.set_locking_script(&locking_script);
        tx.add_input(&txin);

        let signature = tx.sign(&private_key, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let script = Script::from_asm_string(&format!(
            "
            {} {}",
            signature.to_hex().unwrap(),
            pubkey.to_hex().unwrap()
        ))
        .unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        println!("Loaded P2PKH script {:?}", interpreter.script_bits());
        interpreter.run().unwrap();

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn checksig_with_codeseparator_test() {
        let private_key = PrivateKey::from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh").unwrap();
        let pubkey = private_key.to_public_key().unwrap();
        let mut tx = Transaction::new(2, 0);

        let final_locking_script = Script::from_asm_string("OP_CODESEPARATOR OP_CHECKSIGVERIFY OP_CODESEPARATOR OP_CHECKSIGVERIFY OP_CODESEPARATOR OP_CHECKSIG").unwrap();
        let third_locking_script = Script::from_asm_string("OP_CHECKSIGVERIFY OP_CODESEPARATOR OP_CHECKSIGVERIFY OP_CODESEPARATOR OP_CHECKSIG").unwrap();
        let second_locking_script = Script::from_asm_string("OP_CHECKSIGVERIFY OP_CODESEPARATOR OP_CHECKSIG").unwrap();
        let first_locking_script = Script::from_asm_string("OP_CHECKSIG").unwrap();

        let mut txin = TxIn::default();
        txin.set_satoshis(0);

        txin.set_locking_script(&final_locking_script);
        tx.add_input(&txin);

        let sig1 = tx.sign(&private_key, SigHash::InputsOutputs, 0, &first_locking_script, 0).unwrap();
        let sig2 = tx.sign(&private_key, SigHash::InputsOutputs, 0, &second_locking_script, 0).unwrap();
        let sig3 = tx.sign(&private_key, SigHash::InputsOutputs, 0, &third_locking_script, 0).unwrap();

        let script = Script::from_asm_string(&format!(
            "
            {} {} {} {} {} {}",
            sig1.to_hex().unwrap(),
            pubkey.to_hex().unwrap(),
            sig2.to_hex().unwrap(),
            pubkey.to_hex().unwrap(),
            sig3.to_hex().unwrap(),
            pubkey.to_hex().unwrap(),
        ))
        .unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        interpreter.run().unwrap();

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    /**
     * OP_1 SIG_X SIG_Y SIG_Z OP_3 PUBKEY_X PUBKEY_Y PUBKEY_Z OP_3 OP_CHECKMULTISIG
     */
    fn multisig_equal_amount_pubkeys_test() {
        let mut tx = Transaction::new(2, 0);
        let pk1 = PrivateKey::from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh").unwrap();
        let pk2 = PrivateKey::from_wif("Kz859spUJBWUBTYqesPMbW1kmFZ7BisBSJckSVYthvvFZ8cRnaPd").unwrap();
        let pk3 = PrivateKey::from_wif("KxQZuMUEecRFubLb52hmfzK4q1Mq4Wi2FfaEs7ZXHkF2cuJqjK16").unwrap();

        let locking_script = Script::from_asm_string(&format!(
            "{} {} {} OP_3 OP_CHECKMULTISIG",
            &pk1.to_public_key().unwrap().to_hex().unwrap(),
            &pk2.to_public_key().unwrap().to_hex().unwrap(),
            &pk3.to_public_key().unwrap().to_hex().unwrap()
        ))
        .unwrap();

        let mut txin = TxIn::default();
        txin.set_satoshis(0);
        txin.set_locking_script(&locking_script);
        tx.add_input(&txin);

        let sig1 = tx.sign(&pk1, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let sig2 = tx.sign(&pk2, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let sig3 = tx.sign(&pk3, SigHash::ALL, 0, &locking_script, 0).unwrap();

        let script = Script::from_asm_string(&format!("OP_1 {} {} {} OP_3", sig1.to_hex().unwrap(), sig2.to_hex().unwrap(), sig3.to_hex().unwrap())).unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        println!("Loaded MULTISIG script {:?}", interpreter.script_bits());
        interpreter.run().unwrap();

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn multisig_not_enough_pubkeys_test() {
        let mut tx = Transaction::new(2, 0);
        let pk1 = PrivateKey::from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh").unwrap();
        let pk2 = PrivateKey::from_wif("Kz859spUJBWUBTYqesPMbW1kmFZ7BisBSJckSVYthvvFZ8cRnaPd").unwrap();
        let pk3 = PrivateKey::from_wif("KxQZuMUEecRFubLb52hmfzK4q1Mq4Wi2FfaEs7ZXHkF2cuJqjK16").unwrap();

        let locking_script = Script::from_asm_string(&format!(
            "{} {} OP_2 OP_CHECKMULTISIG",
            &pk1.to_public_key().unwrap().to_hex().unwrap(),
            &pk2.to_public_key().unwrap().to_hex().unwrap(),
        ))
        .unwrap();

        let mut txin = TxIn::default();
        txin.set_satoshis(0);
        txin.set_locking_script(&locking_script);
        tx.add_input(&txin);

        let sig1 = tx.sign(&pk1, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let sig2 = tx.sign(&pk2, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let sig3 = tx.sign(&pk3, SigHash::ALL, 0, &locking_script, 0).unwrap();

        let script = Script::from_asm_string(&format!("OP_1 {} {} {} OP_3", sig1.to_hex().unwrap(), sig2.to_hex().unwrap(), sig3.to_hex().unwrap())).unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        println!("Loaded MULTISIG script {:?}", interpreter.script_bits());
        assert!(interpreter.run().is_err());
    }

    #[test]
    fn multisig_extra_pubkeys_test() {
        let mut tx = Transaction::new(2, 0);
        let pk1 = PrivateKey::from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh").unwrap();
        let pk2 = PrivateKey::from_wif("Kz859spUJBWUBTYqesPMbW1kmFZ7BisBSJckSVYthvvFZ8cRnaPd").unwrap();
        let pk3 = PrivateKey::from_wif("KxQZuMUEecRFubLb52hmfzK4q1Mq4Wi2FfaEs7ZXHkF2cuJqjK16").unwrap();

        let locking_script = Script::from_asm_string(&format!(
            "{} {} {} OP_3 OP_CHECKMULTISIG",
            &pk1.to_public_key().unwrap().to_hex().unwrap(),
            &pk2.to_public_key().unwrap().to_hex().unwrap(),
            &pk3.to_public_key().unwrap().to_hex().unwrap()
        ))
        .unwrap();

        let mut txin = TxIn::default();
        txin.set_satoshis(0);
        txin.set_locking_script(&locking_script);
        tx.add_input(&txin);

        let sig1 = tx.sign(&pk1, SigHash::ALL, 0, &locking_script, 0).unwrap();
        let sig2 = tx.sign(&pk2, SigHash::ALL, 0, &locking_script, 0).unwrap();

        let script = Script::from_asm_string(&format!("OP_1 {} {} OP_2", sig1.to_hex().unwrap(), sig2.to_hex().unwrap(),)).unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        println!("Loaded MULTISIG script {:?}", interpreter.script_bits());
        interpreter.run().unwrap();

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }
}
