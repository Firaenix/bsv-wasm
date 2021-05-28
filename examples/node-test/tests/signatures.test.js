import {PrivateKey, Hash as WASMHash} from '../../../pkg/bsv_wasm';
import { assert, util } from 'chai';
import {PrivKey, Ecdsa, KeyPair, Hash } from "bsv";

describe("Signature Tests", function() {
    it('Raw Signature Matches Signature from BSV.JS', () => {
        let wif = "L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC"
        let jswifPrivateKey = PrivKey.fromWif(wif);
        let jsKeyPair = KeyPair.fromPrivKey(jswifPrivateKey);

        let signatureMessage = Buffer.from("Hello, Bitcoin.");

        let signature = Ecdsa.sign(Hash.sha256(signatureMessage), jsKeyPair);

        assert.equal("3044022032c4ac1fe69db038e55e5848ddf99865167a4f5172d5acf910c7ac5d66729cb8022049821966a892afc494777f6445d4757e5662b60d9acb1b2c810a5001892774ac", signature.toHex());

        let wasmPrivKey = PrivateKey.fromWIF(wif);
        let wasmSignature = wasmPrivKey.sign(signatureMessage)

        wasmSignature.toDER()

        assert.equal("3044022032c4ac1fe69db038e55e5848ddf99865167a4f5172d5acf910c7ac5d66729cb8022049821966a892afc494777f6445d4757e5662b60d9acb1b2c810a5001892774ac", wasmSignature.toHex())
      });
});