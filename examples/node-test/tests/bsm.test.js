import { Bsm, PrivKey, PubKey, KeyPair, Address } from 'bsv'
import { PrivateKey, PublicKey, P2PKHAddress, SigningHash, BSM, Signature  } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';

import { assert, util } from 'chai';

describe("Bitcoin Signed Messages", function() {
      this.timeout(30000);

      it('signed message matches BSV.JS', () => {
        for (let index = 0; index < 100; index++) {
          let wif = PrivateKey.from_random().to_wif();

          const priv_wasm = PrivateKey.from_wif(wif);
          const pub_wasm = PublicKey.from_private_key(priv_wasm)
          const address_wasm = P2PKHAddress.from_pubkey(pub_wasm)

          const priv_js = PrivKey.fromWif(wif);
          const pub_js = new PubKey().fromPrivKey(priv_js);
          const address_js = new Address().fromPubKey(pub_js);


          const message = Buffer.from(`Hello, Bitcoin - ${Date.now().toString()}`, 'utf8');

          const signature_js = Bsm.sign(message, new KeyPair().fromPrivKey(priv_js))
          assert.equal(Bsm.verify(message, signature_js, address_js), true);

          // const buf = Buffer.from('18426974636f696e205369676e6564204d6573736167653a0a0e48656c6c6f2c20426974636f696e', 'hex') // calculated by logging the buf inside Bsv.magicHash(message) before sha256d
          
          let signature_wasm = BSM.sign_message(priv_wasm, message);

          const signature_wasm_b64 = Buffer.from(signature_wasm.to_compact_bytes()).toString('base64')
          const verification_wasm = Bsm.verify(message, signature_wasm_b64, address_js)
          assert.equal(verification_wasm, true);


          const wasm_reconstruct_sig = Signature.from_compact_bytes(Buffer.from(signature_wasm.to_compact_bytes(), 'base64'));
        
          assert.equal(address_wasm.verify_bitcoin_message(message, wasm_reconstruct_sig), true);
          assert.equal(BSM.verify_message(message, wasm_reconstruct_sig, address_wasm), true);
          assert.equal(BSM.is_valid_message(message, wasm_reconstruct_sig, address_wasm), true);
          
          assert.equal(address_js.toString(), address_wasm.to_string());

          assert.equal(signature_wasm_b64, signature_js.toString(), "JS and WASM signatures did not match")
          
        }
      });
});
