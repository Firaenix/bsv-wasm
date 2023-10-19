import { Interpreter, Script } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { assert, util } from 'chai';
import { PrivateKey } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { Transaction } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { Hash } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { TxIn } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { SigHash } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';

describe("Interpreter Tests", function() {
  it('Executes 1 + 1 = 2', () => {
    let interpreter = Interpreter.from_script(Script.from_asm_string("OP_1 OP_1 OP_ADD"));
    interpreter.run();
    let state = interpreter.get_state();
    assert.equal(Number(state.get_stack()[0][0]), 2) 
  });

  /*
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
        let script = Script::from_asm_string(&format!("
            {} {}",
            signature.to_hex().unwrap(), pubkey.to_hex().unwrap() 
        )).unwrap();

        txin.set_unlocking_script(&script);
        tx.set_input(0, &txin);

        let mut interpreter = Interpreter::from_transaction(&tx, 0).unwrap();

        println!("Loaded P2PKH script {:?}", interpreter.script_bits());
        interpreter.run().unwrap();
         

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }
  */
  it('Executes Signature unlock', () => {
    const private_key = PrivateKey.from_wif("L2WAdy8C19GHNtZDSkbsVBJrBaF9XHpPLTgmnc2N5aGyguhJf7zh");
    const pub_key = private_key.to_public_key();
    let tx = new Transaction(2, 0);

    let locking_script = Script.from_asm_string(`OP_DUP OP_HASH160 ${Hash.hash_160(pub_key.to_bytes()).to_hex()} OP_EQUALVERIFY OP_CHECKSIG`)
    let txin = TxIn.empty();
    txin.set_satoshis(0n);
    txin.set_locking_script(locking_script);
    tx.add_input(txin);

    let sig = tx.sign(private_key, SigHash.ALL, 0, locking_script, 0n);
    let script = Script.from_asm_string(`${sig.to_hex()} ${pub_key.to_hex()}`);

    txin.set_unlocking_script(script);
    tx.set_input(0, txin)

    let interpreter = Interpreter.from_transaction(tx, 0);
    interpreter.run();
    let state = interpreter.get_state();
    assert.equal(Number(state.get_stack()[0][0]), 1) 
  });
});
